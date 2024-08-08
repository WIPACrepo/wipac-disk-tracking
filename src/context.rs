// context.rs

use std::env;

use crate::error::{ApplicationError::ContextError, Result};

#[derive(Debug)]
pub struct ApplicationContext {
    pub mongo_database: String,
    pub mongo_host: String,
    pub mongo_password: String,
    pub mongo_port: String,
    pub mongo_user: String,
}

impl ApplicationContext {
    pub fn get_mongo_url(&self) -> String {
        format!("mongodb://{}:{}@{}:{}/", self.mongo_user, self.mongo_password, self.mongo_host, self.mongo_port)
    }
}

pub fn build_context() -> Result<ApplicationContext> {
    // read database parameters from the environment
    let mongo_host = env::var("MONGODB_HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let mongo_port = env::var("MONGODB_PORT").unwrap_or_else(|_| "27017".into());
    let mongo_user = env::var("MONGODB_USERNAME").unwrap_or_else(|_| "disk_tracking".into());
    let mongo_database = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "disk_tracking".into());
    let mongo_password = match env::var("MONGODB_PASSWORD") {
        Err(_) => return Err(ContextError("environment variable MONGODB_PASSWORD not defined".to_string())),
        Ok(x) => x,
    };

    // return the application context to the caller
    Ok(ApplicationContext {
        mongo_database,
        mongo_host,
        mongo_password,
        mongo_port,
        mongo_user,
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
