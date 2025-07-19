//! Configuration management for camera auto-toggle
//!
//! This module handles all configuration persistence using the `confy` crate
//! with TOML format. It provides hot-reload support and type-safe configuration.

use chrono::{DateTime, Utc};
use confy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "config";

/// Application name
pub const APP_NAME: &str = "litra-control";

/// Main configuration structure for the entire application
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LitraConfig {
    /// Auto-toggle specific configuration
    pub auto_toggle: AutoToggleConfig,
    /// Device state tracking
    pub device_states: DeviceStates,
}

/// Configuration for the camera auto-toggle functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoToggleConfig {
    /// Whether auto-toggle is enabled
    pub enabled: bool,
    /// Device selection strategy
    pub strategy: AutoToggleStrategy,
    /// Debounce delay in milliseconds
    pub debounce_ms: u64,
}

impl Default for AutoToggleConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: AutoToggleStrategy::default(),
            debounce_ms: 3000,
        }
    }
}

/// Device selection strategies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AutoToggleStrategy {
    /// Control all connected devices
    #[default]
    AllDevices,
    /// Control only the currently selected device
    SelectedDevice {
        #[serde(rename = "serialNumber")]
        serial_number: String,
    },
}

/// Device state tracking for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceStates {
    /// List of devices currently controlled by auto-toggle
    pub auto_toggle_controlled: Vec<String>,
    /// Timestamp of last auto-toggle activation
    pub last_auto_toggle_time: Option<DateTime<Utc>>,
}

/// Configuration manager with hot-reload support
pub struct ConfigManager {
    config: Arc<RwLock<LitraConfig>>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load initial configuration
        let config = Arc::new(RwLock::new(confy::load(APP_NAME, Some(CONFIG_FILE_NAME))?));

        Ok(Self { config })
    }

    /// Get the current configuration
    pub fn get_config(&self) -> LitraConfig {
        self.config.read().expect("Failed to read config").clone()
    }

    /// Update the configuration and save to disk
    pub fn update_config(&self, new_config: LitraConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Save to disk
        confy::store(APP_NAME, Some(CONFIG_FILE_NAME), &new_config)?;

        // Update in-memory config
        *self.config.write().expect("Failed to write config") = new_config;

        Ok(())
    }

    /// Update only the auto-toggle configuration
    pub fn update_auto_toggle_config(
        &self,
        auto_toggle: AutoToggleConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.get_config();
        config.auto_toggle = auto_toggle;
        self.update_config(config)
    }

    /// Update only the device states
    pub fn update_device_states(
        &self,
        device_states: DeviceStates,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.get_config();
        config.device_states = device_states;
        self.update_config(config)
    }

    /// Get the configuration file path
    pub fn get_config_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(confy::get_configuration_file_path(
            APP_NAME,
            Some(CONFIG_FILE_NAME),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = LitraConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: LitraConfig = toml::from_str(&serialized).unwrap();

        assert_eq!(config.auto_toggle.enabled, deserialized.auto_toggle.enabled);
        assert_eq!(
            config.auto_toggle.debounce_ms,
            deserialized.auto_toggle.debounce_ms
        );
    }

    #[test]
    fn test_strategy_serialization() {
        let strategies = vec![
            AutoToggleStrategy::AllDevices,
            AutoToggleStrategy::SelectedDevice {
                serial_number: "ABC123".to_string(),
            },
        ];

        for strategy in strategies {
            let serialized = toml::to_string(&strategy).unwrap();
            let deserialized: AutoToggleStrategy = toml::from_str(&serialized).unwrap();

            match (&strategy, &deserialized) {
                (AutoToggleStrategy::AllDevices, AutoToggleStrategy::AllDevices) => {}
                (
                    AutoToggleStrategy::SelectedDevice { serial_number: a },
                    AutoToggleStrategy::SelectedDevice { serial_number: b },
                ) => {
                    assert_eq!(a, b);
                }
                _ => panic!("Strategy serialization failed"),
            }
        }
    }
}
