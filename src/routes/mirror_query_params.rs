use axum::extract::{Json, Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    id: i32,
    name: String,
}

pub async fn mirror_query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
