use super::DbState;
use crate::{RequestType, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ReplicaDb {
    state: Arc<Mutex<DbState>>,
}

impl ReplicaDb {
    pub fn new(state: Arc<Mutex<DbState>>) -> Self {
        Self { state }
    }

    pub async fn execute(&self, op: RequestType) -> Response {
        let state = self.state.lock().await;
        // Simulate replication lag:
        // The replica always returns the data as it was at version-1
        // For simplicity in this benchmark: if version > 0, we return the PREVIOUS value for the key
        // To make it deterministic: We just return None or an old value if we tracked history.
        // SIMPLIFIED BUG LOGIC:
        // The replica is simply "behind". If the primary just wrote, the replica doesn't see it yet.
        // We simulate this by returning None for any key that was written in the current "tick"
        // Actually, let's just make it return a hardcoded stale value or None if the primary has data.
        
        match op {
            RequestType::Write { .. } => {
                // Replicas don't accept writes directly in this pattern
                Response { success: false, value: None }
            }
            RequestType::Read { key } => {
                // BUG SIMULATION:
                // If the primary has the value, the replica deliberately DOES NOT see it yet.
                // It returns None (or an old value).
                // The test expects the NEW value. If it gets None, it fails.
                Response { success: true, value: None } 
            }
        }
    }
}