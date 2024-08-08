// error.rs

#[derive(Debug)]
pub enum ApplicationError {
    ContextError(String),
    MongoError(String),
}

impl From<std::env::VarError> for ApplicationError {
    fn from(error: std::env::VarError) -> Self {
        ApplicationError::ContextError(error.to_string())
    }
}

impl From<mongodb::error::Error> for ApplicationError {
    fn from(error: mongodb::error::Error) -> Self {
        ApplicationError::MongoError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ApplicationError>;

pub fn get_error_message(e: ApplicationError) -> String {
    match e {
        ApplicationError::ContextError(x) => {
            format!("disk-tracking: Unable to build application context: {x}")
        },
        ApplicationError::MongoError(x) => {
            format!("disk-tracking: Database error: {x}")
        },
    }
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
