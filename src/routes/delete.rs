use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
};

use std::{
    fs,
    path::Path as StdPath,
};

pub async fn delete_file(
    Path(filename): Path<String>,
) -> impl IntoResponse {
    let file_path = format!("uploads/{}", filename);

    if StdPath::new(&file_path).exists() {
        let _ = fs::remove_file(file_path);
    }

    Redirect::to("/")
}