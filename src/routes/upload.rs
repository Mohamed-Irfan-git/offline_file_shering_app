use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};

use serde::Serialize;
use tokio::io::AsyncWriteExt;

use crate::utils::file_name::generate_safe_file_name;

#[derive(Serialize)]
pub struct UploadResponse {
    pub uploaded: Vec<String>,
    pub count: usize,
}

pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    let mut uploaded = Vec::new();

    while let Ok(Some(mut field)) = multipart.next_field().await {
        let original_name = field.file_name().unwrap_or("file").to_string();

        let saved_name = generate_safe_file_name(&original_name);
        let path = format!("uploads/{}", saved_name);

        let mut outfile = match tokio::fs::File::create(&path).await {
            Ok(f) => f,
            Err(_) => continue,
        };

        loop {
            match field.chunk().await {
                Ok(Some(chunk)) => {
                    if outfile.write_all(&chunk).await.is_err() {
                        let _ = tokio::fs::remove_file(&path).await;
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
                Ok(None) => break,
                Err(_) => {
                    let _ = tokio::fs::remove_file(&path).await;
                    return StatusCode::BAD_REQUEST.into_response();
                }
            }
        }

        uploaded.push(saved_name);
    }

    let count = uploaded.len();

    if count == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(UploadResponse {
                uploaded: vec![],
                count: 0,
            }),
        )
            .into_response();
    }

    Json(UploadResponse { uploaded, count }).into_response()
}
