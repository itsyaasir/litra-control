use super::DeviceManagerState;
/// Brightness control commands for Litra devices.
///
/// This module provides comprehensive brightness management functionality including
/// brightness control in lumens and percentage, range validation, and increment/decrement
/// operations with proper device-specific limits.
use crate::error::AppError;
use tauri::State;

/// Sets the brightness of a specific Litra device using lumens.
///
/// This command sets the absolute brightness level in lumens. The value must be
/// within the device's supported range (varies by device type).
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `lumens` - The brightness value in lumens
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
/// await invoke('set_device_brightness', {
///     serialNumber: 'ABC123456',
///     lumens: 150
/// });
/// ```
#[tauri::command]
pub async fn set_device_brightness(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    lumens: u16,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

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
            message: format!(
                "Failed to set brightness for device {serial_number}: {e}"
            ),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(())
}

/// Sets the brightness of a specific Litra device using percentage.
///
/// This command sets the brightness as a percentage of the device's maximum
/// brightness. The percentage is automatically converted to the appropriate
/// lumens value for the specific device type.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `percentage` - The brightness percentage (0-100)
///
/// # Returns
///
/// Returns success or an error if the device cannot be found, the percentage
/// is invalid, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// await invoke('set_device_brightness_percentage', {
///     serialNumber: 'ABC123456',
///     percentage: 75
/// });
/// ```
#[tauri::command]
pub async fn set_device_brightness_percentage(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    percentage: u8,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Validate percentage range
    if percentage > 100 {
        return Err(AppError {
            message: format!(
                "Percentage {percentage} is invalid. Must be between 0-100"
            ),
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
            message: format!(
                "Failed to set brightness for device {serial_number}: {e}"
            ),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(())
}

/// Gets the current brightness of a specific Litra device.
///
/// This command returns comprehensive brightness information including both
/// lumens and percentage values, along with device capabilities.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to query
///
/// # Returns
///
/// Returns a BrightnessInfo structure with current brightness and device limits,
/// or an error if the device cannot be found or queried.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const brightness = await invoke<BrightnessInfo>('get_device_brightness', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn get_device_brightness(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<BrightnessInfo, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current brightness
    let current_lumens = handle.brightness_in_lumen().map_err(|e| AppError {
        message: format!(
            "Failed to get brightness for device {serial_number}: {e}"
        ),
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

/// Increases the brightness of a specific Litra device.
///
/// This command increases the brightness by the specified amount in lumens.
/// The operation will fail if the resulting brightness exceeds the device's
/// maximum brightness.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `lumens_increase` - The amount to increase brightness by (in lumens)
///
/// # Returns
///
/// Returns the new brightness level in lumens or an error if the device
/// cannot be found, the increase would exceed limits, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newBrightness = await invoke<number>('increase_device_brightness', {
///     serialNumber: 'ABC123456',
///     lumensIncrease: 25
/// });
/// ```
#[tauri::command]
pub async fn increase_device_brightness(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    lumens_increase: u16,
) -> Result<u16, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current brightness
    let current_lumens = handle.brightness_in_lumen().map_err(|e| AppError {
        message: format!(
            "Failed to get brightness for device {serial_number}: {e}"
        ),
        error_type: "BrightnessControlError".to_string(),
    })?;

    // Calculate new brightness
    let new_brightness = current_lumens.saturating_add(lumens_increase);
    let max_brightness = handle.maximum_brightness_in_lumen();

    if new_brightness > max_brightness {
        return Err(AppError {
            message: format!(
                "Brightness increase would exceed maximum. Current: {current_lumens} lm, Increase: {lumens_increase} lm, Max: {max_brightness} lm"
            ),
            error_type: "BrightnessRangeError".to_string(),
        });
    }

    handle
        .set_brightness_in_lumen(new_brightness)
        .map_err(|e| AppError {
            message: format!(
                "Failed to increase brightness for device {serial_number}: {e}"
            ),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(new_brightness)
}

/// Decreases the brightness of a specific Litra device.
///
/// This command decreases the brightness by the specified amount in lumens.
/// The operation will fail if the resulting brightness falls below the device's
/// minimum brightness.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `lumens_decrease` - The amount to decrease brightness by (in lumens)
///
/// # Returns
///
/// Returns the new brightness level in lumens or an error if the device
/// cannot be found, the decrease would fall below limits, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newBrightness = await invoke<number>('decrease_device_brightness', {
///     serialNumber: 'ABC123456',
///     lumensDecrease: 25
/// });
/// ```
#[tauri::command]
pub async fn decrease_device_brightness(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    lumens_decrease: u16,
) -> Result<u16, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current brightness
    let current_lumens = handle.brightness_in_lumen().map_err(|e| AppError {
        message: format!(
            "Failed to get brightness for device {serial_number}: {e}"
        ),
        error_type: "BrightnessControlError".to_string(),
    })?;

    // Calculate new brightness
    let new_brightness = current_lumens.saturating_sub(lumens_decrease);
    let min_brightness = handle.minimum_brightness_in_lumen();

    if new_brightness < min_brightness {
        return Err(AppError {
            message: format!(
                "Brightness decrease would fall below minimum. Current: {current_lumens} lm, Decrease: {lumens_decrease} lm, Min: {min_brightness} lm"
            ),
            error_type: "BrightnessRangeError".to_string(),
        });
    }

    handle
        .set_brightness_in_lumen(new_brightness)
        .map_err(|e| AppError {
            message: format!(
                "Failed to decrease brightness for device {serial_number}: {e}"
            ),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(new_brightness)
}

/// Increases the brightness of a specific Litra device by percentage.
///
/// This command increases the brightness by the specified percentage points.
/// For example, if current brightness is 50% and you increase by 25%, the new
/// brightness will be 75%.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `percentage_increase` - The percentage points to increase by (0-100)
///
/// # Returns
///
/// Returns the new brightness percentage or an error if the device cannot be
/// found, the increase would exceed 100%, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newPercentage = await invoke<number>('increase_device_brightness_percentage', {
///     serialNumber: 'ABC123456',
///     percentageIncrease: 10
/// });
/// ```
#[tauri::command]
pub async fn increase_device_brightness_percentage(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    percentage_increase: u8,
) -> Result<u8, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current brightness info
    let current_lumens = handle.brightness_in_lumen().map_err(|e| AppError {
        message: format!(
            "Failed to get brightness for device {serial_number}: {e}"
        ),
        error_type: "BrightnessControlError".to_string(),
    })?;

    let min_brightness = handle.minimum_brightness_in_lumen();
    let max_brightness = handle.maximum_brightness_in_lumen();
    let range = max_brightness - min_brightness;

    // Calculate current percentage
    let current_percentage = if range > 0 {
        ((current_lumens - min_brightness) as f64 / range as f64 * 100.0) as u8
    } else {
        0
    };

    // Calculate new percentage
    let new_percentage = current_percentage.saturating_add(percentage_increase);
    if new_percentage > 100 {
        return Err(AppError {
            message: format!(
                "Percentage increase would exceed 100%. Current: {current_percentage}%, Increase: {percentage_increase}%"
            ),
            error_type: "BrightnessRangeError".to_string(),
        });
    }

    // Convert to lumens and set
    let new_lumens = min_brightness + ((range as f64 * new_percentage as f64 / 100.0) as u16);

    handle
        .set_brightness_in_lumen(new_lumens)
        .map_err(|e| AppError {
            message: format!(
                "Failed to increase brightness for device {serial_number}: {e}"
            ),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(new_percentage)
}

/// Decreases the brightness of a specific Litra device by percentage.
///
/// This command decreases the brightness by the specified percentage points.
/// For example, if current brightness is 75% and you decrease by 25%, the new
/// brightness will be 50%.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `percentage_decrease` - The percentage points to decrease by (0-100)
///
/// # Returns
///
/// Returns the new brightness percentage or an error if the device cannot be
/// found, the decrease would fall below 0%, or the operation fails.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const newPercentage = await invoke<number>('decrease_device_brightness_percentage', {
///     serialNumber: 'ABC123456',
///     percentageDecrease: 10
/// });
/// ```
#[tauri::command]
pub async fn decrease_device_brightness_percentage(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    percentage_decrease: u8,
) -> Result<u8, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    // Get current brightness info
    let current_lumens = handle.brightness_in_lumen().map_err(|e| AppError {
        message: format!(
            "Failed to get brightness for device {serial_number}: {e}"
        ),
        error_type: "BrightnessControlError".to_string(),
    })?;

    let min_brightness = handle.minimum_brightness_in_lumen();
    let max_brightness = handle.maximum_brightness_in_lumen();
    let range = max_brightness - min_brightness;

    // Calculate current percentage
    let current_percentage = if range > 0 {
        ((current_lumens - min_brightness) as f64 / range as f64 * 100.0) as u8
    } else {
        0
    };

    // Calculate new percentage
    let new_percentage = current_percentage.saturating_sub(percentage_decrease);

    // Convert to lumens and set
    let new_lumens = min_brightness + ((range as f64 * new_percentage as f64 / 100.0) as u16);

    handle
        .set_brightness_in_lumen(new_lumens)
        .map_err(|e| AppError {
            message: format!(
                "Failed to decrease brightness for device {serial_number}: {e}"
            ),
            error_type: "BrightnessControlError".to_string(),
        })?;

    Ok(new_percentage)
}

/// Validates if a brightness value is within the device's supported range.
///
/// This utility function checks if a brightness value in lumens is valid
/// for the specified device.
///
/// # Arguments
///
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to check
/// * `lumens` - The brightness value to validate
///
/// # Returns
///
/// Returns true if the brightness is valid, false otherwise.
///
/// # Frontend Usage
///
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const isValid = await invoke<boolean>('validate_device_brightness', {
///     serialNumber: 'ABC123456',
///     lumens: 150
/// });
/// ```
#[tauri::command]
pub async fn validate_device_brightness(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    lumens: u16,
) -> Result<bool, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {e}"),
        error_type: "MutexError".to_string(),
    })?;

    let handle = manager.get_device_handle(&serial_number)?;

    let min_brightness = handle.minimum_brightness_in_lumen();
    let max_brightness = handle.maximum_brightness_in_lumen();

    Ok(lumens >= min_brightness && lumens <= max_brightness)
}

/// Comprehensive brightness information structure.
///
/// This structure contains all brightness-related information for a device
/// including current values and device capabilities.
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
