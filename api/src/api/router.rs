use std::sync::Arc;

use axum::{Router, response::IntoResponse};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::{AppState, error::AppError};

pub fn router(app_state: Arc<AppState>) -> Router
{   
    //let static_router = super::static_router(Arc::clone(&app_state));
    let app_router = super::app_router(Arc::clone(&app_state));
    Router::new()      
        // .route(&with_api_version(ApiVersion::V1, "/sse"), get(crate::services::sse_handler))
        .with_state(app_state.clone())
        .layer(super::layers::cors_layer(app_state.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .merge(app_router)
        //.merge(static_router)
}


async fn handler_404() -> impl IntoResponse 
{
    AppError::PathNotFound
}


pub fn combine_path(path: &'static str) -> String
{
    ["api/v1/", path].concat()
}
pub fn with_api_version(version: ApiVersion, path: &'static str) -> String
{
    [version.as_ref(), path].concat()
}
/// /api_v{version number}
pub enum ApiVersion 
{
    V1,
    V2
}
impl AsRef<str> for ApiVersion
{
    fn as_ref(&self) -> &str 
    {
        match self 
        {
            ApiVersion::V1 => "/api/v1",
            ApiVersion::V2 => "/api/v2"    
        }
    }
}