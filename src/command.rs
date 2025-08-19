#[derive(Debug)]
pub enum Command {
    Insert,
    Delete,
    Update,
    Get,
    Exit,
    Unknown,
}

pub fn parse_command_args(command: &str) -> Vec<&str> {
    command.split_whitespace().skip(1).collect()
}

pub fn parse_command_type(command: &str) -> Command {
    match command {
        "insert" => Command::Insert,
        "delete" => Command::Delete,
        "update" => Command::Update,
        "get" => Command::Get,
        "exit" => Command::Exit,
        _ => Command::Unknown,
    }
}
