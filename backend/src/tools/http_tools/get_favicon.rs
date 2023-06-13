use std::convert::Infallible;

use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Response},
    response::IntoResponse,
};

pub async fn favicon() -> Result<impl IntoResponse, Infallible> {
    let favicon_path = "frontend/favicon.ico";
    let img = tokio::fs::read(&favicon_path)
        .await
        .expect("Cannot upload img");

    let mut response: Response<Body> = Response::new(img.into());
    response
        .headers_mut()
        .insert(CONTENT_TYPE, "image/x-icon".parse().unwrap());

    Ok(response)
}
