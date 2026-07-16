//! Middleware Module
//! 
//! Request/response processing middleware pipeline.
//! 
//! # Purpose
//! 
//! Middleware provides a pipeline for processing requests and responses.
//! It handles cross-cutting concerns like logging, metrics, and tracing.
//! 
//! # Current Middleware
//! 
//! - `RequestLogger`: Logs incoming requests
//! - `MetricsCollector`: Collects routing metrics
//! - `SessionValidator`: Validates session state

use crate::{RequestType, Response, SessionContext};
use std::time::Instant;

/// Request logger middleware
pub struct RequestLogger;

impl RequestLogger {
    /// Log a request
    pub fn log_request(request: &RequestType) {
        log::info!("Request: {:?}", request);
    }

    /// Log a response
    pub fn log_response(response: &Response) {
        log::info!("Response: success={}, node={}", response.success, response.node);
    }
}

/// Metrics collector middleware
pub struct MetricsCollector {
    request_count: u64,
    error_count: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            request_count: 0,
            error_count: 0,
        }
    }

    pub fn record_request(&mut self) {
        self.request_count += 1;
    }

    pub fn record_error(&mut self) {
        self.error_count += 1;
    }

    pub fn error_rate(&self) -> f64 {
        if self.request_count == 0 {
            return 0.0;
        }
        self.error_count as f64 / self.request_count as f64
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Session validator middleware
/// 
/// Validates that session context is properly maintained across requests.
pub struct SessionValidator;

impl SessionValidator {
    /// Validate session before processing
    pub fn validate(session: &SessionContext) -> bool {
        // Session ID should be non-zero
        if session.id() == 0 {
            log::warn!("Invalid session ID");
            return false;
        }
        true
    }

    /// Check session health
    pub fn check_health(session: &SessionContext) -> SessionHealth {
        if session.id() == 0 {
            SessionHealth::Invalid
        } else if session.requires_primary() {
            SessionHealth::PrimaryRoute
        } else {
            SessionHealth::Healthy
        }
    }
}

/// Session health status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionHealth {
    Healthy,
    PrimaryRoute,
    Invalid,
}

/// Request timing middleware
pub struct TimingMiddleware {
    start_time: Option<Instant>,
}

impl TimingMiddleware {
    pub fn new() -> Self {
        Self { start_time: None }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start_time
            .map(|t| t.elapsed().as_millis() as u64)
            .unwrap_or(0)
    }
}

impl Default for TimingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// Process a request through the middleware pipeline
pub async fn process_through_pipeline(
    request: RequestType,
    session: &SessionContext,
) -> Response {
    // Start timing
    let mut timing = TimingMiddleware::new();
    timing.start();

    // Validate session
    if !SessionValidator::validate(session) {
        return Response {
            success: false,
            value: None,
            node: "validation_failed".to_string(),
            latency_ms: timing.elapsed_ms(),
        };
    }

    // Log request
    RequestLogger::log_request(&request);

    // Process request
    let response = crate::router::handle_request(request, session).await;

    // Log response
    RequestLogger::log_response(&response);

    response
}
