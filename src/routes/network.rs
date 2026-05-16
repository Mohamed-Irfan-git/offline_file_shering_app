use axum::{
    http::{header, HeaderMap},
    response::IntoResponse,
    Json,
};

use crate::{
    models::NetworkInfo,
    utils::{
        network::{self, PORT},
        qr::generate_qr_svg,
    },
};

pub async fn get_network(headers: HeaderMap) -> Json<NetworkInfo> {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok());

    let (share_url, urls, host_is_local) = network::resolve_share_url(host);

    Json(NetworkInfo {
        share_url: share_url.clone(),
        urls,
        host_is_local,
        port: PORT,
        qr_svg: generate_qr_svg(&share_url),
    })
}

pub async fn get_qr_svg(headers: HeaderMap) -> impl IntoResponse {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok());

    let (share_url, _, _) = network::resolve_share_url(host);

    (
        [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        generate_qr_svg(&share_url),
    )
}
