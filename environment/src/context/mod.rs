//! Session Context Management
//! 
//! This module manages the consistency cookie that dictates routing behavior.

use std::cell::Cell;

// BUG: Using thread_local! in an async environment with work-stealing.
// When a task awaits, it may resume on a different thread.
// The thread_local storage will not be migrated, causing the cookie to be lost.
thread_local! {
    static CONSISTENCY_COOKIE: Cell<Option<u64>> = const { Cell::new(None) };
}

pub struct RequestContext;

impl RequestContext {
    pub fn new() -> Self {
        Self
    }

    pub fn set_cookie(&self, cookie: u64) {
        CONSISTENCY_COOKIE.with(|c| c.set(Some(cookie)));
    }

    pub fn get_cookie(&self) -> Option<u64> {
        CONSISTENCY_COOKIE.with(|c| c.get())
    }
    
    pub fn clear(&self) {
        CONSISTENCY_COOKIE.with(|c| c.set(None));
    }
}