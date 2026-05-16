mod routes;
mod templates;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};

use local_ip_address::local_ip;

use routes::{
    delete::delete_file,
    home::home,
    upload::upload_file,
};

use std::{
    fs,
    net::SocketAddr,
};

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    fs::create_dir_all("uploads").unwrap();

    let app = Router::new()

        .route("/", get(home))

        .route("/upload", post(upload_file))

        .route(
            "/delete/:filename",
            get(delete_file),
        )

        .nest_service(
            "/files",
            ServeDir::new("uploads"),
        )

        .nest_service(
            "/static",
            ServeDir::new("static"),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    println!("Server running:");
    println!("Local: http://localhost:5000");

    if let Ok(ip) = local_ip() {
        println!("LAN: http://{}:5000", ip);
    }

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}