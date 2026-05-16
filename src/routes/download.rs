use axum::{
    body::Body,
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use tokio_util::io::ReaderStream;

use crate::utils::{
    file_name::display_file_name,
    path_safety::upload_file_path,
};

pub async fn download_file(
    Path(filename): Path<String>,
) -> impl IntoResponse {
    let Some(path) = upload_file_path(&filename) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let file = match tokio::fs::File::open(&path).await {
        Ok(f) => f,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let metadata = match file.metadata().await {
        Ok(m) => m,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mime = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    let display = display_file_name(&filename);

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let disposition = format!("attachment; filename=\"{}\"", display);

    match Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CONTENT_LENGTH, metadata.len())
        .header(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&disposition).unwrap_or_else(|_| {
                HeaderValue::from_static("attachment")
            }),
        )
        .body(body)
    {
        Ok(response) => response.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
