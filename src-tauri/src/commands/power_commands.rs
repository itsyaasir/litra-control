/// Power control commands for Litra devices.
/// 
/// This module provides comprehensive power management functionality including
/// power on/off operations, toggle functionality, and power state querying.

use crate::error::AppError;
use super::DeviceManagerState;
use tauri::State;

/// Turns a specific Litra device on.
/// 
/// This command powers on the specified device. The device must be connected
/// and accessible for this operation to succeed.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to power on
/// 
/// # Returns
/// 
/// Returns success or an error if the device cannot be found or powered on.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// await invoke('device_power_on', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn device_power_on(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    let handle = manager.get_device_handle(&serial_number)?;
    
    handle.set_on(true).map_err(|e| AppError {
        message: format!("Failed to power on device {}: {}", serial_number, e),
        error_type: "PowerControlError".to_string(),
    })?;
    
    Ok(())
}

/// Turns a specific Litra device off.
/// 
/// This command powers off the specified device. The device must be connected
/// and accessible for this operation to succeed.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to power off
/// 
/// # Returns
/// 
/// Returns success or an error if the device cannot be found or powered off.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// await invoke('device_power_off', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn device_power_off(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    let handle = manager.get_device_handle(&serial_number)?;
    
    handle.set_on(false).map_err(|e| AppError {
        message: format!("Failed to power off device {}: {}", serial_number, e),
        error_type: "PowerControlError".to_string(),
    })?;
    
    Ok(())
}

/// Toggles the power state of a specific Litra device.
/// 
/// This command switches the device between on and off states. If the device
/// is currently on, it will be turned off, and vice versa.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to toggle
/// 
/// # Returns
/// 
/// Returns the new power state (true for on, false for off) or an error
/// if the device cannot be found or toggled.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// const newState = await invoke<boolean>('device_power_toggle', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn device_power_toggle(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<bool, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    let handle = manager.get_device_handle(&serial_number)?;
    
    // Get current power state
    let current_state = handle.is_on().map_err(|e| AppError {
        message: format!("Failed to get power state for device {}: {}", serial_number, e),
        error_type: "PowerControlError".to_string(),
    })?;
    
    // Toggle to opposite state
    let new_state = !current_state;
    
    handle.set_on(new_state).map_err(|e| AppError {
        message: format!("Failed to toggle power for device {}: {}", serial_number, e),
        error_type: "PowerControlError".to_string(),
    })?;
    
    Ok(new_state)
}

/// Gets the current power state of a specific Litra device.
/// 
/// This command queries the device to determine if it is currently powered on or off.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to query
/// 
/// # Returns
/// 
/// Returns the current power state (true for on, false for off) or an error
/// if the device cannot be found or queried.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// const isOn = await invoke<boolean>('get_device_power_status', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn get_device_power_status(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<bool, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    let handle = manager.get_device_handle(&serial_number)?;
    
    handle.is_on().map_err(|e| AppError {
        message: format!("Failed to get power state for device {}: {}", serial_number, e),
        error_type: "PowerControlError".to_string(),
    })
}

/// Sets the power state of a specific Litra device.
/// 
/// This is a more direct command that allows setting the exact power state
/// without needing to know the current state.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to control
/// * `power_on` - The desired power state (true for on, false for off)
/// 
/// # Returns
/// 
/// Returns success or an error if the device cannot be found or controlled.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// await invoke('set_device_power', {
///     serialNumber: 'ABC123456',
///     powerOn: true
/// });
/// ```
#[tauri::command]
pub async fn set_device_power(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
    power_on: bool,
) -> Result<(), AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    let handle = manager.get_device_handle(&serial_number)?;
    
    handle.set_on(power_on).map_err(|e| AppError {
        message: format!("Failed to set power state for device {}: {}", serial_number, e),
        error_type: "PowerControlError".to_string(),
    })?;
    
    Ok(())
}

/// Validates that a device is accessible for power operations.
/// 
/// This utility function can be used to check if a device is ready for
/// power control operations before attempting them.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to validate
/// 
/// # Returns
/// 
/// Returns true if the device is accessible, false otherwise.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// const isReady = await invoke<boolean>('validate_device_power_ready', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn validate_device_power_ready(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<bool, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    match manager.get_device_handle(&serial_number) {
        Ok(handle) => {
            // Try to query the power state to ensure the device is responsive
            match handle.is_on() {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        Err(_) => Ok(false),
    }
}