//! Request Router Module
//! 
//! Routes database requests to appropriate nodes based on session state.
//! 
//! # Routing Logic
//! 
//! The router examines the session context to determine whether to route
//! requests to the primary or replica nodes:
//! 
//! - Write operations always go to primary
//! - Read operations go to primary if session requires it
//! - Otherwise, reads go to replica for load distribution

use crate::{RequestType, Response, SessionContext, db};
use std::time::Instant;

/// Route decision
#[derive(Debug, Clone, PartialEq)]
pub enum RouteDecision {
    Primary,
    Replica,
    Error(String),
}

/// Handle a request with routing
pub async fn handle_request(
    request: RequestType,
    session: &SessionContext,
) -> Response {
    let decision = decide_route(&request, session);
    
    match decision {
        RouteDecision::Primary => {
            let start = Instant::now();
            let result = db::route_to_primary(request).await;
            let elapsed = start.elapsed().as_millis() as u64;
            Response {
                success: result.0,
                value: result.1,
                node: "primary".to_string(),
                latency_ms: elapsed,
            }
        }
        RouteDecision::Replica => {
            let start = Instant::now();
            let result = db::route_to_replica(request).await;
            let elapsed = start.elapsed().as_millis() as u64;
            Response {
                success: result.0,
                value: result.1,
                node: "replica".to_string(),
                latency_ms: elapsed,
            }
        }
        RouteDecision::Error(msg) => {
            Response {
                success: false,
                value: None,
                node: "error".to_string(),
                latency_ms: 0,
            }
        }
    }
}

/// Decide which node to route to
fn decide_route(request: &RequestType, session: &SessionContext) -> RouteDecision {
    match request {
        // Writes always go to primary
        RequestType::Write { .. } => {
            session.record_write();
            RouteDecision::Primary
        }
        RequestType::Delete { .. } => {
            session.record_write();
            RouteDecision::Primary
        }
        // Reads go to primary if session requires it
        RequestType::Read { .. } => {
            if session.requires_primary() {
                RouteDecision::Primary
            } else {
                RouteDecision::Replica
            }
        }
    }
}

/// Route statistics for monitoring
pub struct RouterStats {
    pub primary_routes: u64,
    pub replica_routes: u64,
    pub errors: u64,
}

impl RouterStats {
    pub fn new() -> Self {
        Self {
            primary_routes: 0,
            replica_routes: 0,
            errors: 0,
        }
    }

    pub fn record_primary(&mut self) {
        self.primary_routes += 1;
    }

    pub fn record_replica(&mut self) {
        self.replica_routes += 1;
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn primary_ratio(&self) -> f64 {
        let total = self.primary_routes + self.replica_routes;
        if total == 0 {
            return 0.0;
        }
        self.primary_routes as f64 / total as f64
    }
}

impl Default for RouterStats {
    fn default() -> Self {
        Self::new()
    }
}
