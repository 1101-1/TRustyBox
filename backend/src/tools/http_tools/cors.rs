use tower_http::cors::{Any, CorsLayer};

pub fn create_cors() -> CorsLayer {
    CorsLayer::new().allow_origin(Any)
}
