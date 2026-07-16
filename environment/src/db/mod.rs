mod primary;
mod replica;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{RequestType, Response};
use once_cell::sync::OnceCell;

pub use primary::PrimaryDb;
pub use replica::ReplicaDb;

#[derive(Debug, Default)]
pub struct DbState {
    pub data: std::collections::HashMap<String, String>,
    pub version: u64,
}

pub struct DatabaseCluster {
    pub primary: PrimaryDb,
    pub replica: ReplicaDb,
}

impl DatabaseCluster {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(DbState::default()));
        Self {
            primary: PrimaryDb::new(state.clone()),
            replica: ReplicaDb::new(state),
        }
    }

    pub async fn execute(&self, op: RequestType, use_primary: bool) -> Response {
        if use_primary {
            self.primary.execute(op).await
        } else {
            self.replica.execute(op).await
        }
    }
}

pub static DB_CLUSTER: OnceCell<DatabaseCluster> = OnceCell::new();

pub fn get_cluster() -> &'static DatabaseCluster {
    DB_CLUSTER.get().expect("Database cluster not initialized")
}

pub fn init_cluster() {
    let _ = DB_CLUSTER.set(DatabaseCluster::new());
}