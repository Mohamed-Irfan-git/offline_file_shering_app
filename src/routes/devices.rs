use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use chrono::Utc;
use serde::Deserialize;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::{
    Device,
    RegisterDeviceRequest,
    RenameDeviceRequest,
};

type DeviceStore = Arc<Mutex<HashMap<String, Device>>>;

const ONLINE_TIMEOUT_SECONDS: i64 = 20;

#[derive(Deserialize)]
pub struct DevicesQuery {
    pub exclude: Option<String>,
}

pub async fn register_device(
    State(devices): State<DeviceStore>,
    Json(payload): Json<RegisterDeviceRequest>,
) -> Json<Device> {
    let name = payload.name.trim();

    let device = Device {
        id: payload.id.clone(),
        name: if name.is_empty() {
            format!("{} · {}", payload.platform, payload.browser)
        } else {
            name.to_string()
        },
        device_type: payload.device_type,
        platform: payload.platform,
        browser: payload.browser,
        last_seen: Utc::now().timestamp(),
    };

    let mut store = devices.lock().unwrap();

    store.insert(device.id.clone(), device.clone());

    Json(device)
}

pub async fn rename_device(
    State(devices): State<DeviceStore>,
    Json(payload): Json<RenameDeviceRequest>,
) -> impl IntoResponse {
    let name = payload.name.trim();

    if name.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let mut store = devices.lock().unwrap();

    if let Some(device) = store.get_mut(&payload.id) {
        device.name = name.to_string();
        return Json(device.clone()).into_response();
    }

    StatusCode::NOT_FOUND.into_response()
}

pub async fn get_devices(
    State(devices): State<DeviceStore>,
    axum::extract::Query(query): axum::extract::Query<DevicesQuery>,
) -> Json<Vec<Device>> {
    let now = Utc::now().timestamp();

    let mut store = devices.lock().unwrap();

    store.retain(|_, device| now - device.last_seen <= ONLINE_TIMEOUT_SECONDS);

    let mut online_devices: Vec<Device> = store.values().cloned().collect();

    online_devices.sort_by(|a, b| {
        let a_self = query.exclude.as_ref() == Some(&a.id);
        let b_self = query.exclude.as_ref() == Some(&b.id);

        match (a_self, b_self) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    Json(online_devices)
}
