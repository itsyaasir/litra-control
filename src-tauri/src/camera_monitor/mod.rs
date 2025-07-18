//! Camera Auto-Toggle Module
//!
//! This module provides automatic camera detection and lighting control functionality.
//! It monitors `/dev/video*` devices and automatically toggles Litra device power
//! based on camera activity.

pub mod monitor;
pub mod strategies;

pub use monitor::CameraMonitor;

use std::sync::Arc;
use tokio::sync::Mutex;

/// Global state type for camera monitoring
pub type CameraMonitorState = Arc<Mutex<CameraMonitor>>;

/// Result type for camera monitor operations
pub type CameraMonitorResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
