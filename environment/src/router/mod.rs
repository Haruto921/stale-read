use crate::{RequestType, Response, db::get_cluster, context::RequestContext};

pub async fn handle_request(req: RequestType, ctx: &RequestContext) -> Response {
    match req {
        RequestType::Write { ref key, .. } => {
            // Perform write on primary
            let resp = get_cluster().execute(req.clone(), true).await;
            
            if resp.success {
                // Set cookie to enforce primary reads for subsequent operations
                // Using a dummy cookie value (timestamp or ID)
                ctx.set_cookie(12345);
            }
            resp
        }
        RequestType::Read { .. } => {
            // Check cookie to decide routing
            let use_primary = ctx.get_cookie().is_some();
            get_cluster().execute(req, use_primary).await
        }
    }
}