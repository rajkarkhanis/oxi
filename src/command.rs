use crate::store::Store;
use crate::resp::{self, Command};

pub fn process_command(input: &str, store: &Store) -> String {
    let command = resp::parse(input);

    match &command {
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

        Command::Del(keys) => {
            let deleted = store.del(keys);
            format!(":{}\r\n", deleted)
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
        store.set(&"foo".to_string(), &"bar".to_string());

        let get_result = process_command("*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n", &store);
        assert_eq!(get_result, "$3\r\nbar\r\n");
    }

    #[test]
    fn test_unknown_command() {
        let store = Store::new();
        let response = process_command("*1\r\n$4\r\nBLAH\r\n", &store);
        assert_eq!(response, "-Unknown or malformed command: BLAH\r\n");
    }


    #[test]
    fn test_del_existing_and_missing_keys() {
        let store = Store::new();
        store.set(&"foo".to_string(), &"bar".to_string());
        store.set(&"baz".to_string(), &"qux".to_string());
        store.set(&"corge".to_string(), &"grault".to_string());

        let del_some = process_command("*3\r\n$3\r\nDEL\r\n$3\r\nfoo\r\n$3\r\nbar\r\n", &store);
        assert_eq!(del_some, ":1\r\n");

        let del_all = process_command("*4\r\n$3\r\nDEL\r\n$3\r\nfoo\r\n$3\r\nbaz\r\n$5\r\ncorge\r\n", &store);
        assert_eq!(del_all, ":2\r\n");

        let del_none = process_command("*2\r\n$3\r\nDEL\r\n$3\r\nnoop\r\n", &store);
        assert_eq!(del_none, ":0\r\n");
    }
}
