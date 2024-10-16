// health.rs

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_extra::response::ErasedJson;
use serde::Serialize;

use crate::context::ApplicationContext;
use crate::database::count_disk_events;

#[derive(Serialize)]
struct HealthResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub fn build_router(context: ApplicationContext) -> Router {
    // build the routes under /health
    Router::new()
        .route("/health", get(get_health))
        .with_state(context)
}

pub async fn get_health(context: State<ApplicationContext>) -> impl IntoResponse {
    match count_disk_events(&context).await {
        Ok(count) => (
            StatusCode::OK,
            ErasedJson::pretty(HealthResponse {
                status: "ok".to_string(),
                count: Some(count),
                message: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErasedJson::pretty(HealthResponse {
                status: "error".to_string(),
                count: None,
                message: Some(format!("Failed to connect to the database: {:?}", e)),
            }),
        ),
    }
}
