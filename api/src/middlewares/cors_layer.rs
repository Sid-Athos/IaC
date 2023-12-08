use std::str::FromStr;
use axum::http::HeaderName;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};


pub fn init_cors_layer()-> CorsLayer {
    let allowed_headers = vec![HeaderName::from_str("api-key").unwrap(), HeaderName::from_str("Content-Type").unwrap()];
    CorsLayer::new()
        .allow_headers(AllowHeaders::from(allowed_headers))
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
}