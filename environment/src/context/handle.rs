//! Context Handle Module
//! 
//! Provides the low-level handle for session state management.
//! 
//! # Performance Considerations
//! 
//! This module uses thread-local storage for optimal performance in
//! high-throughput scenarios. Thread-local access is faster than
//! synchronized access patterns.

use std::cell::Cell;

#[derive(Default, Clone, Copy)]
struct ContextState {
    requires_primary: bool,
    has_writes: bool,
}

thread_local! {
    static CONTEXT_STATE: Cell<ContextState> = Cell::new(ContextState {
        requires_primary: false,
        has_writes: false,
    });
}

#[derive(Default, Clone)]
pub struct ContextHandle;

impl ContextHandle {
    pub fn new() -> Self {
        Self
    }

    pub fn requires_primary(&self) -> bool {
        CONTEXT_STATE.with(|state| state.get().requires_primary)
    }

    pub fn set_requires_primary(&self, value: bool) {
        CONTEXT_STATE.with(|state| {
            let mut current = state.get();
            current.requires_primary = value;
            state.set(current);
        });
    }

    pub fn record_write(&self) {
        CONTEXT_STATE.with(|state| {
            let mut current = state.get();
            current.has_writes = true;
            current.requires_primary = true;
            state.set(current);
        });
    }

    pub fn has_writes(&self) -> bool {
        CONTEXT_STATE.with(|state| state.get().has_writes)
    }
}
