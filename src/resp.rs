pub enum Command {
    Get(String),
    Set(String, String),
    Unknown(String),
}

pub fn parse(input: &str) -> Command {
    let parts: Vec<&str> = input.split("\r\n").collect();
    if parts.len() < 4 {
        return Command::Unknown(input.to_string());
    }

    if parts[2].eq_ignore_ascii_case("SET") && parts.len() >= 7 {
        return Command::Set(parts[4].to_string(), parts[6].to_string());
    }

    if parts[2].eq_ignore_ascii_case("GET") && parts.len() >= 5 {
        return Command::Get(parts[4].to_string());
    }

    Command::Unknown(input.to_string())
}
