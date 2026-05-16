use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub platform: String,
    pub browser: String,
    pub last_seen: i64,
}

#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub platform: String,
    pub browser: String,
}

#[derive(Deserialize)]
pub struct RenameDeviceRequest {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub share_url: String,
    pub urls: Vec<String>,
    pub host_is_local: bool,
    pub port: u16,
    pub qr_svg: String,
}

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub display_name: String,
    pub size: String,
    pub size_bytes: u64,
    pub icon: String,
    pub is_image: bool,
    pub modified: i64,
}
