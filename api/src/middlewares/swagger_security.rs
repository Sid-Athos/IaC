use std::env;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use utoipa::Modify;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let schemes = vec![("api-key" ,SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("api-key")))),
                           ("bearer", SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()))].into_iter();
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_schemes_from_iter(
                schemes
            )
        }
    }
}
pub async fn check_api_key<B>(req: Request<B>, next: Next<B>) -> Result<Response, (StatusCode, String)>{
    match req.headers().get("api-key") {
        Some(header) if header != &env::var("API_KEY").unwrap() => {
            tracing::warn!("Invalid api key");
            Err((
                StatusCode::UNAUTHORIZED,
                "Incorrect or missing Api Key".to_string(),
            ))
        },
        None => {
            tracing::warn!("Missing api key");
            Err((
                StatusCode::UNAUTHORIZED,
                "Incorrect or missing Api Key".to_string(),
            ))
        },
        _ => {
            tracing::info!("Valid api-key provided");
            Ok(next.run(req).await)
        },
    }
}