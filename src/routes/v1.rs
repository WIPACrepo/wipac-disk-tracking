// v1.rs

pub mod disks;
pub mod events;
pub mod health;

use axum::{
    routing::{get, post},
    Router,
};

use crate::context::ApplicationContext;
use crate::routes::v1::events::{
    get_events, post_closed, post_formatted, post_opened, post_sighted,
};
use crate::routes::v1::health::get_health;

pub fn build_router(context: ApplicationContext) -> Router {
    // build the routes under /api/v1
    Router::new()
        .route("/health", get(get_health))
        .route("/events/closed", post(post_closed))
        .route("/events/formatted", post(post_formatted))
        .route("/events/opened", post(post_opened))
        .route("/events/sighted", post(post_sighted))
        .route("/events/:id", get(get_events))
        .with_state(context)
}
