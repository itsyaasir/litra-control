use crate::device::{DeviceInfo, DeviceManager};
use crate::error::AppError;
use crate::AppState;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Global device manager state shared across all commands.
///
/// This is wrapped in a Mutex to ensure thread-safe access from multiple
/// frontend requests.
pub type DeviceManagerState = Arc<Mutex<DeviceManager>>;

/// Discovers all connected Litra devices.
#[tauri::command]
pub async fn discover_devices(state: State<'_, AppState>) -> Result<Vec<DeviceInfo>, AppError> {
    let mut manager = state.device_manager.lock().await;

    // Refresh the device list to ensure we have the latest information
    manager.refresh_devices()?;

    // Get all connected devices
    manager.get_all_devices()
}

/// Gets detailed information for a specific device.
#[tauri::command]
pub async fn get_device_info(
    state: State<'_, AppState>,
    serial_number: String,
) -> Result<DeviceInfo, AppError> {
    let manager = state.device_manager.lock().await;

    manager.get_device_info(&serial_number)
}

/// Refreshes the device list.
#[tauri::command]
pub async fn refresh_devices(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut manager = state.device_manager.lock().await;

    manager.refresh_devices()
}
