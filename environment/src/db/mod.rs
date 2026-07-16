//! Database Module

pub mod primary;
pub mod replica;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::task;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum RequestType {
    Read { key: String },
    Write { key: String, value: String },
    Delete { key: String },
}

#[derive(Debug, Clone)]
pub struct Response {
    pub success: bool,
    pub value: Option<String>,
    pub node: String,
    pub latency_ms: u64,
}

/// Global primary storage
static PRIMARY_DATA: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

/// Global replica storage (simulated with lag)
static REPLICA_DATA: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

/// Initialize the database cluster
pub fn init_cluster() {
    let mut primary = PRIMARY_DATA.write().unwrap();
    primary.clear();
    primary.insert("init:key".to_string(), "init:value".to_string());
    
    let mut replica = REPLICA_DATA.write().unwrap();
    replica.clear();
    replica.insert("init:key".to_string(), "init:value".to_string());
}

/// Route a write to primary
pub async fn route_to_primary(request: RequestType) -> (bool, Option<String>) {
    match request {
        RequestType::Write { key, value } => {
            let mut data = PRIMARY_DATA.write().unwrap();
            data.insert(key.clone(), value);
            
            // Simulate async replication with lag
            let key_clone = key.clone();
            let value_clone = data.get(&key).cloned().unwrap_or_default();
            drop(data);
            
            // Replicate asynchronously
            task::spawn_blocking(move || {
                // Simulate 50ms replication lag
                std::thread::sleep(Duration::from_millis(50));
                
                let mut replica = REPLICA_DATA.write().unwrap();
                replica.insert(key_clone, value_clone);
            });
            
            (true, None)
        }
        RequestType::Read { key } => {
            let data = PRIMARY_DATA.read().unwrap();
            let value = data.get(&key).cloned();
            (true, value)
        }
        RequestType::Delete { key } => {
            let mut data = PRIMARY_DATA.write().unwrap();
            data.remove(&key);
            
            // Async delete replication
            let key_clone = key.clone();
            drop(data);
            
            task::spawn_blocking(move || {
                std::thread::sleep(Duration::from_millis(50));
                let mut replica = REPLICA_DATA.write().unwrap();
                replica.remove(&key_clone);
            });
            
            (true, None)
        }
    }
}

/// Route a read to replica
pub async fn route_to_replica(request: RequestType) -> (bool, Option<String>) {
    match request {
        RequestType::Read { key } => {
            // Use blocking read to simulate replica latency
            let key_clone = key.clone();
            let result = task::spawn_blocking(move || {
                let data = REPLICA_DATA.read().unwrap();
                data.get(&key_clone).cloned()
            }).await;
            
            match result {
                Ok(value) => (true, value),
                Err(_) => (false, None),
            }
        }
        _ => (false, None),
    }
}

/// Force replica sync (for testing)
pub fn sync_replica() {
    let primary = PRIMARY_DATA.read().unwrap();
    let mut replica = REPLICA_DATA.write().unwrap();
    *replica = primary.clone();
}
