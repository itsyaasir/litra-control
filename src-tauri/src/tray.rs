//! System tray management for Litra Control
//!
//! This module handles all system tray functionality including menu creation,
//! event handling, and device power control from the tray menu.

use crate::{refresh_devices, AppState, DeviceInfo};
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, Runtime};

/// Initialize the system tray with menu and event handlers
pub async fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    // Get device manager to check for devices
    let app_state = app.state::<AppState>();
    let device_manager = app_state.device_manager.clone();

    let devices = device_manager
        .lock()
        .await
        .get_all_devices()
        .unwrap_or_default();

    // Build the tray menu
    let menu = build_tray_menu(app, &devices)?;

    // Create system tray
    let _tray = TrayIconBuilder::new()
        .tooltip("Litra Control")
        .menu(&menu)
        .on_menu_event(move |app, event| {
            tauri::async_runtime::block_on(async {
                handle_tray_menu_event(app, event.id.as_ref()).await;
            });
        })
        .icon(
            app.default_window_icon()
                .expect("Failed to get default window icon")
                .clone(),
        )
        .build(app)?;

    Ok(())
}

/// Build the tray menu with device-specific options
fn build_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
    devices: &[DeviceInfo],
) -> Result<tauri::menu::Menu<R>, Box<dyn std::error::Error>> {
    let show_hide = MenuItemBuilder::with_id("show_hide", "Show/Hide").build(app)?;
    let separator1 = tauri::menu::PredefinedMenuItem::separator(app)?;

    // Create power submenu with device options
    let mut power_submenu = SubmenuBuilder::new(app, "Toggle Power");

    // Add "All Devices" option
    let all_devices = MenuItemBuilder::with_id("power_all", "All Devices").build(app)?;
    power_submenu = power_submenu.item(&all_devices);

    // Add separator if we have devices
    if !devices.is_empty() {
        power_submenu = power_submenu.separator();

        // Add individual devices
        for device in devices {
            let device_item = MenuItemBuilder::with_id(
                format!("power_{}", device.serial_number),
                format!("{} ({})", device.device_type, device.serial_number),
            )
            .build(app)?;
            power_submenu = power_submenu.item(&device_item);
        }
    }

    let power_menu = power_submenu.build()?;

    let refresh_devices =
        MenuItemBuilder::with_id("refresh_devices", "Refresh Devices").build(app)?;

    let separator2 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[
            &show_hide,
            &separator1,
            &power_menu,
            &refresh_devices,
            &separator2,
            &quit,
        ])
        .build()?;

    Ok(menu)
}

/// Handle tray menu events
async fn handle_tray_menu_event<R: Runtime>(app: &AppHandle<R>, event_id: &str) {
    match event_id {
        "quit" => {
            app.exit(0);
        }
        "show_hide" => {
            toggle_window_visibility(app);
        }
        "power_all" => {
            toggle_all_devices(app).await;
        }
        "refresh_devices" => {
            refresh_devices(app.state::<AppState>())
                .await
                .expect("Failed to refresh devices");
            let _ = app.emit("device-refresh", ());
        }
        event_id if event_id.starts_with("power_") => {
            let serial_number = event_id.strip_prefix("power_").unwrap();
            toggle_device_power(app, serial_number).await;
        }
        _ => {}
    }
}

/// Toggle power for all connected devices
async fn toggle_all_devices<R: Runtime>(app: &AppHandle<R>) {
    let app_state = app.state::<AppState>();
    let dm = app_state.device_manager.lock().await;
    if let Ok(devices) = dm.get_all_devices() {
        for device in devices {
            // Get device handle and toggle power
            if let Ok(handle) = dm.get_device_handle(&device.serial_number) {
                let _ = handle.set_on(!device.is_on);
            }
        }
    }

    let _ = app.emit("device-refresh", ());
}

/// Toggle power for a specific device
async fn toggle_device_power<R: Runtime>(app: &AppHandle<R>, serial_number: &str) {
    let app_state = app.state::<AppState>();
    let dm = app_state.device_manager.lock().await;

    // Get device handle
    if let Ok(handle) = dm.get_device_handle(serial_number) {
        if let Ok(is_on) = handle.is_on() {
            let _ = handle.set_on(!is_on);
        }
    }

    let _ = app.emit("device-refresh", ());
}

/// Toggle window visibility
fn toggle_window_visibility<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}
