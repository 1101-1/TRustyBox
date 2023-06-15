use axum::{
    http::{Response, StatusCode},
    Json,
};

use super::types::upload_types::UploadResponse;

pub async fn upload_err_resp(
    err: String,
    status_code: StatusCode,
) -> (axum::http::StatusCode, Json<UploadResponse>) {
    let response = UploadResponse {
        short_path: None,
        full_url: None,
        error: Some(err),
        aes_key: None,
    };
    return (status_code, Json(response));
}

pub async fn download_err_resp(
    err: String, 
    status_code: StatusCode
) -> Response<axum::body::Body> {
    let response = Response::builder()
        .status(status_code)
        .body(err.into())
        .unwrap();
    response
}
