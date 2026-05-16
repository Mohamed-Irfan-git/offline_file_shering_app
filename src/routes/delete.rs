use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
};

use crate::utils::path_safety::upload_file_path;

pub async fn delete_file(Path(filename): Path<String>) -> impl IntoResponse {
    if let Some(path) = upload_file_path(&filename) {
        if path.exists() {
            let _ = std::fs::remove_file(path);
        }
    }

    Redirect::to("/")
}
