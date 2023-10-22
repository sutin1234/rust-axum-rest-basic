mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_path_variable;
mod mirror_query_params;
mod mirror_user_agent;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_path_variable::mirror_path_variable;
use mirror_query_params::mirror_query_params;
use mirror_user_agent::mirror_user_agent;

pub fn create_routes() -> Router<(), Body> {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/mirror_path_variable/:id", get(mirror_path_variable))
        .route("/mirror_query_params", get(mirror_query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
}
