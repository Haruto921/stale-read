//! Utility Module
//! 
//! Common utilities and helper functions.
//! 
//! # Contents
//! 
//! - Time utilities
//! - Hash utilities
//! - String utilities
//! - Concurrency helpers

use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Get current timestamp in milliseconds
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as u64
}

/// Get current timestamp in seconds
pub fn current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

/// Hash a string using FNV-1a
pub fn fnv_hash(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Generate a random string of given length
pub fn random_string(len: usize) -> String {
    use std::collections::vec_deque::VecDeque;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    let mut rng_state = fnv_hash(&format!("{}", current_timestamp_ms())) as usize;
    
    (0..len)
        .map(|_| {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let idx = (rng_state / 65536) as usize % CHARSET.len();
            CHARSET[idx] as char
        })
        .collect()
}

/// Sleep for a given duration in async context
pub async fn async_sleep(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

/// Yield to the scheduler
pub async fn yield_now() {
    tokio::task::yield_now().await;
}

/// Spin wait for a condition
pub async fn spin_wait<F, Fut>(check: F, timeout_ms: u64) -> bool
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = current_timestamp_ms();
    loop {
        if check().await {
            return true;
        }
        if current_timestamp_ms() - start > timeout_ms {
            return false;
        }
        yield_now().await;
    }
}

/// Atomic counter for generating unique IDs
pub struct IdGenerator {
    counter: std::sync::atomic::AtomicU64,
}

impl IdGenerator {
    pub fn new(start: u64) -> Self {
        Self {
            counter: std::sync::atomic::AtomicU64::new(start),
        }
    }

    pub fn next(&self) -> u64 {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new(1)
    }
}

/// Measure execution time of a future
pub async fn measure_time<F, T>(f: F) -> (T, Duration)
where
    F: std::future::Future<Output = T>,
{
    let start = std::time::Instant::now();
    let result = f.await;
    (result, start.elapsed())
}

/// Format duration in human readable form
pub fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{}.{:03}s", d.as_secs(), d.subsec_millis())
    } else if d.as_millis() > 0 {
        format!("{}.{:03}ms", d.as_millis(), d.subsec_micros() % 1000)
    } else {
        format!("{}µs", d.as_micros())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv_hash() {
        assert_eq!(fnv_hash("hello"), fnv_hash("hello"));
        assert_ne!(fnv_hash("hello"), fnv_hash("world"));
    }

    #[test]
    fn test_random_string() {
        let s1 = random_string(16);
        let s2 = random_string(16);
        assert_eq!(s1.len(), 16);
        assert_eq!(s2.len(), 16);
        assert_ne!(s1, s2);
    }

    #[tokio::test]
    async fn test_yield() {
        let mut count = 0;
        for _ in 0..10 {
            yield_now().await;
            count += 1;
        }
        assert_eq!(count, 10);
    }
}
