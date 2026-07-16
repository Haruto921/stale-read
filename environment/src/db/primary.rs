use super::DbState;
use crate::{RequestType, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PrimaryDb {
    state: Arc<Mutex<DbState>>,
}

impl PrimaryDb {
    pub fn new(state: Arc<Mutex<DbState>>) -> Self {
        Self { state }
    }

    pub async fn execute(&self, op: RequestType) -> Response {
        let mut state = self.state.lock().await;
        match op {
            RequestType::Write { key, value } => {
                state.data.insert(key.clone(), value.clone());
                state.version += 1;
                Response { success: true, value: None }
            }
            RequestType::Read { key } => {
                let val = state.data.get(&key).cloned();
                Response { success: true, value: val }
            }
        }
    }
}