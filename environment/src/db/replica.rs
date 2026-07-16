use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ReplicaNode {
    storage: Arc<RwLock<HashMap<String, String>>>,
    last_sync: u64,
}

impl ReplicaNode {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            last_sync: 0,
        }
    }

    pub fn read(&self, key: &str) -> Option<String> {
        let data = self.storage.read().unwrap();
        data.get(key).cloned()
    }

    pub fn sync_from_primary(&mut self, data: &HashMap<String, String>) {
        let mut storage = self.storage.write().unwrap();
        *storage = data.clone();
        self.last_sync = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn last_sync(&self) -> u64 {
        self.last_sync
    }
}

impl Default for ReplicaNode {
    fn default() -> Self {
        Self::new()
    }
}
