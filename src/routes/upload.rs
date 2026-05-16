use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
};

use tokio::io::AsyncWriteExt;

use uuid::Uuid;

pub async fn upload_file(
    mut multipart: Multipart,
) -> impl IntoResponse {

    while let Some(field) = multipart
        .next_field()
        .await
        .unwrap()
    {

        let original_name = field
            .file_name()
            .unwrap_or("file")
            .to_string();

        let extension = original_name
            .split('.')
            .last()
            .unwrap_or("");

        let saved_name = format!(
            "{}.{}",
            Uuid::new_v4(),
            extension
        );

        let path = format!(
            "uploads/{}",
            saved_name
        );

        let data = field
            .bytes()
            .await
            .unwrap();

        let mut file = tokio::fs::File::create(&path)
            .await
            .unwrap();

        file.write_all(&data)
            .await
            .unwrap();

        return Html(
            "Upload completed".to_string()
        );
    }

    Html(
        "No file uploaded".to_string()
    )
}