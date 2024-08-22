// routes.rs

pub mod v1;

use axum::{middleware, Router};

use crate::context::ApplicationContext;
use crate::middleware::log_request;

pub fn build_router(context: ApplicationContext) -> Router {
    Router::new()
        .with_state(context.clone())
        .nest("/api/v1", v1::build_router(context.clone()))
        .layer(middleware::from_fn(log_request))
}

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
