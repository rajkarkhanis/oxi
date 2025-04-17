#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get() {
        let input = "*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n";
        let result = parse(input);
        assert_eq!(result, Command::Get("foo".to_string()));
    }

    #[test]
    fn test_parse_invalid_get() {
        let input = "*1\r\n$3\r\nGET\r\n";
        let result = parse(input);
        assert_eq!(result, Command::Unknown(input.to_string()));
    }

    #[test]
    fn test_valid_set() {
        let input = "*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let result = parse(input);
        assert_eq!(result, Command::Set("foo".to_string(), "bar".to_string()));
    }

    #[test]
    fn test_invalid_set_missing_value() {
        let input = "*2\r\n$3\r\nSET\r\n$3\r\nfoo\r\n";
        let result = parse(input);
        assert_eq!(result, Command::Unknown(input.to_string()));
    }
}
