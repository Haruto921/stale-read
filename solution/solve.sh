#!/bin/bash
set -euo pipefail

cd /app

# Fix the bug by replacing the thread_local implementation with explicit context passing.
# We overwrite the context/mod.rs file with the corrected version.

cat > src/context/mod.rs << 'EOF'
//! Session Context Management
//! 
//! FIXED: Using RefCell for interior mutability within the struct.
//! The struct itself is passed by reference, so it stays with the task regardless of thread migration.

use std::cell::RefCell;

pub struct RequestContext {
    cookie: RefCell<Option<u64>>,
}

impl RequestContext {
    pub fn new() -> Self {
        Self { cookie: RefCell::new(None) }
    }

    pub fn set_cookie(&self, cookie: u64) {
        *self.cookie.borrow_mut() = Some(cookie);
    }

    pub fn get_cookie(&self) -> Option<u64> {
        *self.cookie.borrow()
    }
    
    pub fn clear(&self) {
        *self.cookie.borrow_mut() = None;
    }
}
EOF

# Recompile and run tests to verify
cargo test --test integration_test test_read_your_writes --release