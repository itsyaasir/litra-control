//! Litra Control - A modern desktop application for controlling Logitech Litra devices.
//!
//! This application provides a comprehensive interface for managing Litra devices,
//! including power control, brightness adjustment, and temperature settings.

mod camera_monitor;
mod cli;
mod commands;
pub mod config;
mod device;
mod error;
mod tray;

pub use commands::*;
pub use device::{DeviceInfo, DeviceManager};
pub use error::{AppError, AppResult};
use tauri::Manager;
use tokio::sync::Mutex;

use std::sync::Arc;

use crate::camera_monitor::{CameraMonitor, CameraMonitorState};
use crate::config::ConfigManager;

/// The application state.
///
/// This struct contains the application state, including the device manager,
/// config manager, and camera monitor.
pub struct AppState {
    /// The device manager.
    pub device_manager: DeviceManagerState,
    /// The config manager.
    pub config_manager: Arc<ConfigManager>,
    /// The camera monitor.
    pub camera_monitor: CameraMonitorState,
}

/// The application state constructor.
///
/// This function creates a new application state.
impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let device_manager = Arc::new(Mutex::new(
            DeviceManager::new().expect("Failed to initialize device manager"),
        ));

        let config_manager = ConfigManager::new().expect("Failed to initialize config manager");

        Self {
            device_manager: device_manager.clone(),
            config_manager: Arc::new(config_manager),
            camera_monitor: Arc::new(Mutex::new(CameraMonitor::new(device_manager))),
        }
    }
}

/// Initializes and runs the Tauri application.
///
/// This function sets up the application state, registers command handlers,
/// and starts the main event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            // Setup system tray
            tauri::async_runtime::block_on(async {
                tray::setup_tray(app.app_handle())
                    .await
                    .expect("Failed to setup tray");
            });

            // Handle CLI args
            if let Err(e) = crate::cli::handle_cli_args(app) {
                eprintln!("Error handling CLI args: {e}");
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            discover_devices,
            get_device_info,
            refresh_devices,
            device_power_toggle,
            set_device_power,
            set_device_brightness,
            set_device_brightness_percentage,
            get_device_brightness,
            set_device_temperature,
            get_device_temperature,
            set_temperature_in_kelvin,
            set_brightness_in_lumen,
            start_camera_monitoring,
            stop_camera_monitoring,
            is_camera_monitoring,
            get_camera_device_count,
            get_controlled_devices,
            debug_camera_system,
            update_camera_config,
            get_camera_config,
        ])
        .on_window_event(|window, event| {
            // Handle window close to minimize to tray instead
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
