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
