use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BodyRequest {
    username: Option<String>,
    password: String,
}

pub async fn validate_income_data(Json(body): Json<BodyRequest>) {
    dbg!(body);
    // body
}