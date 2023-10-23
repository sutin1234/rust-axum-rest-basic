use axum::{body::Body, Extension, middleware, Router, routing::{get, post}};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

use always_error::always_error;
use always_success::always_success;
use hello_world::hello_world;
use middleware_message::middleware_message;
use middleware_read_custom_header::middleware_read_custom_header;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_path_variable::mirror_path_variable;
use mirror_query_params::mirror_query_params;
use mirror_user_agent::mirror_user_agent;
use respond_json_data::respond_json_data;
use set_middleware_custom_header::set_middleware_custom_header;
use validate_income_data::validate_income_data;

mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_path_variable;
mod mirror_query_params;
mod mirror_user_agent;
mod middleware_message;
mod middleware_read_custom_header;
mod set_middleware_custom_header;
mod always_error;
mod always_success;
mod respond_json_data;
mod validate_income_data;

#[derive(Clone)]
pub struct ShareData {
    pub message: String,
}

pub fn create_routes() -> Router<(), Body> {
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data: ShareData = ShareData {
        message: "middleware shared data".to_owned(),
    };
    Router::new()
        .route("/middleware_read_custom_header", get(middleware_read_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/mirror_path_variable/:id", get(mirror_path_variable))
        .route("/mirror_query_params", get(mirror_query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_error", get(always_error))
        .route("/always_success", get(always_success))
        .route("/respond_json_data", get(respond_json_data))
        .route("/validate_income_data", post(validate_income_data))
        .layer(cors)
        .layer(Extension(shared_data))
}
