use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn always_success() -> Response {
    (
        StatusCode::CREATED,
        "this is 201".to_owned()
    ).into_response()
}