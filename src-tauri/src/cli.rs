//! CLI argument handling for Litra Control.

use tauri::{App, Manager};
use tauri_plugin_cli::{ArgData, CliExt};

/// Handles CLI arguments for the application.
///
/// This function processes command-line arguments passed to the application,
/// particularly handling the `--minimized` flag to start the app minimized to tray.
pub fn handle_cli_args(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    match app.cli().matches() {
        Ok(matches) => {
            // Check if the app should start minimized
            if matches
                .args
                .get("minimized")
                .unwrap_or(&ArgData::default())
                .value
                .as_bool()
                .unwrap_or(false)
            {
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        window.hide()?;
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to get CLI args: {e}");
            Err(e.into())
        }
    }
}
