#[derive(Debug, PartialEq)]
pub enum Command {
    Get(String),
    Set(String, String),
    Del(Vec<String>),
    Unknown(String),
}

pub fn parse(input: &str) -> Command {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    if lines.is_empty() {
        return Command::Unknown(input.to_string());
    }

    let mut args = vec![];
    for line in lines {
        if !line.starts_with('*') && !line.starts_with('$') {
            args.push(line);
        }
    }

    match args.as_slice() {
        ["SET", key, value] => Command::Set(key.to_string(), value.to_string()),
        ["GET", key] => Command::Get(key.to_string()),
        ["DEL", rest @ ..] if !rest.is_empty() => {
            Command::Del(rest.iter().map(|s| s.to_string()).collect())
        }
        [cmd, ..] => Command::Unknown(cmd.to_string()),
        _ => Command::Unknown(input.to_string()),
    }
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
        assert_eq!(result, Command::Unknown("GET".to_string()));
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
        assert_eq!(result, Command::Unknown("SET".to_string()));
    }


    #[test]
    fn test_parse_del() {
        let input = "*3\r\n$3\r\nDEL\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";
        let expected = Command::Del(vec!["foo".to_string(), "bar".to_string()]);
        assert_eq!(parse(input), expected);
    }
}
