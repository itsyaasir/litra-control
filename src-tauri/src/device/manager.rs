/// Device manager implementation for handling Litra device operations.

use super::types::DeviceInfo;
use crate::error::{AppError, AppResult, device_not_found_error, device_communication_error};
use litra::Litra;

/// Device manager responsible for all device operations.
/// 
/// This struct manages the Litra context and provides high-level operations
/// for device discovery, state management, and communication.
pub struct DeviceManager {
    /// Litra context for device communication
    context: Litra,
}

impl DeviceManager {
    /// Creates a new DeviceManager instance.
    /// 
    /// # Returns
    /// 
    /// Returns a new DeviceManager instance or an error if the Litra context
    /// cannot be initialized.
    pub fn new() -> AppResult<Self> {
        let context = Litra::new().map_err(AppError::from)?;
        Ok(DeviceManager { context })
    }

    /// Refreshes the internal device list.
    /// 
    /// This method should be called periodically to ensure the device list
    /// is up-to-date with currently connected devices.
    pub fn refresh_devices(&mut self) -> AppResult<()> {
        self.context
            .refresh_connected_devices()
            .map_err(AppError::from)
    }

    /// Retrieves detailed information for a specific device.
    /// 
    /// # Arguments
    /// 
    /// * `serial_number` - The serial number of the device to query
    /// 
    /// # Returns
    /// 
    /// Returns complete device information or an error if the device is not found
    /// or cannot be accessed.
    pub fn get_device_info(&self, serial_number: &str) -> AppResult<DeviceInfo> {
        let devices: Vec<_> = self.context.get_connected_devices().collect();
        
        for device in devices {
            let device_serial = device.device_info().serial_number().unwrap_or("");
            if device_serial == serial_number {
                return self.create_device_info_from_device(&device);
            }
        }
        
        Err(device_not_found_error(serial_number))
    }

    /// Retrieves information for all connected devices.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of DeviceInfo structures for all discovered devices.
    /// Devices that cannot be accessed are marked as disconnected.
    pub fn get_all_devices(&self) -> AppResult<Vec<DeviceInfo>> {
        let devices: Vec<_> = self.context.get_connected_devices().collect();
        let mut device_infos = Vec::new();
        
        for device in devices {
            let device_serial = device.device_info().serial_number().unwrap_or("");
            let device_type = device.device_type().to_string();
            
            match self.create_device_info_from_device(&device) {
                Ok(info) => device_infos.push(info),
                Err(_) => {
                    // Device found but couldn't open, mark as disconnected
                    device_infos.push(DeviceInfo::disconnected(
                        device_serial.to_string(),
                        device_type,
                    ));
                }
            }
        }
        
        Ok(device_infos)
    }

    /// Creates a DeviceInfo structure from a Litra device.
    /// 
    /// # Arguments
    /// 
    /// * `device` - The Litra device to extract information from
    /// 
    /// # Returns
    /// 
    /// Returns a complete DeviceInfo structure with current device state.
    fn create_device_info_from_device(&self, device: &litra::Device) -> AppResult<DeviceInfo> {
        let device_serial = device.device_info().serial_number().unwrap_or("");
        let device_type = device.device_type().to_string();
        
        let handle = device.open(&self.context).map_err(|e| {
            device_communication_error(&format!("Failed to open device {}: {}", device_serial, e))
        })?;
        
        // Query current device state
        let is_on = handle.is_on().map_err(|e| {
            device_communication_error(&format!("Failed to get power state: {}", e))
        })?;
        
        let brightness_lumens = handle.brightness_in_lumen().map_err(|e| {
            device_communication_error(&format!("Failed to get brightness: {}", e))
        })?;
        
        let temperature_kelvin = handle.temperature_in_kelvin().map_err(|e| {
            device_communication_error(&format!("Failed to get temperature: {}", e))
        })?;
        
        // Get device capabilities
        let min_brightness = handle.minimum_brightness_in_lumen();
        let max_brightness = handle.maximum_brightness_in_lumen();
        let min_temperature = handle.minimum_temperature_in_kelvin();
        let max_temperature = handle.maximum_temperature_in_kelvin();
        
        // Calculate brightness percentage
        let brightness_percentage = if max_brightness > min_brightness {
            let range = max_brightness - min_brightness;
            let relative_brightness = brightness_lumens.saturating_sub(min_brightness);
            ((relative_brightness as f64 / range as f64) * 100.0) as u8
        } else {
            0
        };
        
        Ok(DeviceInfo {
            serial_number: device_serial.to_string(),
            device_type,
            is_connected: true,
            is_on,
            brightness_lumens,
            brightness_percentage,
            temperature_kelvin,
            min_brightness_lumens: min_brightness,
            max_brightness_lumens: max_brightness,
            min_temperature_kelvin: min_temperature,
            max_temperature_kelvin: max_temperature,
        })
    }

    /// Finds a device by serial number and returns a handle to it.
    /// 
    /// # Arguments
    /// 
    /// * `serial_number` - The serial number of the device to find
    /// 
    /// # Returns
    /// 
    /// Returns a device handle or an error if the device is not found.
    pub fn get_device_handle(&self, serial_number: &str) -> AppResult<litra::DeviceHandle> {
        let devices: Vec<_> = self.context.get_connected_devices().collect();
        
        for device in devices {
            let device_serial = device.device_info().serial_number().unwrap_or("");
            if device_serial == serial_number {
                return device.open(&self.context).map_err(|e| {
                    device_communication_error(&format!("Failed to open device {}: {}", device_serial, e))
                });
            }
        }
        
        Err(device_not_found_error(serial_number))
    }
}