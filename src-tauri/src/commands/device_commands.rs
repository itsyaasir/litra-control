/// Device-related Tauri commands.
/// 
/// This module contains all the commands related to device discovery,
/// state management, and control operations.

use crate::device::{DeviceInfo, DeviceManager};
use crate::error::AppError;
use std::sync::Mutex;
use tauri::State;

/// Global device manager state shared across all commands.
/// 
/// This is wrapped in a Mutex to ensure thread-safe access from multiple
/// frontend requests.
pub type DeviceManagerState = Mutex<DeviceManager>;

/// Discovers all connected Litra devices.
/// 
/// This command scans for all connected Litra devices and returns their
/// current state and capabilities. It's safe to call this command frequently
/// as it will refresh the device list automatically.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// 
/// # Returns
/// 
/// Returns a vector of DeviceInfo structures for all discovered devices,
/// or an error if device discovery fails.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// const devices = await invoke<DeviceInfo[]>('discover_devices');
/// ```
#[tauri::command]
pub async fn discover_devices(
    device_manager: State<'_, DeviceManagerState>,
) -> Result<Vec<DeviceInfo>, AppError> {
    let mut manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    // Refresh the device list to ensure we have the latest information
    manager.refresh_devices()?;
    
    // Get all connected devices
    manager.get_all_devices()
}

/// Gets detailed information for a specific device.
/// 
/// This command retrieves comprehensive information about a single device
/// including its current state, capabilities, and configuration limits.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// * `serial_number` - The serial number of the device to query
/// 
/// # Returns
/// 
/// Returns detailed DeviceInfo for the specified device or an error if
/// the device is not found or cannot be accessed.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// const device = await invoke<DeviceInfo>('get_device_info', {
///     serialNumber: 'ABC123456'
/// });
/// ```
#[tauri::command]
pub async fn get_device_info(
    device_manager: State<'_, DeviceManagerState>,
    serial_number: String,
) -> Result<DeviceInfo, AppError> {
    let manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    manager.get_device_info(&serial_number)
}

/// Refreshes the device list.
/// 
/// This command forces a refresh of the internal device list, which is useful
/// when devices are connected or disconnected while the application is running.
/// 
/// # Arguments
/// 
/// * `device_manager` - The global device manager state
/// 
/// # Returns
/// 
/// Returns success or an error if the refresh operation fails.
/// 
/// # Frontend Usage
/// 
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// 
/// await invoke('refresh_devices');
/// ```
#[tauri::command]
pub async fn refresh_devices(
    device_manager: State<'_, DeviceManagerState>,
) -> Result<(), AppError> {
    let mut manager = device_manager.lock().map_err(|e| AppError {
        message: format!("Failed to acquire device manager lock: {}", e),
        error_type: "MutexError".to_string(),
    })?;
    
    manager.refresh_devices()
}