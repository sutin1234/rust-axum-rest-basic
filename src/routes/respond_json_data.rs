use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn respond_json_data() -> Json<Data> {
    let resp = Data {
        message: "message respond".to_owned(),
        count: 0,
        username: "sutin axum".to_owned(),
    };
    Json(resp)
}