// main.rs

pub mod context;
pub mod database;
pub mod error;
pub mod event;
pub mod middleware;
pub mod routes;
pub mod smartctl;

use axum_keycloak_auth::{instance::KeycloakAuthInstance, instance::KeycloakConfig, Url};
use log::{error, info};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use context::build_context;
use error::{get_error_message, Result};
use routes::build_router;

#[tokio::main]
async fn main() {
    // initialize logging, configured by environment
    env_logger::init();
    // run the application, report any errors
    if let Err(e) = do_main().await {
        error!("Error: {}", get_error_message(e))
    }
}

async fn do_main() -> Result<()> {
    // set up the context of the application
    let context = build_context()
        .await
        .expect("Unable to build application context");
    // set up keycloak instance for authentication
    let keycloak_auth_instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse(&context.oauth_url).unwrap())
            .realm(String::from(&context.oauth_realm))
            .build(),
    );
    // establish our listening port
    let listener = TcpListener::bind(format!("0.0.0.0:{}", context.port))
        .await
        .unwrap_or_else(|_| panic!("Unable to listen on port {}", context.port));
    // build the application router
    let app = build_router(context, keycloak_auth_instance.into())
        .into_make_service_with_connect_info::<SocketAddr>();
    // start the disk tracking service
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    // tell the caller that there were no errors
    Ok(())
}
