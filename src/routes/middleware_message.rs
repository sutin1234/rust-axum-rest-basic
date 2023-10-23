use axum::Extension;

use crate::routes::ShareData;

pub async fn middleware_message(Extension(shared_data): Extension<ShareData>) -> String {
    shared_data.message
}