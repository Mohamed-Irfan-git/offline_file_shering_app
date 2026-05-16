use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use local_ip_address::local_ip;
use std::{fs, net::SocketAddr};
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    fs::create_dir_all("uploads").unwrap();

    let app = Router::new()
        .route("/", get(home))
        .route("/upload", post(upload_file))
        .nest_service("/files", ServeDir::new("uploads"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    println!("Server running:");
    println!("Local: http://localhost:5000");

    if let Ok(ip) = local_ip() {
        println!("LAN:   http://{}:5000", ip);
    }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>LAN Share</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                background: #0b0b0b;
                color: white;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
            }
            .card {
                width: 420px;
                background: #151515;
                padding: 30px;
                border-radius: 20px;
                box-shadow: 0 20px 60px rgba(0,0,0,0.4);
            }
            h1 {
                color: #ff6a00;
            }
            input, button {
                width: 100%;
                padding: 14px;
                margin-top: 15px;
                border-radius: 12px;
                border: none;
            }
            button {
                background: #ff6a00;
                color: white;
                font-weight: bold;
                cursor: pointer;
            }
            a {
                color: #ff6a00;
            }
        </style>
    </head>
    <body>
        <div class="card">
            <h1>LAN Share</h1>
            <p>Send files offline inside the same Wi-Fi/LAN.</p>

            <form action="/upload" method="post" enctype="multipart/form-data">
                <input type="file" name="file" required />
                <button type="submit">Upload File</button>
            </form>

            <p>After upload, open:</p>
            <a href="/files">View Uploaded Files</a>
        </div>
    </body>
    </html>
    "#;

    Html(html.to_string())
}

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("file").to_string();
        let extension = file_name
            .split('.')
            .last()
            .unwrap_or("");

        let saved_name = format!("{}.{}", Uuid::new_v4(), extension);
        let path = format!("uploads/{}", saved_name);

        let data = field.bytes().await.unwrap();

        let mut file = tokio::fs::File::create(&path).await.unwrap();
        file.write_all(&data).await.unwrap();

        return Html(format!(
            r#"
            <h2>File uploaded successfully</h2>
            <p>Saved as: {}</p>
            <a href="/files/{}">Download File</a><br><br>
            <a href="/">Upload another file</a>
            "#,
            saved_name, saved_name
        ));
    }

    Html("No file uploaded".to_string())
}