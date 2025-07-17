/// Device management module for Litra Control application.
/// 
/// This module provides comprehensive device management functionality including
/// device discovery, state management, and communication with Litra devices.

pub mod manager;
pub mod types;

pub use manager::DeviceManager;
pub use types::DeviceInfo;