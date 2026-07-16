//! Main binary for stale-read-expert

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    println!("Stale Read Expert - Run tests with: cargo test");
}
