use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UploadResponse {
    pub short_path: Option<String>,
    pub error: Option<String>,
    pub full_url: Option<String>,
    pub aes_key: Option<String>,
}

#[derive(Deserialize)]
pub struct UploadPayload {
    pub encryption: Option<String>,
}
