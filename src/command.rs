use crate::store::Store;
use crate::resp::{self, Command};

pub fn process_command(input: &str, store: &Store) -> String {
    let command = resp::parse(input);

    match command {
        Command::Set(key, value) => {
            store.set(key, value);
            return "+OK\r\n".to_string();
        },

        Command::Get(key) => {
            match store.get(&key) {
                Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                None => "-Key not found\r\n".to_string(),
            }
        },

        Command::Unknown(input) => format!("-Unknown or malformed command: {}\r\n", input)
    }
}

