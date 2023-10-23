use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn middleware_read_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}