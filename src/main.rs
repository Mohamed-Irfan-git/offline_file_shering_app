mod models;
mod routes;
mod templates;
mod utils;
// main file
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};

use routes::{
    delete::delete_file,
    devices::{get_devices, register_device, rename_device},
    download::download_file,
    files::{delete_file_api, list_files},
    home::home,
    network::{get_network, get_qr_svg},
    ping::ping,
    upload::upload_file,
};

use std::{
    collections::HashMap,
    fs,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tower_http::services::ServeDir;

use utils::network;

#[tokio::main]
async fn main() {
    fs::create_dir_all("uploads").unwrap();

    let devices = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(home))
        .route("/upload", post(upload_file))
        .route("/delete/:filename", get(delete_file))
        .route("/download/:filename", get(download_file))
        .route("/api/files", get(list_files))
        .route("/api/files/:filename", delete(delete_file_api))
        .route("/api/devices/register", post(register_device))
        .route("/api/devices/rename", post(rename_device))
        .route("/api/devices", get(get_devices))
        .route("/api/network", get(get_network))
        .route("/api/ping", get(ping))
        .route("/api/qr.svg", get(get_qr_svg))
        .nest_service("/files", ServeDir::new("uploads"))
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::disable())
        .with_state(devices);

    let addr = SocketAddr::from(([0, 0, 0, 0], network::PORT));

    println!("LAN Share — share any file on your local network");
    println!("This device (host): http://localhost:{}", network::PORT);

    for url in network::lan_urls() {
        println!("Other devices use: {}", url);
    }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
