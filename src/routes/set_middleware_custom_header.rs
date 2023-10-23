use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

use crate::routes::middleware_read_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers.get("message").ok_or_else(|| {
        StatusCode::BAD_REQUEST
    })?;
    let msg = message.to_str().map_err(|_error| StatusCode::BAD_REQUEST)?.to_owned();
    let extension = request.extensions_mut();
    extension.insert(HeaderMessage(msg));
    Ok(next.run(request).await)
}