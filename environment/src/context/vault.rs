//! State Vault Module
//! 
//! Provides optional synchronized state storage for cross-thread scenarios.
//! 
//! # Overview
//! 
//! This module contains alternative storage backends for session state.
//! While thread-local storage is fast, it doesn't work across threads.
//! 
//! # Available Backends
//! 
//! - `VaultStorage`: Thread-safe storage using Mutex
//! - `AtomicVault`: Lock-free storage using AtomicBool
//! 
//! These are provided as options but are not currently used by default.

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

/// Thread-safe vault storage for session state
/// 
/// This provides synchronized access to session state across threads.
/// It is slower than thread-local but works correctly in all scenarios.
pub struct VaultStorage {
    requires_primary: Arc<Mutex<bool>>,
    has_writes: Arc<Mutex<bool>>,
}

impl VaultStorage {
    /// Create a new vault storage
    pub fn new() -> Self {
        Self {
            requires_primary: Arc::new(Mutex::new(false)),
            has_writes: Arc::new(Mutex::new(false)),
        }
    }

    /// Set the requires primary flag
    pub fn set_requires_primary(&self, value: bool) {
        *self.requires_primary.lock().unwrap() = value;
    }

    /// Get the requires primary flag
    pub fn requires_primary(&self) -> bool {
        *self.requires_primary.lock().unwrap()
    }

    /// Record a write
    pub fn record_write(&self) {
        *self.requires_primary.lock().unwrap() = true;
        *self.has_writes.lock().unwrap() = true;
    }

    /// Check for writes
    pub fn has_writes(&self) -> bool {
        *self.has_writes.lock().unwrap()
    }
}

impl Default for VaultStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Atomic vault storage using lock-free primitives
/// 
/// This is a more advanced implementation using atomic operations.
/// It provides better performance than Mutex-based storage.
pub struct AtomicVault {
    requires_primary: AtomicBool,
    has_writes: AtomicBool,
}

impl AtomicVault {
    /// Create a new atomic vault
    pub fn new() -> Self {
        Self {
            requires_primary: AtomicBool::new(false),
            has_writes: AtomicBool::new(false),
        }
    }

    /// Set requires primary flag
    pub fn set_requires_primary(&self, value: bool) {
        self.requires_primary.store(value, Ordering::SeqCst);
    }

    /// Get requires primary flag
    pub fn requires_primary(&self) -> bool {
        self.requires_primary.load(Ordering::SeqCst)
    }

    /// Record a write operation
    pub fn record_write(&self) {
        self.requires_primary.store(true, Ordering::SeqCst);
        self.has_writes.store(true, Ordering::SeqCst);
    }

    /// Check for writes
    pub fn has_writes(&self) -> bool {
        self.has_writes.load(Ordering::SeqCst)
    }
}

impl Default for AtomicVault {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_storage() {
        let vault = VaultStorage::new();
        assert!(!vault.requires_primary());
        
        vault.record_write();
        assert!(vault.requires_primary());
        assert!(vault.has_writes());
    }

    #[test]
    fn test_atomic_vault() {
        let vault = AtomicVault::new();
        assert!(!vault.requires_primary());
        
        vault.record_write();
        assert!(vault.requires_primary());
        assert!(vault.has_writes());
    }
}
