use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PrimaryNode {
    storage: Arc<RwLock<HashMap<String, String>>>,
    start_time: u64,
}

impl PrimaryNode {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn write(&mut self, key: String, value: String) -> bool {
        let mut data = self.storage.write().unwrap();
        data.insert(key, value);
        true
    }

    pub fn read(&self, key: &str) -> Option<String> {
        let data = self.storage.read().unwrap();
        data.get(key).cloned()
    }
}

impl Default for PrimaryNode {
    fn default() -> Self {
        Self::new()
    }
}
