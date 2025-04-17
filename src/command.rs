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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::Store;

    #[test]
    fn test_set() {
        let store = Store::new();
        let set_result = process_command("*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n", &store);
        assert_eq!(set_result, "+OK\r\n");
    }

    #[test]
    fn test_get() {
        let store = Store::new();
        store.set("foo".into(), "bar".into());

        let get_result = process_command("*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n", &store);
        assert_eq!(get_result, "$3\r\nbar\r\n");
    }

    #[test]
    fn test_unknown_command() {
        let store = Store::new();
        let response = process_command("*1\r\n$4\r\nBLAH\r\n", &store);
        assert_eq!(response, "-Unknown or malformed command: *1\r\n$4\r\nBLAH\r\n\r\n");
    }
}
