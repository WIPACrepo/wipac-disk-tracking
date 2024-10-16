// v1.rs

pub mod disks;
pub mod events;

use axum::{
    routing::{get, post},
    Router,
};
use axum_keycloak_auth::{
    instance::KeycloakAuthInstance, layer::KeycloakAuthLayer, PassthroughMode,
};
use std::sync::Arc;

use crate::context::ApplicationContext;
use crate::middleware::EmptyExtra;
use crate::routes::v1::events::{
    get_events, post_closed, post_formatted, post_opened, post_sighted,
};

pub fn build_router(context: ApplicationContext, instance: Arc<KeycloakAuthInstance>) -> Router {
    // save the audience
    let oauth_audience = context.oauth_audience.clone();

    // build the routes under /api/v1
    Router::new()
        .route("/events/closed", post(post_closed))
        .route("/events/formatted", post(post_formatted))
        .route("/events/opened", post(post_opened))
        .route("/events/sighted", post(post_sighted))
        .route("/events/:id", get(get_events))
        .with_state(context)
        .layer(
            KeycloakAuthLayer::<String, EmptyExtra>::builder()
                .instance(instance)
                .passthrough_mode(PassthroughMode::Block)
                .persist_raw_claims(false)
                .expected_audiences(vec![oauth_audience])
                .required_roles(vec![String::from("system")])
                .build(),
        )
}
