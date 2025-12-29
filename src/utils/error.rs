use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Task not found")]
    TaskNotFound,
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Invalid input data")]
    ValidationError(#[from] ValidationErrors),
    
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("JWT error")] /*#[from] attribute Automatically implements From<jsonwebtoken::errors::Error> for your enum. This lets you use the ? operator on jsonwebtoken functions and convert their errors into your custom error type automatically. */
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AppError::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found"),
            AppError::Unauthorized => (StatusCode::FORBIDDEN, "Unauthorized access"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation failed"),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::JwtError(_) => (StatusCode::UNAUTHORIZED, "Token error"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
}