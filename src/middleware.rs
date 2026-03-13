use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::info;

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
