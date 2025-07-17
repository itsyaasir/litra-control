use super::DeviceManagerState;
/// Temperature control commands for Litra devices.
///
/// This module provides comprehensive temperature management functionality including
/// temperature control in Kelvin, range validation, and increment/decrement operations
/// with proper 100K step validation as required by Litra devices.
use crate::error::AppError;
use tauri::State;

/// Minimum temperature supported by all Litra devices (in Kelvin).
const MIN_TEMPERATURE: u16 = 2700;

/// Maximum temperature supported by all Litra devices (in Kelvin).
const MAX_TEMPERATURE: u16 = 6500;

/// Temperature increment step required by Litra devices (in Kelvin).
const TEMPERATURE_STEP: u16 = 100;

/// Sets the color temperature of a specific Litra device.
///
/// This command sets the absolute color temperature in Kelvin. The value must be
/// between 2700K and 6500K and must be a multiple of 100K as required by Litra devices.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `kelvin` - The temperature value in Kelvin (must be multiple of 100)
///
/// # Returns
///
/// Returns success or an error if the device cannot be found, the value is invalid,
/// or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// await invoke('set_device_temperature', {
///     serialNumber: 'ABC123456',
///     kelvin: 4500
/// });
/// ```
#[tauri::command]
pub async fn set_device_temperature(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    kelvin: u16,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

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
///
/// This command returns comprehensive temperature information including the current
/// temperature and device capabilities.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to query
///
/// # Returns
///
/// Returns a TemperatureInfo structure with current temperature and device limits,
/// or an error if the device cannot be found or queried.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const temperature = await invoke<TemperatureInfo>('get_device_temperature', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn get_device_temperature(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<TemperatureInfo, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

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

/// Increases the color temperature of a specific Litra device.
///
/// This command increases the temperature by the specified amount in Kelvin.
/// The increase amount must be a multiple of 100K and the resulting temperature
/// must not exceed the maximum supported temperature.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `kelvin_increase` - The amount to increase temperature by (must be multiple of 100K)
///
/// # Returns
///
/// Returns the new temperature level in Kelvin or an error if the device
/// cannot be found, the increase is invalid, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newTemperature = await invoke<number>('increase_device_temperature', {
///     serialNumber: 'ABC123456',
///     kelvinIncrease: 200
/// });
/// ```
#[tauri::command]
pub async fn increase_device_temperature(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    kelvin_increase: u16,
) -> Result<u16, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Validate temperature increment (must be multiple of 100K)
    if kelvin_increase % TEMPERATURE_STEP != 0 {
        return Err(AppError {
            message: format!(
                "Temperature increase {kelvin_increase} K is invalid. Must be a multiple of {TEMPERATURE_STEP} K"
            ),
            error_type: "TemperatureIncrementError".to_string(),
        });
    }

    // Get current temperature
    let current_kelvin = handle.temperature_in_kelvin().map_err(|e| AppError {
        message: format!("Failed to get temperature for device {serial_number}: {e}"),
        error_type: "TemperatureControlError".to_string(),
    })?;

    // Calculate new temperature
    let new_temperature = current_kelvin.saturating_add(kelvin_increase);
    let max_temperature = handle.maximum_temperature_in_kelvin();

    if new_temperature > max_temperature {
        return Err(AppError {
            message: format!(
                "Temperature increase would exceed maximum. Current: {current_kelvin} K, Increase: {kelvin_increase} K, Max: {max_temperature} K"
            ),
            error_type: "TemperatureRangeError".to_string(),
        });
    }

    handle
        .set_temperature_in_kelvin(new_temperature)
        .map_err(|e| AppError {
            message: format!("Failed to increase temperature for device {serial_number}: {e}"),
            error_type: "TemperatureControlError".to_string(),
        })?;

    Ok(new_temperature)
}

/// Decreases the color temperature of a specific Litra device.
///
/// This command decreases the temperature by the specified amount in Kelvin.
/// The decrease amount must be a multiple of 100K and the resulting temperature
/// must not fall below the minimum supported temperature.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `kelvin_decrease` - The amount to decrease temperature by (must be multiple of 100K)
///
/// # Returns
///
/// Returns the new temperature level in Kelvin or an error if the device
/// cannot be found, the decrease is invalid, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newTemperature = await invoke<number>('decrease_device_temperature', {
///     serialNumber: 'ABC123456',
///     kelvinDecrease: 300
/// });
/// ```
#[tauri::command]
pub async fn decrease_device_temperature(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    kelvin_decrease: u16,
) -> Result<u16, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Validate temperature decrement (must be multiple of 100K)
    if kelvin_decrease % TEMPERATURE_STEP != 0 {
        return Err(AppError {
            message: format!(
                "Temperature decrease {kelvin_decrease} K is invalid. Must be a multiple of {TEMPERATURE_STEP} K"
            ),
            error_type: "TemperatureIncrementError".to_string(),
        });
    }

    // Get current temperature
    let current_kelvin = handle.temperature_in_kelvin().map_err(|e| AppError {
        message: format!("Failed to get temperature for device {serial_number}: {e}"),
        error_type: "TemperatureControlError".to_string(),
    })?;

    // Calculate new temperature
    let new_temperature = current_kelvin.saturating_sub(kelvin_decrease);
    let min_temperature = handle.minimum_temperature_in_kelvin();

    if new_temperature < min_temperature {
        return Err(AppError {
            message: format!(
                "Temperature decrease would fall below minimum. Current: {current_kelvin} K, Decrease: {kelvin_decrease} K, Min: {min_temperature} K"
            ),
            error_type: "TemperatureRangeError".to_string(),
        });
    }

    handle
        .set_temperature_in_kelvin(new_temperature)
        .map_err(|e| AppError {
            message: format!("Failed to decrease temperature for device {serial_number}: {e}"),
            error_type: "TemperatureControlError".to_string(),
        })?;

    Ok(new_temperature)
}

/// Validates if a temperature value is valid for Litra devices.
///
/// This utility function checks if a temperature value meets all requirements:
/// - Within the supported range (2700K-6500K)
/// - Multiple of 100K as required by Litra devices
///
/// # Arguments
///
/// * `kelvin` - The temperature value to validate
///
/// # Returns
///
/// Returns true if the temperature is valid, false otherwise.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const isValid = await invoke<boolean>('validate_temperature', {
///     kelvin: 4500
/// });
/// ```
#[tauri::command]
pub async fn validate_temperature(kelvin: u16) -> Result<bool, AppError> {
    Ok((MIN_TEMPERATURE..=MAX_TEMPERATURE).contains(&kelvin) && kelvin % TEMPERATURE_STEP == 0)
}

/// Validates if a temperature value is valid for a specific device.
///
/// This function checks if a temperature value is valid for the specified device,
/// taking into account device-specific limits if they differ from the standard range.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to check
/// * `kelvin` - The temperature value to validate
///
/// # Returns
///
/// Returns true if the temperature is valid for the device, false otherwise.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const isValid = await invoke<boolean>('validate_device_temperature', {
///     serialNumber: 'ABC123456',
///     kelvin: 4500
/// });
/// ```
#[tauri::command]
pub async fn validate_device_temperature(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    kelvin: u16,
) -> Result<bool, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    let min_kelvin = handle.minimum_temperature_in_kelvin();
    let max_kelvin = handle.maximum_temperature_in_kelvin();

    Ok(kelvin >= min_kelvin && kelvin <= max_kelvin && kelvin % TEMPERATURE_STEP == 0)
}

/// Rounds a temperature value to the nearest valid increment.
///
/// This utility function rounds a temperature value to the nearest multiple of 100K
/// within the supported range, making it valid for Litra devices.
///
/// # Arguments
///
/// * `kelvin` - The temperature value to round
///
/// # Returns
///
/// Returns the rounded temperature value that is valid for Litra devices.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const rounded = await invoke<number>('round_temperature', {
///     kelvin: 4550
/// }); // Returns 4500
/// ```
#[tauri::command]
pub async fn round_temperature(kelvin: u16) -> Result<u16, AppError> {
    // Clamp to supported range
    let clamped = kelvin.max(MIN_TEMPERATURE).min(MAX_TEMPERATURE);

    // Round to nearest 100K
    let rounded = ((clamped as f64 / TEMPERATURE_STEP as f64).round() as u16) * TEMPERATURE_STEP;

    // Ensure it's still within range after rounding
    Ok(rounded.max(MIN_TEMPERATURE).min(MAX_TEMPERATURE))
}

/// Sets the temperature of a specific Litra device using Kelvin.
///
/// This command sets the absolute temperature in Kelvin.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `kelvin` - The temperature value in Kelvin
///
/// # Returns
///
/// Returns success or an error if the device cannot be found, the value is invalid,
/// or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// await invoke('set_temperature_in_kelvin', {
///     serialNumber: 'ABC123456',
///     kelvin: 4500
/// });
/// ```
#[tauri::command]
pub async fn set_temperature_in_kelvin(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    kelvin: u16,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    handle
        .set_temperature_in_kelvin(kelvin)
        .map_err(|e| AppError {
            message: format!("Failed to set temperature for device {serial_number}: {e}"),
            error_type: "TemperatureControlError".to_string(),
        })?;

    Ok(())
}

/// Comprehensive temperature information structure.
///
/// This structure contains all temperature-related information for a device
/// including current values and device capabilities.
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
