pub mod handle;
pub mod vault;

pub use handle::ContextHandle;

/// Maximum number of active sessions
const MAX_SESSIONS: usize = 10_000;

/// Session context for routing decisions
#[derive(Clone)]
pub struct SessionContext {
    /// Internal handle
    handle: ContextHandle,
    /// Session ID
    session_id: u64,
}

impl SessionContext {
    /// Create a new session context
    pub fn new() -> Self {
        Self {
            handle: ContextHandle::new(),
            session_id: Self::generate_session_id(),
        }
    }

    /// Generate a unique session ID
    fn generate_session_id() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    /// Get the session ID
    pub fn id(&self) -> u64 {
        self.session_id
    }

    /// Check if this session requires primary routing
    pub fn requires_primary(&self) -> bool {
        self.handle.requires_primary()
    }

    /// Set the primary requirement flag
    pub fn set_requires_primary(&self, value: bool) {
        self.handle.set_requires_primary(value)
    }

    /// Record that a write occurred in this session
    pub fn record_write(&self) {
        self.handle.record_write()
    }

    /// Check if any writes occurred in this session
    pub fn has_writes(&self) -> bool {
        self.handle.has_writes()
    }
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let ctx = SessionContext::new();
        assert!(!ctx.requires_primary());
    }

    #[test]
    fn test_session_id_unique() {
        let ctx1 = SessionContext::new();
        let ctx2 = SessionContext::new();
        assert_ne!(ctx1.id(), ctx2.id());
    }
}
