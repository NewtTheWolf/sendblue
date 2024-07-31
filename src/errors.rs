//! Error Types
//!
//! This module provides the error types that can occur when using the Sendblue API client.

use thiserror::Error;

/// Errors that can occur when using the Sendblue API client
///
/// # Variants
///
/// * `BadRequest` - Represents a bad request error with a message
/// * `Unknown` - Represents an unknown error with a message
/// * `ValidationError` - Represents a validation error with a message
/// * `ReqwestError` - Represents an error that occurred during a request
///
/// # Examples
///
/// ```
/// use sendblue::error::SendblueError;
///
/// let error = SendblueError::BadRequest("Invalid request".into());
/// ```
#[derive(Error, Debug)]
pub enum SendblueError {
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
