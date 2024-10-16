// routes.rs

pub mod health;
pub mod token;
pub mod v1;

use axum::{http::StatusCode, middleware, response::IntoResponse, Router};
use axum_keycloak_auth::instance::KeycloakAuthInstance;
use log::error;
use std::sync::Arc;

use crate::context::ApplicationContext;
use crate::middleware::{log_request, log_token};

pub async fn handle_error<E>(err: E) -> impl IntoResponse
where
    E: std::fmt::Display,
{
    eprintln!("Error occurred: {}", err);
    error!("Error occurred: {}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "An internal error occurred.",
    )
}

pub async fn handle_error2() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "An internal error occurred",
    )
}

pub fn build_router(context: ApplicationContext, instance: Arc<KeycloakAuthInstance>) -> Router {
    Router::new()
        .with_state(context.clone())
        .merge(crate::routes::health::build_router(context.clone()))
        .merge(crate::routes::token::build_router(
            context.clone(),
            instance.clone(),
        ))
        .nest(
            "/api/v1",
            crate::routes::v1::build_router(context.clone(), instance.clone()),
        )
        .layer(middleware::from_fn(log_token))
        .layer(middleware::from_fn(log_request))
}

// async fn get_log_error() -> impl IntoResponse {
//     let simulated_error = Some("Database on fire!");

//     if let Some(err_message) = simulated_error {
//         error!("An error occurred: {}", err_message);
//         return (StatusCode::INTERNAL_SERVER_ERROR, err_message).into_response();
//     }

//     (StatusCode::OK, "Database only smoking; this is fine, everything is fine.").into_response()
// }

//---------------------------------------------------------------------------
//---------------------------------------------------------------------------
//---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }
}
