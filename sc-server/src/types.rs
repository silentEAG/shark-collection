use crate::error::ServerError;

pub type Result<T> = std::result::Result<T, ServerError>;
