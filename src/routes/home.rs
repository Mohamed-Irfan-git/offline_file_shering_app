use axum::{
    http::{header, HeaderMap},
    response::Html,
};

use crate::{
    templates::html::page_html,
    utils::{network, qr::generate_qr_svg},
};

pub async fn home(headers: HeaderMap) -> Html<String> {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok());

    let (share_url, _urls, host_is_local) = network::resolve_share_url(host);
    let qr_svg = generate_qr_svg(&share_url);

    Html(page_html(&share_url, &qr_svg, host_is_local))
}
