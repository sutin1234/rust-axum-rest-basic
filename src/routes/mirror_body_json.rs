use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJsonRequest {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJsonRequest>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_server: "hello from Axum Server".to_owned(),
    })
}
