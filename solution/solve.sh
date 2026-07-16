#!/bin/bash
# Solution for Stale Read Expert
# Bug: thread_local storage in ContextHandle doesn't persist across task migration
# Fix: Use Arc<Mutex<>> instead of thread_local

set -euo pipefail

cd "$(dirname "$0")/../environment"

echo "=== Stale Read Expert - Applying Fix ==="

cat > src/context/handle.rs << 'EOF'
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct ContextState {
    pub requires_primary: bool,
    pub has_writes: bool,
}

#[derive(Default, Clone)]
pub struct ContextHandle {
    state: Arc<Mutex<ContextState>>,
}

impl ContextHandle {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ContextState::default())),
        }
    }

    pub fn requires_primary(&self) -> bool {
        self.state.lock().unwrap().requires_primary
    }

    pub fn set_requires_primary(&self, value: bool) {
        self.state.lock().unwrap().requires_primary = value;
    }

    pub fn record_write(&self) {
        let mut state = self.state.lock().unwrap();
        state.has_writes = true;
        state.requires_primary = true;
    }

    pub fn has_writes(&self) -> bool {
        self.state.lock().unwrap().has_writes
    }
}
EOF

echo "Fixed: Replaced thread_local with Arc<Mutex<ContextState>>"

echo ""
echo "=== Running Tests ==="
cargo test --test integration_test --release -- --test-threads=1 --nocapture

echo ""
echo "=== All Tests Passed ==="
