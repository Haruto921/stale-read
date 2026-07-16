//! Session Module
//! 
//! Higher-level session management utilities.
//! 
//! # Purpose
//! 
//! This module provides additional session management functionality
//! beyond the basic context. It includes session validation,
//! expiration, and state management utilities.

use crate::SessionContext;
use std::collections::HashMap;
use std::sync::Mutex;

/// Session registry for tracking active sessions
pub struct SessionRegistry {
    sessions: Mutex<HashMap<u64, SessionInfo>>,
}

#[derive(Debug, Clone)]
struct SessionInfo {
    created_at: std::time::Instant,
    last_access: std::time::Instant,
    request_count: u64,
}

impl SessionRegistry {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Register a new session
    pub fn register(&self, ctx: &SessionContext) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(ctx.id(), SessionInfo {
            created_at: std::time::Instant::now(),
            last_access: std::time::Instant::now(),
            request_count: 0,
        });
    }

    /// Record activity for a session
    pub fn record_activity(&self, ctx: &SessionContext) {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(info) = sessions.get_mut(&ctx.id()) {
            info.last_access = std::time::Instant::now();
            info.request_count += 1;
        }
    }

    /// Get session age in seconds
    pub fn session_age(&self, ctx: &SessionContext) -> u64 {
        let sessions = self.sessions.lock().unwrap();
        if let Some(info) = sessions.get(&ctx.id()) {
            info.created_at.elapsed().as_secs()
        } else {
            0
        }
    }

    /// Get session request count
    pub fn request_count(&self, ctx: &SessionContext) -> u64 {
        let sessions = self.sessions.lock().unwrap();
        if let Some(info) = sessions.get(&ctx.id()) {
            info.request_count
        } else {
            0
        }
    }

    /// Check if session is stale (no activity for N seconds)
    pub fn is_stale(&self, ctx: &SessionContext, timeout_secs: u64) -> bool {
        let sessions = self.sessions.lock().unwrap();
        if let Some(info) = sessions.get(&ctx.id()) {
            info.last_access.elapsed().as_secs() > timeout_secs
        } else {
            true
        }
    }

    /// Clean up old sessions
    pub fn cleanup(&self, max_age_secs: u64) -> usize {
        let mut sessions = self.sessions.lock().unwrap();
        let before = sessions.len();
        sessions.retain(|_, info| {
            info.created_at.elapsed().as_secs() < max_age_secs
        });
        before - sessions.len()
    }
}

impl Default for SessionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate session consistency
pub fn validate_session(ctx: &SessionContext) -> SessionValidationResult {
    if ctx.id() == 0 {
        return SessionValidationResult::Invalid("Zero session ID".to_string());
    }
    
    if ctx.has_writes() && !ctx.requires_primary() {
        return SessionValidationResult::Invalid(
            "Session has writes but doesn't require primary".to_string()
        );
    }
    
    SessionValidationResult::Valid
}

/// Session validation result
#[derive(Debug, Clone, PartialEq)]
pub enum SessionValidationResult {
    Valid,
    Invalid(String),
}

/// Check if a session is in a valid state for routing
pub fn is_valid_for_routing(ctx: &SessionContext) -> bool {
    matches!(validate_session(ctx), SessionValidationResult::Valid)
}

/// Get session status summary
pub fn session_status(ctx: &SessionContext) -> String {
    let mut parts = vec![];
    
    parts.push(format!("id={}", ctx.id()));
    
    if ctx.has_writes() {
        parts.push("writes=true".to_string());
    }
    
    if ctx.requires_primary() {
        parts.push("primary=required".to_string());
    } else {
        parts.push("primary=optional".to_string());
    }
    
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let ctx = SessionContext::new();
        assert_eq!(validate_session(&ctx), SessionValidationResult::Valid);
        
        ctx.record_write();
        assert!(ctx.requires_primary());
        assert_eq!(validate_session(&ctx), SessionValidationResult::Valid);
    }

    #[test]
    fn test_registry() {
        let registry = SessionRegistry::new();
        let ctx = SessionContext::new();
        
        registry.register(&ctx);
        registry.record_activity(&ctx);
        
        assert_eq!(registry.request_count(&ctx), 1);
    }
}
