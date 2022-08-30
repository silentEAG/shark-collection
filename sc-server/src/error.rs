use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    DataBase(#[from] sqlx::Error),
    #[error("Error with message: `{0}`")]
    OtherWithMessage(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::IO(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            ServerError::DataBase(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            _ => (StatusCode::BAD_REQUEST, "Oops! Unkown Error!".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
