pub mod brightness_commands;
/// Tauri commands module for the Litra Control application.
///
/// This module contains all the Tauri commands that can be invoked from the frontend.
/// Each command is properly documented and handles errors gracefully.
pub mod device_commands;
pub mod power_commands;
pub mod temperature_commands;

pub use brightness_commands::*;
pub use device_commands::*;
pub use power_commands::*;
pub use temperature_commands::*;
