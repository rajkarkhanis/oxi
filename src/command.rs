use crate::store::Store;

pub fn process_command(input: &str, store: &Store) -> String {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return "ERROR: Empty command. Try `SET key value` or `GET key`\r\n".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "SET" => {
            if parts.len() != 3 {
                return "ERROR: Incorrect SET command. Usage: `SET key value`\r\n".to_string();
            }
            let key = parts[1];
            let value = parts[2];
            store.set(key.to_string(), value.to_string());
            "OK\r\n".to_string()
        }

        "GET" => {
            if parts.len() != 2 {
                return "ERROR: Incorrect GET command. Usage: `GET key`\r\n".to_string();
            }
            let key = parts[1];
            match store.get(key) {
                Some(value) => format!("{}: {}\r\n", key, value),
                None => "ERROR: Key not found\r\n".to_string(),
            }
        }

        "HELP" => {
            help_message()
        }

        _ => {
            "ERROR: Unknown command. Try `HELP` for usage.\r\n".to_string()
        }
    }
}

fn help_message() -> String {
    [
        "Available commands:",
        "  SET key value  - Store a value",
        "  GET key        - Retrieve a value",
        "  HELP           - Show this help message",
        ""
    ].join("\r\n")
}
