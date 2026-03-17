use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::info;

/// Middleware that logs incoming requests with method, path, status, and latency.
///
/// This middleware wraps the request handler and logs:
/// - HTTP method (GET, POST, etc.)
/// - Request path
/// - Response status code
/// - Request latency in milliseconds
///
/// # Examples
///
/// ```text
/// GET /api/users 200 15ms
/// POST /api/users 201 42ms
/// GET /api/users/123 404 3ms
/// ```
///
/// # Usage
///
/// This middleware is automatically applied to all routes when using
/// [`APIApp::run`](crate::APIApp::run).
///
/// # Arguments
///
/// * `req` - The incoming HTTP request
/// * `next` - The next middleware/handler in the chain
///
/// # Returns
///
/// The HTTP response from the handler
pub async fn log_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().path().to_string();
    let start = Instant::now();

    let res = next.run(req).await;
    let latency = start.elapsed().as_millis();
    let status = res.status();

    info!("{} {} {} {}ms", method, uri, status, latency);
    res
}
