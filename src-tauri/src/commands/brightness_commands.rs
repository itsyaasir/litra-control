//! Brightness control commands for Litra devices.
//!
//! This module provides comprehensive brightness management functionality including
//! brightness control in lumens and percentage, range validation, and increment/decrement
//! operations with proper device-specific limits.
use crate::error::AppError;
use crate::AppState;
use tauri::State;

/// Comprehensive brightness information structure.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BrightnessInfo {
    /// Current brightness level in lumens
    pub current_lumens: u16,
    /// Current brightness as percentage (0-100)
    pub current_percentage: u8,
    /// Minimum brightness supported by the device
    pub min_lumens: u16,
    /// Maximum brightness supported by the device
    pub max_lumens: u16,
}

/// Sets the brightness of a specific Litra device using lumens.
#[tauri::command]
pub async fn set_device_brightness(
    state: State<'_, AppState>,
    serial_number: String,
    lumens: u16,
) -> Result<(), AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    // Validate brightness range
    let min_brightness = handle.minimum_brightness_in_lumen();
    let max_brightness = handle.maximum_brightness_in_lumen();

    if lumens < min_brightness || lumens > max_brightness {
        return Err(AppError {
            message: format!(
                "Brightness {lumens} lm is out of range. Device {serial_number} supports {min_brightness}-{max_brightness} lm"
            ),
            error_type: "BrightnessRangeError".to_string(),
        });
    }

    handle
        .set_brightness_in_lumen(lumens)
        .map_err(|e| AppError {
            message: format!("Failed to set brightness for device {serial_number}: {e}"),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(())
}

/// Sets the brightness of a specific Litra device using percentage.
#[tauri::command]
pub async fn set_device_brightness_percentage(
    state: State<'_, AppState>,
    serial_number: String,
    percentage: u8,
) -> Result<(), AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    // Validate percentage range
    if percentage > 100 {
        return Err(AppError {
            message: format!("Percentage {percentage} is invalid. Must be between 0-100"),
            error_type: "BrightnessRangeError".to_string(),
        });
    }

    // Convert percentage to lumens
    let min_brightness = handle.minimum_brightness_in_lumen();
    let max_brightness = handle.maximum_brightness_in_lumen();
    let range = max_brightness - min_brightness;
    let lumens = min_brightness + ((range as f64 * percentage as f64 / 100.0) as u16);

    handle
        .set_brightness_in_lumen(lumens)
        .map_err(|e| AppError {
            message: format!("Failed to set brightness for device {serial_number}: {e}"),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(())
}

/// Gets the current brightness of a specific Litra device.
#[tauri::command]
pub async fn get_device_brightness(
    state: State<'_, AppState>,
    serial_number: String,
) -> Result<BrightnessInfo, AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current brightness
    let current_lumens = handle.brightness_in_lumen().map_err(|e| AppError {
        message: format!("Failed to get brightness for device {serial_number}: {e}"),
        error_type: "BrightnessControlError".to_string(),
    })?;

    // Get device limits
    let min_brightness = handle.minimum_brightness_in_lumen();
    let max_brightness = handle.maximum_brightness_in_lumen();

    // Calculate percentage
    let range = max_brightness - min_brightness;
    let percentage = if range > 0 {
        ((current_lumens - min_brightness) as f64 / range as f64 * 100.0) as u8
    } else {
        0
    };

    Ok(BrightnessInfo {
        current_lumens,
        current_percentage: percentage,
        min_lumens: min_brightness,
        max_lumens: max_brightness,
    })
}

/// Sets the brightness of a specific Litra device using lumens.
#[tauri::command]
pub async fn set_brightness_in_lumen(
    state: State<'_, AppState>,
    serial_number: String,
    lumens: u16,
) -> Result<(), AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    handle
        .set_brightness_in_lumen(lumens)
        .map_err(|e| AppError {
            message: format!("Failed to set brightness for device {serial_number}: {e}"),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(())
}
