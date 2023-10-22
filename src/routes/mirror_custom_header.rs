use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let username_value = headers.get("X-UserName").unwrap();
    let username = username_value.to_str().unwrap().to_owned();
    username
}
