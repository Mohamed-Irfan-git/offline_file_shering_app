use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct PingResponse {
    pub ok: bool,
}

pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse { ok: true })
}
