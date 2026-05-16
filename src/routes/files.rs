use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use std::fs;

use crate::{
    models::FileInfo,
    utils::{
        file_icon::get_file_icon, file_name::display_file_name, file_size::format_file_size,
        path_safety::upload_file_path,
    },
};

pub async fn list_files() -> Json<Vec<FileInfo>> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir("uploads") {
        for entry in entries.flatten() {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let file_name = entry.file_name().to_string_lossy().to_string();

            let metadata = entry.metadata().ok();
            let size_bytes = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            let modified = metadata
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            let lower = file_name.to_lowercase();
            let is_image = lower.ends_with(".png")
                || lower.ends_with(".jpg")
                || lower.ends_with(".jpeg")
                || lower.ends_with(".gif")
                || lower.ends_with(".webp");

            files.push(FileInfo {
                name: file_name.clone(),
                display_name: display_file_name(&file_name),
                size: format_file_size(size_bytes),
                size_bytes,
                icon: get_file_icon(&file_name).to_string(),
                is_image,
                modified,
            });
        }
    }

    files.sort_by(|a, b| b.modified.cmp(&a.modified));

    Json(files)
}

pub async fn delete_file_api(Path(filename): Path<String>) -> impl IntoResponse {
    let Some(path) = upload_file_path(&filename) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    if path.exists() && fs::remove_file(path).is_ok() {
        return StatusCode::OK.into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}
