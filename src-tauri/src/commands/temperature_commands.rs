//! Temperature control commands for Litra devices.
//!
//! This module provides comprehensive temperature management functionality including
//! temperature control in Kelvin, range validation, and increment/decrement operations
//! with proper 100K step validation as required by Litra devices.

use crate::error::AppError;
use crate::AppState;
use tauri::State;

/// Minimum temperature supported by all Litra devices (in Kelvin).
const MIN_TEMPERATURE: u16 = 2700;

/// Maximum temperature supported by all Litra devices (in Kelvin).
const MAX_TEMPERATURE: u16 = 6500;

/// Temperature increment step required by Litra devices (in Kelvin).
const TEMPERATURE_STEP: u16 = 100;

/// Comprehensive temperature information structure.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TemperatureInfo {
    /// Current temperature in Kelvin
    pub current_kelvin: u16,
    /// Current temperature as percentage of range (0-100)
    pub current_percentage: u8,
    /// Minimum temperature supported by the device
    pub min_kelvin: u16,
    /// Maximum temperature supported by the device
    pub max_kelvin: u16,

    /// Temperature step increment (always 100K for Litra devices)
    pub step_kelvin: u16,
}

/// Sets the color temperature of a specific Litra device.
///
/// This command sets the absolute color temperature in Kelvin. The value must be
/// between 2700K and 6500K and must be a multiple of 100K as required by Litra devices.
#[tauri::command]
pub async fn set_device_temperature(
    state: State<'_, AppState>,
    serial_number: String,
    kelvin: u16,
) -> Result<(), AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    // Validate temperature range
    if !(MIN_TEMPERATURE..=MAX_TEMPERATURE).contains(&kelvin) {
        return Err(AppError {
            message: format!(
                "Temperature {kelvin} K is out of range. Supported range: {MIN_TEMPERATURE}-{MAX_TEMPERATURE} K"
            ),
            error_type: "TemperatureRangeError".to_string(),
        });
    }

    // Validate temperature increment (must be multiple of 100K)
    if kelvin % TEMPERATURE_STEP != 0 {
        return Err(AppError {
            message: format!(
                "Temperature {kelvin} K is invalid. Must be a multiple of {TEMPERATURE_STEP} K"
            ),
            error_type: "TemperatureIncrementError".to_string(),
        });
    }

    handle
        .set_temperature_in_kelvin(kelvin)
        .map_err(|e| AppError {
            message: format!("Failed to set temperature for device {serial_number}: {e}"),
            error_type: "TemperatureControlError".to_string(),
        })?;

    Ok(())
}

/// Gets the current color temperature of a specific Litra device.
#[tauri::command]
pub async fn get_device_temperature(
    state: State<'_, AppState>,
    serial_number: String,
) -> Result<TemperatureInfo, AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current temperature
    let current_kelvin = handle.temperature_in_kelvin().map_err(|e| AppError {
        message: format!("Failed to get temperature for device {serial_number}: {e}"),
        error_type: "TemperatureControlError".to_string(),
    })?;

    // Get device limits
    let min_kelvin = handle.minimum_temperature_in_kelvin();
    let max_kelvin = handle.maximum_temperature_in_kelvin();

    // Calculate temperature percentage for UI representation
    let range = max_kelvin - min_kelvin;
    let percentage = if range > 0 {
        ((current_kelvin - min_kelvin) as f64 / range as f64 * 100.0) as u8
    } else {
        0
    };

    Ok(TemperatureInfo {
        current_kelvin,
        current_percentage: percentage,
        min_kelvin,
        max_kelvin,
        step_kelvin: TEMPERATURE_STEP,
    })
}

/// Sets the temperature of a specific Litra device using Kelvin.
///
/// This command sets the absolute temperature in Kelvin.
#[tauri::command]
pub async fn set_temperature_in_kelvin(
    state: State<'_, AppState>,
    serial_number: String,
    kelvin: u16,
) -> Result<(), AppError> {
    let manager = state.device_manager.lock().await;

    let handle = manager.get_device_handle(&serial_number)?;

    handle
        .set_temperature_in_kelvin(kelvin)
        .map_err(|e| AppError {
            message: format!("Failed to set temperature for device {serial_number}: {e}"),
            error_type: "TemperatureControlError".to_string(),
        })?;

    Ok(())
}
