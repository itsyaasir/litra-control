//! Power control commands for Litra devices.
//!
//! This module provides comprehensive power management functionality including
//! power on/off operations, toggle functionality, and power state querying.

use crate::error::AppError;
use crate::AppState;
use tauri::State;

/// Toggles the power state of a specific Litra device.
#[tauri::command]
pub async fn device_power_toggle(
    state: State<'_, AppState>,
    serial_number: String,
) -> Result<bool, AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current power state
    let current_state = handle.is_on().map_err(|e| AppError {
        message: format!("Failed to get power state for device {serial_number}: {e}"),
        error_type: "PowerControlError".to_string(),
    })?;

    // Toggle to opposite state
    let new_state = !current_state;

    handle.set_on(new_state).map_err(|e| AppError {
        message: format!("Failed to toggle power for device {serial_number}: {e}"),
        error_type: "PowerControlError".to_string(),
    })?;

    Ok(new_state)
}

/// Sets the power state of a specific Litra device.
#[tauri::command]
pub async fn set_device_power(
    state: State<'_, AppState>,
    serial_number: String,
    power_on: bool,
) -> Result<(), AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    handle.set_on(power_on).map_err(|e| AppError {
        message: format!("Failed to set power state for device {serial_number}: {e}"),
        error_type: "PowerControlError".to_string(),
    })?;

    Ok(())
}
