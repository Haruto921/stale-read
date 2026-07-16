use stale_read_fix::db;

#[tokio::main]
async fn main() {
    // In a real app, this would start a server. 
    // For this benchmark, the logic is exercised via integration tests.
    println!("Service started. Run integration tests to verify consistency.");
}

// Re-export for backward compatibility
pub use stale_read_fix::{process_request, RequestType, Response, db::init_cluster};