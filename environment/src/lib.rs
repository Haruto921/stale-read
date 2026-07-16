pub mod db;
pub mod context;
pub mod router;

pub use router::handle_request;
pub use context::RequestContext;

pub async fn process_request(req: RequestType) -> Response {
    let ctx = RequestContext::new();
    handle_request(req, &ctx).await
}

#[derive(Debug, Clone)]
pub enum RequestType {
    Write { key: String, value: String },
    Read { key: String },
}

#[derive(Debug, Clone)]
pub struct Response {
    pub success: bool,
    pub value: Option<String>,
}
