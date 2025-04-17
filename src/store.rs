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

    pub fn set(&self, key: String, value: String) {
        let mut db = self.data.lock().unwrap();
        db.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let store = Store::new();

        store.set("foo".to_string(), "bar".to_string());
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
