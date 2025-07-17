/// Litra Control - A modern desktop application for controlling Logitech Litra devices.
///
/// This application provides a comprehensive interface for managing Litra devices,
/// including power control, brightness adjustment, and temperature settings.
// Module declarations
mod commands;
mod device;
mod error;

// Re-export public types for easier access
pub use commands::*;
pub use device::{DeviceInfo, DeviceManager};
pub use error::{AppError, AppResult};

use std::sync::Mutex;

/// Initializes and runs the Tauri application.
///
/// This function sets up the application state, registers command handlers,
/// and starts the main event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize the device manager
    let device_manager: DeviceManagerState = match DeviceManager::new() {
        Ok(manager) => Mutex::new(manager),
        Err(e) => {
            eprintln!("Failed to initialize device manager: {e}");
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(device_manager)
        .invoke_handler(tauri::generate_handler![
            discover_devices,
            get_device_info,
            refresh_devices,
            device_power_on,
            device_power_off,
            device_power_toggle,
            get_device_power_status,
            set_device_power,
            validate_device_power_ready,
            set_device_brightness,
            set_device_brightness_percentage,
            get_device_brightness,
            increase_device_brightness,
            decrease_device_brightness,
            increase_device_brightness_percentage,
            decrease_device_brightness_percentage,
            validate_device_brightness,
            set_device_temperature,
            get_device_temperature,
            increase_device_temperature,
            decrease_device_temperature,
            validate_temperature,
            validate_device_temperature,
            round_temperature,
            set_temperature_in_kelvin,
            set_brightness_in_lumen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
