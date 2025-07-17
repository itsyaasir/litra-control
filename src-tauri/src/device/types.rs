/// Type definitions for device information and state.

use serde::{Deserialize, Serialize};

/// Complete device information structure for frontend communication.
/// 
/// This structure contains all the necessary information about a Litra device
/// including its current state, capabilities, and configuration limits.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    /// Device serial number (unique identifier)
    pub serial_number: String,
    
    /// Device model name (e.g., "Litra Glow", "Litra Beam", "Litra Beam LX")
    pub device_type: String,
    
    /// Current connection status
    pub is_connected: bool,
    
    /// Current power state (on/off)
    pub is_on: bool,
    
    /// Current brightness level in lumens
    pub brightness_lumens: u16,
    
    /// Current brightness as percentage (0-100)
    pub brightness_percentage: u8,
    
    /// Current color temperature in Kelvin
    pub temperature_kelvin: u16,
    
    /// Minimum brightness supported by this device
    pub min_brightness_lumens: u16,
    
    /// Maximum brightness supported by this device
    pub max_brightness_lumens: u16,
    
    /// Minimum color temperature supported (typically 2700K)
    pub min_temperature_kelvin: u16,
    
    /// Maximum color temperature supported (typically 6500K)
    pub max_temperature_kelvin: u16,
}

impl DeviceInfo {
    /// Creates a new DeviceInfo with default values for a disconnected device.
    pub fn disconnected(serial_number: String, device_type: String) -> Self {
        Self {
            serial_number,
            device_type,
            is_connected: false,
            is_on: false,
            brightness_lumens: 0,
            brightness_percentage: 0,
            temperature_kelvin: 2700,
            min_brightness_lumens: 20,
            max_brightness_lumens: 250,
            min_temperature_kelvin: 2700,
            max_temperature_kelvin: 6500,
        }
    }
    
    /// Calculates brightness percentage from lumens based on device limits.
    pub fn calculate_brightness_percentage(&self) -> u8 {
        if self.max_brightness_lumens > self.min_brightness_lumens {
            let range = self.max_brightness_lumens - self.min_brightness_lumens;
            let relative_brightness = self.brightness_lumens.saturating_sub(self.min_brightness_lumens);
            ((relative_brightness as f64 / range as f64) * 100.0) as u8
        } else {
            0
        }
    }
    
    /// Calculates lumens from percentage based on device limits.
    pub fn calculate_lumens_from_percentage(&self, percentage: u8) -> u16 {
        let percentage = percentage.min(100);
        let range = self.max_brightness_lumens - self.min_brightness_lumens;
        self.min_brightness_lumens + ((range as f64 * percentage as f64 / 100.0) as u16)
    }
    
    /// Validates if the given brightness in lumens is within device limits.
    pub fn is_valid_brightness(&self, lumens: u16) -> bool {
        lumens >= self.min_brightness_lumens && lumens <= self.max_brightness_lumens
    }
    
    /// Validates if the given temperature is within device limits and is a valid increment.
    pub fn is_valid_temperature(&self, kelvin: u16) -> bool {
        kelvin >= self.min_temperature_kelvin 
            && kelvin <= self.max_temperature_kelvin
            && kelvin % 100 == 0
    }
}