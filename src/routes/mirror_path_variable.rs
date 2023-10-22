use axum::extract::Path;

pub async fn mirror_path_variable(Path(id): Path<i32>) -> String {
    id.to_string()
}
