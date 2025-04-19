use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let db = self.data.lock().unwrap();
        db.get(key).cloned()
    }

    pub fn set(&self, key: &String, value: &String) {
        let mut db = self.data.lock().unwrap();
        db.insert(key.to_string(), value.to_string());
    }

    pub fn del(&self, keys: &Vec<String>) -> usize {
        let mut db = self.data.lock().unwrap();
        let mut count = 0;

        for key in keys {
            if db.remove(key.as_str()).is_some() {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let store = Store::new();

        store.set(&"foo".to_string(), &"bar".to_string());
        let value = store.get("foo");

        assert_eq!(value, Some("bar".to_string()));
    }

    #[test]
    fn test_get_missing_key() {
        let store = Store::new();

        let value = store.get("nonexistent");
        assert_eq!(value, None);
    }
}
