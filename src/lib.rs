use windows::Win32::Foundation::{BOOL, HANDLE};
mod execute;
use execute::execute;
mod cmd;
mod config;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DLLMain(dll_module: HANDLE, call_reason: u32, lpv_reserved: u32) -> BOOL {
    match call_reason {
        _ => {
            execute();
            BOOL(0)
        }
    }
}
