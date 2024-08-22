// context.rs

use mongodb::{options::ClientOptions, Client};
use std::env;
use std::time::Duration;

use crate::error::{ApplicationError::ContextError, Result};

#[derive(Clone, Debug)]
pub struct ApplicationContext {
    pub mongo_client: Client,
    pub mongo_database: String,
    pub mongo_host: String,
    pub mongo_password: String,
    pub mongo_port: String,
    pub mongo_user: String,
    pub port: u16,
}

pub async fn build_context() -> Result<ApplicationContext> {
    // read database connection parameters from the environment
    let mongo_user = env::var("MONGODB_USERNAME").unwrap_or_else(|_| "disk_tracking".into());
    let mongo_password = match env::var("MONGODB_PASSWORD") {
        Err(_) => {
            return Err(ContextError(
                "environment variable MONGODB_PASSWORD not defined".to_string(),
            ))
        }
        Ok(x) => x,
    };
    let mongo_host = env::var("MONGODB_HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let mongo_port = env::var("MONGODB_PORT_NUMBER").unwrap_or_else(|_| "27017".into());
    let mongo_database = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "disk_tracking".into());

    // read application port from the environment
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let port = match port_str.parse::<u16>() {
        Err(_) => {
            return Err(ContextError(format!(
                "environment variable PORT contains invalid value: {}",
                port_str
            )))
        }
        Ok(x) => x,
    };

    // read database connection timeout from the environment
    let mongodb_timeout_secs_str = env::var("MONGODB_TIMEOUT_SECS").unwrap_or_else(|_| "30".into());
    let mongodb_timeout_secs = match mongodb_timeout_secs_str.parse::<u64>() {
        Err(_) => {
            return Err(ContextError(format!(
                "environment variable MONGODB_TIMEOUT_SECS contains invalid value: {}",
                mongodb_timeout_secs_str
            )))
        }
        Ok(x) => x,
    };

    // build the database client
    let conn_str = format!(
        "mongodb://{}:{}@{}:{}/{}",
        mongo_user, mongo_password, mongo_host, mongo_port, mongo_database
    );
    let mut client_options = ClientOptions::parse(conn_str).await?;
    client_options.connect_timeout = Some(Duration::from_secs(mongodb_timeout_secs));
    client_options.server_selection_timeout = Some(Duration::from_secs(mongodb_timeout_secs));
    let mongo_client = Client::with_options(client_options)?;

    // return the application context to the caller
    Ok(ApplicationContext {
        mongo_client,
        mongo_database,
        mongo_host,
        mongo_password,
        mongo_port,
        mongo_user,
        port,
    })
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
