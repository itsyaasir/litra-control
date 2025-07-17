/// Error handling module for the Litra Control application.
///
/// This module provides comprehensive error types and utilities for handling
/// device communication errors, I/O errors, and application-specific errors.
use litra::DeviceError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Application-specific error type that can be serialized for frontend communication.
///
/// This error type wraps various underlying errors and provides a consistent
/// interface for error handling throughout the application.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    /// Human-readable error message
    pub message: String,
    /// Categorized error type for programmatic handling
    pub error_type: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl std::error::Error for AppError {}

/// Converts Litra device errors to application errors.
impl From<DeviceError> for AppError {
    fn from(error: DeviceError) -> Self {
        AppError {
            message: error.to_string(),
            error_type: "DeviceError".to_string(),
        }
    }
}

/// Converts I/O errors to application errors.
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError {
            message: error.to_string(),
            error_type: "IoError".to_string(),
        }
    }
}

/// Convenience type alias for Results that may contain application errors.
pub type AppResult<T> = Result<T, AppError>;

/// Creates a custom application error with the specified message and type.
pub fn app_error(message: &str, error_type: &str) -> AppError {
    AppError {
        message: message.to_string(),
        error_type: error_type.to_string(),
    }
}

/// Creates a device not found error.
pub fn device_not_found_error(serial_number: &str) -> AppError {
    app_error(
        &format!("Device with serial number {serial_number} not found"),
        "DeviceNotFound",
    )
}

/// Creates a device communication error.
pub fn device_communication_error(message: &str) -> AppError {
    app_error(message, "DeviceCommunicationError")
}
