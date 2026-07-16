use crate::SessionContext;
use std::collections::VecDeque;
use std::sync::Mutex;

pub struct PoolConfig {
    pub max_size: usize,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self { max_size: 1000 }
    }
}

pub struct SessionPool {
    pool: Mutex<VecDeque<SessionContext>>,
    config: PoolConfig,
}

impl SessionPool {
    pub fn new() -> Self {
        Self {
            pool: Mutex::new(VecDeque::new()),
            config: PoolConfig::default(),
        }
    }

    pub fn acquire(&self) -> SessionContext {
        let mut pool = self.pool.lock().unwrap();
        pool.pop_front().unwrap_or_else(SessionContext::new)
    }

    pub fn release(&self, ctx: SessionContext) {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < self.config.max_size {
            pool.push_back(ctx);
        }
    }
}

impl Default for SessionPool {
    fn default() -> Self { Self::new() }
}

use once_cell::sync::Lazy;
use std::sync::Arc;

static GLOBAL_POOL: Lazy<Arc<SessionPool>> = Lazy::new(|| Arc::new(SessionPool::new()));

pub fn global_pool() -> Arc<SessionPool> {
    GLOBAL_POOL.clone()
}

pub fn acquire_session() -> SessionContext {
    global_pool().acquire()
}

pub fn release_session(ctx: SessionContext) {
    global_pool().release(ctx);
}
