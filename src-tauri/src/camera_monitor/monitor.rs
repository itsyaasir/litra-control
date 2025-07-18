//! Core camera monitoring implementation
//!
//! This module contains the main camera monitoring logic using inotify
//! to detect camera device activity on Linux systems.

use crate::camera_monitor::{
    strategies::{create_device_selector, DeviceSelector},
    CameraMonitorResult,
};
use crate::commands::DeviceManagerState;
use crate::config::AutoToggleConfig;
use inotify::{EventMask, Inotify, WatchMask};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;

const MONITOR_PATH: &str = "/dev";
const VIDEO_DEVICE_FILTER: &str = "video*";

/// Main camera monitor structure
pub struct CameraMonitor {
    /// Device manager reference
    device_manager: DeviceManagerState,
    /// Current monitoring state
    is_monitoring: bool,
    /// Current camera device count
    device_count: usize,
    /// Last event timestamp for debouncing
    last_event_time: Option<Instant>,
    /// Monitoring task handle
    monitor_handle: Option<tokio::task::JoinHandle<()>>,
    /// Devices currently controlled by auto-toggle
    controlled_devices: Vec<String>,
    /// Channel for stopping monitoring
    stop_tx: Option<mpsc::Sender<()>>,
}

impl CameraMonitor {
    /// Create a new camera monitor
    pub fn new(device_manager: DeviceManagerState) -> Self {
        Self {
            device_manager,
            is_monitoring: false,
            device_count: 0,
            last_event_time: None,
            monitor_handle: None,
            controlled_devices: Vec::new(),
            stop_tx: None,
        }
    }

    /// Start camera monitoring
    pub async fn start_monitoring(
        &mut self,
        auto_toggle_config: AutoToggleConfig,
    ) -> CameraMonitorResult<()> {
        if self.is_monitoring {
            return Ok(());
        }

        if !auto_toggle_config.enabled {
            return Err("Auto-toggle is disabled in configuration".into());
        }

        // Initialize device count to 0 - we'll track actual usage through events
        self.device_count = 0;

        // Create stop channel
        let (stop_tx, stop_rx) = mpsc::channel(1);
        self.stop_tx = Some(stop_tx);

        // Start monitoring task
        let monitor_handle = self.start_monitor_task(auto_toggle_config, stop_rx).await?;
        self.monitor_handle = Some(monitor_handle);

        self.is_monitoring = true;

        Ok(())
    }

    /// Stop camera monitoring
    pub async fn stop_monitoring(&mut self) -> CameraMonitorResult<()> {
        if !self.is_monitoring {
            return Ok(());
        }

        // Send stop signal
        if let Some(stop_tx) = self.stop_tx.take() {
            let _ = stop_tx.send(()).await;
        }

        // Wait for monitoring task to complete
        if let Some(handle) = self.monitor_handle.take() {
            handle.abort();
        }

        self.is_monitoring = false;
        self.device_count = 0;
        self.last_event_time = None;

        Ok(())
    }

    /// Check if monitoring is active
    pub fn is_monitoring(&self) -> bool {
        self.is_monitoring
    }

    /// Get current device count
    pub fn get_device_count(&self) -> usize {
        self.device_count
    }

    /// Get controlled devices
    pub fn get_controlled_devices(&self) -> Vec<String> {
        self.controlled_devices.clone()
    }

    /// Start the monitoring task
    async fn start_monitor_task(
        &self,
        config: AutoToggleConfig,
        mut stop_rx: mpsc::Receiver<()>,
    ) -> CameraMonitorResult<tokio::task::JoinHandle<()>> {
        let device_manager = self.device_manager.clone();

        let handle = tokio::spawn(async move {
            if let Err(e) = Self::monitor_loop(config, device_manager, &mut stop_rx).await {
                eprintln!("Camera monitor error: {e}");
            }
        });

        Ok(handle)
    }

    /// Main monitoring loop
    async fn monitor_loop(
        config: AutoToggleConfig,
        device_manager: DeviceManagerState,
        stop_rx: &mut mpsc::Receiver<()>,
    ) -> CameraMonitorResult<()> {
        let mut inotify = Inotify::init()?;
        let _watch_descriptor = inotify.watches().add(
            MONITOR_PATH,
            WatchMask::OPEN | WatchMask::CLOSE_WRITE | WatchMask::CLOSE_NOWRITE,
        )?;

        // Initialize device count to 0 - we'll track actual usage through events
        let mut device_count = 0;
        let mut last_event_time: Option<Instant> = None;
        let mut controlled_devices: Vec<String> = Vec::new();

        // Create device selector
        let device_selector = create_device_selector(&config.strategy);

        println!(
            "Camera monitoring started, watching: {MONITOR_PATH}, tracking actual camera usage"
        );

        // Start with no active cameras - devices will only turn on when cameras are actually opened
        println!("Monitoring camera activity, devices will turn on when cameras are opened");

        let mut buffer = [0; 1024];

        loop {
            // Check for stop signal
            if let Ok(()) = stop_rx.try_recv() {
                break;
            }

            // Read inotify events (non-blocking)
            match inotify.read_events(&mut buffer) {
                Ok(events) => {
                    let mut video_events = Vec::new();

                    // Filter for video device events
                    for event in events {
                        if let Some(name) = event.name {
                            if let Some(name_str) = name.to_str() {
                                if name_str.starts_with(&VIDEO_DEVICE_FILTER.replace("*", "")) {
                                    video_events.push((name_str.to_string(), event.mask));
                                }
                            }
                        }
                    }

                    // Process video events
                    if !video_events.is_empty() {
                        device_count = Self::process_video_events(
                            video_events,
                            device_count,
                            &*device_selector,
                            &device_manager,
                            &mut controlled_devices,
                        )
                        .await?;

                        last_event_time = Some(Instant::now());
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No events available, check debounce
                    if let Some(event_time) = last_event_time {
                        if event_time.elapsed() >= Duration::from_millis(config.debounce_ms) {
                            // Debounce period expired, finalize state
                            Self::finalize_device_state(
                                device_count,
                                &device_manager,
                                &mut controlled_devices,
                            )
                            .await?;
                            last_event_time = None;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Inotify error: {e}");
                }
            }

            // Small delay to prevent busy waiting
            sleep(Duration::from_millis(100)).await;
        }

        println!("Camera monitoring stopped");
        Ok(())
    }

    /// Process video device events
    async fn process_video_events(
        events: Vec<(String, EventMask)>,
        mut device_count: usize,
        device_selector: &dyn DeviceSelector,
        device_manager: &DeviceManagerState,
        controlled_devices: &mut Vec<String>,
    ) -> CameraMonitorResult<usize> {
        let mut open_count = 0;
        let mut close_count = 0;

        // Count open/close events
        for (device_name, mask) in events {
            match mask {
                EventMask::OPEN => {
                    open_count += 1;
                    println!("Camera opened: {device_name}");
                }
                EventMask::CLOSE_WRITE | EventMask::CLOSE_NOWRITE => {
                    close_count += 1;
                    println!("Camera closed: {device_name}");
                }
                _ => {}
            }
        }

        // Update device count based on net change
        let net_change = open_count - close_count;
        if net_change > 0 {
            device_count = device_count.saturating_add(net_change as usize);
        } else if net_change < 0 {
            device_count = device_count.saturating_sub((-net_change) as usize);
        }

        // React to any camera activity changes
        if net_change != 0 {
            println!("Camera activity change: {net_change}, total active sessions: {device_count}");

            if device_count > 0 && net_change > 0 {
                // Turn on devices when cameras are opened
                println!("Cameras detected, turning on devices");
                Self::turn_on_devices(device_selector, device_manager, controlled_devices).await?;
            } else if device_count == 0 {
                // Turn off devices immediately when no active camera sessions
                println!("No active camera sessions, turning off devices");
                Self::turn_off_devices(device_manager, controlled_devices).await?;
            }
        }

        Ok(device_count)
    }

    /// Finalize device state after debounce period
    async fn finalize_device_state(
        device_count: usize,
        device_manager: &DeviceManagerState,
        controlled_devices: &mut Vec<String>,
    ) -> CameraMonitorResult<()> {
        if device_count == 0 {
            println!("Debounce period completed, no active camera sessions - turning off devices");
            // Turn off devices after debounce
            Self::turn_off_devices(device_manager, controlled_devices).await?;
        } else {
            println!(
                "Debounce period completed, {device_count} camera sessions still active - keeping devices on"
            );
        }

        Ok(())
    }

    /// Turn on devices based on strategy
    async fn turn_on_devices(
        device_selector: &dyn DeviceSelector,
        device_manager: &DeviceManagerState,
        controlled_devices: &mut Vec<String>,
    ) -> CameraMonitorResult<()> {
        let devices = {
            let dm = device_manager.lock().await;
            dm.get_all_devices()?
        };

        for device in devices {
            if device_selector.should_control_device(&device) && !device.is_on {
                // Turn on device
                let success = {
                    let dm = device_manager.lock().await;
                    if let Ok(handle) = dm.get_device_handle(&device.serial_number) {
                        handle.set_on(true).is_ok()
                    } else {
                        false
                    }
                };

                if success {
                    controlled_devices.push(device.serial_number.clone());
                }
            }
        }

        Ok(())
    }

    /// Turn off devices based on strategy
    async fn turn_off_devices(
        device_manager: &DeviceManagerState,
        controlled_devices: &mut Vec<String>,
    ) -> CameraMonitorResult<()> {
        // Only turn off devices we turned on
        for serial_number in controlled_devices.drain(..) {
            // Turn off device
            let success = {
                let dm = device_manager.lock().await;
                if let Ok(handle) = dm.get_device_handle(&serial_number) {
                    handle.set_on(false).is_ok()
                } else {
                    false
                }
            };

            if success {}
        }

        Ok(())
    }
}
