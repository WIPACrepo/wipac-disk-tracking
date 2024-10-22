// middleware.rs

use axum::{
    extract::ConnectInfo, extract::Request, http::StatusCode, middleware::Next, response::Response,
};
use core::net::SocketAddr;
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmptyExtra;

pub async fn log_request(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // if we can't figure out the IP address, then we don't know it
    let mut client_ip = addr.ip().to_string();

    // try to get the IP address from the request headers
    if let Some(header) = req.headers().get("x-forwarded-for") {
        if let Ok(ip) = header.to_str() {
            client_ip = ip.to_string();
        }
    }

    // log about it
    info!("{} {} {}", client_ip, req.method(), req.uri());

    // call the next middleware in the chain
    Ok(next.run(req).await)
}

pub async fn log_token(req: Request, next: Next) -> Result<Response, StatusCode> {
    // if we've got an 'Authorization' header in the request
    if let Some(header) = req.headers().get("Authorization") {
        // and we can obtain the value of that header as a string slice
        if let Ok(token) = header.to_str() {
            // log about it, and continue in the request processing stack
            debug!("Authorization: {}", token);
            return Ok(next.run(req).await);
        }
    }

    // log about the fact that no token was found
    debug!("Authorization: None");
    Ok(next.run(req).await)
}
