use std::{io::Write, net};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;

const CONNECT_ADDRESS: &str = "127.0.0.1:8001";

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
    let mut stream =
        net::TcpStream::connect(CONNECT_ADDRESS).expect("Couldn't establish TCP connection");

    stream.write_all("$> ".as_bytes()).unwrap();
    show_messagebox("C2 Activated!\x00");
}
