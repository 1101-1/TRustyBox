use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors() -> CorsLayer {
    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
    .allow_origin(Any);
    cors
}
