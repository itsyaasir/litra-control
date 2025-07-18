//! Strategy pattern implementation for device selection
//!
//! This module provides different strategies for selecting which devices
//! should be controlled by the auto-toggle functionality.

use crate::config::AutoToggleStrategy;
use crate::device::DeviceInfo;
use async_trait::async_trait;

/// Trait for device selection strategies
#[async_trait]
pub trait DeviceSelector: Send + Sync {
    /// Determine if a device should be controlled by auto-toggle
    fn should_control_device(&self, device: &DeviceInfo) -> bool;
}

/// Strategy that controls all connected devices
#[derive(Debug, Clone)]
pub struct AllDevicesStrategy;

impl AllDevicesStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DeviceSelector for AllDevicesStrategy {
    fn should_control_device(&self, device: &DeviceInfo) -> bool {
        // Control all connected devices
        device.is_connected
    }
}

/// Strategy that controls only a specific device
#[derive(Debug, Clone)]
pub struct SelectedDeviceStrategy {
    pub serial_number: String,
}

impl SelectedDeviceStrategy {
    pub fn new(serial_number: String) -> Self {
        Self { serial_number }
    }
}

#[async_trait]
impl DeviceSelector for SelectedDeviceStrategy {
    fn should_control_device(&self, device: &DeviceInfo) -> bool {
        device.is_connected && device.serial_number == self.serial_number
    }
}

/// Factory function to create device selectors from configuration
pub fn create_device_selector(strategy: &AutoToggleStrategy) -> Box<dyn DeviceSelector> {
    match strategy {
        AutoToggleStrategy::AllDevices => Box::new(AllDevicesStrategy::new()),
        AutoToggleStrategy::SelectedDevice { serial_number } => {
            Box::new(SelectedDeviceStrategy::new(serial_number.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_device(serial: &str, connected: bool) -> DeviceInfo {
        DeviceInfo {
            serial_number: serial.to_string(),
            device_type: "Litra Glow".to_string(),
            is_connected: connected,
            is_on: false,
            brightness_lumens: 100,
            min_brightness_lumens: 20,
            max_brightness_lumens: 250,
            temperature_kelvin: 5600,
            min_temperature_kelvin: 2700,
            max_temperature_kelvin: 6500,
            brightness_percentage: 0,
        }
    }

    #[test]
    fn test_all_devices_strategy() {
        let strategy = AllDevicesStrategy::new();
        let connected_device = create_test_device("ABC123", true);
        let disconnected_device = create_test_device("DEF456", false);

        assert!(strategy.should_control_device(&connected_device));
        assert!(!strategy.should_control_device(&disconnected_device));
    }

    #[test]
    fn test_selected_device_strategy() {
        let strategy = SelectedDeviceStrategy::new("ABC123".to_string());
        let target_device = create_test_device("ABC123", true);
        let other_device = create_test_device("DEF456", true);
        let disconnected_target = create_test_device("ABC123", false);

        assert!(strategy.should_control_device(&target_device));
        assert!(!strategy.should_control_device(&other_device));
        assert!(!strategy.should_control_device(&disconnected_target));
    }
}
