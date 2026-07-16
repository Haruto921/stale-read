//! Token Manager
//! 
//! This module manages the consistency token that tracks whether a write
//! has occurred in the current session.
//! 
//! FIX: Changed from thread_local to Arc-based storage for cross-thread persistence.

use std::sync::{Arc, Mutex};

/// Token manager for consistency tracking
pub struct TokenManager {
    token: Arc<Mutex<Option<u64>>>,
}

impl TokenManager {
    /// Create a new token manager
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
        }
    }

    /// Set the consistency token
    pub fn set_token(&self, token: u64) {
        *self.token.lock().unwrap() = Some(token);
    }

    /// Get the current token
    pub fn get_token(&self) -> Option<u64> {
        *self.token.lock().unwrap()
    }

    /// Clear the token
    pub fn clear(&self) {
        *self.token.lock().unwrap() = None;
    }
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_basic() {
        let manager = TokenManager::new();
        
        assert!(manager.get_token().is_none());
        
        manager.set_token(12345);
        assert_eq!(manager.get_token(), Some(12345));
        
        manager.clear();
        assert!(manager.get_token().is_none());
    }
}
