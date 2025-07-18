//! Camera auto-toggle related Tauri commands.
//!
//! This module contains all the commands related to camera monitoring,
//! auto-toggle configuration, and device state management.
use crate::config::AutoToggleConfig;
use crate::error::AppError;
use crate::AppState;
use tauri::State;

/// Start camera monitoring for auto-toggle functionality.
#[tauri::command]
pub async fn start_camera_monitoring(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut monitor = state.camera_monitor.lock().await;
    let config = state.config_manager.get_config();

    println!("Starting camera monitoring...");
    match monitor.start_monitoring(config.auto_toggle).await {
        Ok(()) => {
            println!("Camera monitoring started successfully");
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to start camera monitoring: {e}");
            println!("Error: {error_msg}");
            Err(AppError {
                message: error_msg,
                error_type: "CameraMonitorError".to_string(),
            })
        }
    }
}

/// Stop camera monitoring for auto-toggle functionality.
///
/// This command stops the camera monitoring system and disables
/// automatic control of Litra devices.
#[tauri::command]
pub async fn stop_camera_monitoring(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut monitor = state.camera_monitor.lock().await;

    monitor.stop_monitoring().await.map_err(|e| AppError {
        message: format!("Failed to stop camera monitoring: {e}"),
        error_type: "CameraMonitorError".to_string(),
    })
}

/// Get the current camera monitoring status.
#[tauri::command]
pub async fn is_camera_monitoring(state: State<'_, AppState>) -> Result<bool, AppError> {
    let monitor = state.camera_monitor.lock().await;

    Ok(monitor.is_monitoring())
}

/// Get the current camera device count.
#[tauri::command]
pub async fn get_camera_device_count(state: State<'_, AppState>) -> Result<usize, AppError> {
    let monitor = state.camera_monitor.lock().await;

    Ok(monitor.get_device_count())
}

/// Get the list of devices currently controlled by auto-toggle.
#[tauri::command]
pub async fn get_controlled_devices(state: State<'_, AppState>) -> Result<Vec<String>, AppError> {
    let monitor = state.camera_monitor.lock().await;

    Ok(monitor.get_controlled_devices())
}

/// Debug command to check system state
#[tauri::command]
pub async fn debug_camera_system(state: State<'_, AppState>) -> Result<String, AppError> {
    let monitor = state.camera_monitor.lock().await;

    let debug_info = format!(
        "Debug Info:\n\
        - Is monitoring: {}\n\
        - Device count: {}\n\
        - Controlled devices: {:?}\n\
        - Monitor path exists: {}\n\
        - Video devices found: {:?}",
        monitor.is_monitoring(),
        monitor.get_device_count(),
        monitor.get_controlled_devices(),
        std::path::Path::new("/dev").exists(),
        std::fs::read_dir("/dev")
            .map(|entries| entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .filter(|name| name.starts_with("video"))
                .collect::<Vec<_>>())
            .unwrap_or_else(|_| vec!["Error reading /dev".to_string()])
    );

    println!("{debug_info}");
    Ok(debug_info)
}

/// Update camera auto-toggle configuration
#[tauri::command]
pub async fn update_camera_config(
    state: State<'_, AppState>,
    config: AutoToggleConfig,
) -> Result<(), AppError> {
    println!("Received config update: {config:?}");

    // Basic validation
    if config.debounce_ms < 100 || config.debounce_ms > 30000 {
        return Err(AppError {
            message: "Debounce time must be between 100ms and 30000ms".to_string(),
            error_type: "ValidationError".to_string(),
        });
    }

    state
        .config_manager
        .update_auto_toggle_config(config)
        .map_err(|e| AppError {
            message: format!("Failed to update config: {e}"),
            error_type: "ConfigError".to_string(),
        })?;

    println!("Config saved successfully");
    Ok(())
}

/// Get current camera auto-toggle configuration
#[tauri::command]
pub async fn get_camera_config(state: State<'_, AppState>) -> Result<AutoToggleConfig, AppError> {
    let full_config = state.config_manager.get_config();

    Ok(full_config.auto_toggle)
}
