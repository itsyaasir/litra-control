Directory structure:
└── timrogers-litra-rs/
    ├── README.md
    ├── 99-litra.rules
    ├── Cargo.toml
    ├── LICENSE.md
    ├── rust-toolchain.toml
    ├── .codespellignore
    ├── .pre-commit-config.yaml
    ├── src/
    │   ├── lib.rs
    │   ├── main.rs
    │   └── mcp.rs
    └── .github/
        ├── dependabot.yml
        ├── FUNDING.yml
        └── workflows/
            ├── build_and_release.yml
            └── pre-commit.yml


Files Content:

================================================
FILE: README.md
================================================
# `litra-rs`

💡 Control Logitech Litra lights from the command line, Model Context Protocol (MCP) clients and Rust applications

---

## Features

With this tool, you can:

- Turn your light on and off
- Check if the light is on or off
- Set, get, increase and decrease the brightness of your light
- Set, get, increase and decrease the temperature of your light

> [!TIP]
> 🖲️ Want to automatically turn your Litra on and off when your webcam turns on and off? Check out [`litra-autotoggle`](https://github.com/timrogers/litra-autotoggle)!

## Supported devices

The following Logitech Litra devices, __connected via USB__, are supported:

* [Logitech Litra Glow](https://www.logitech.com/en-gb/products/lighting/litra-glow.946-000002.html)
* [Logitech Litra Beam](https://www.logitech.com/en-gb/products/lighting/litra-beam.946-000007.html)
* [Logitech Litra Beam LX](https://www.logitechg.com/en-gb/products/cameras-lighting/litra-beam-lx-led-light.946-000015.html)

## Installation

### macOS or Linux via [Homebrew](https://brew.sh/)

1. Install the latest version by running `brew tap timrogers/tap && brew install litra`.
1. Run `litra --help` to check that everything is working and see the available commands.

### macOS, Linux or Windows via [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine, if it isn't already installed.
1. Install the `litra` crate by running `cargo install litra`.
1. Run `litra --help` to check that everything is working and see the available commands.

### macOS, Linux or Windows via direct binary download

1. Download the [latest release](https://github.com/timrogers/litra-rs/releases/latest) for your platform. macOS, Linux and Windows devices are supported.
2. Add the binary to `$PATH`, so you can execute it from your shell. For the best experience, call it `litra` on macOS and Linux, and `litra.exe` on Windows.
3. Run `litra --help` to check that everything is working and see the available commands.

### Configuring `udev` permissions on Linux

On most Linux operating systems, you will need to manually configure permissions using [`udev`](https://www.man7.org/linux/man-pages/man7/udev.7.html) to allow non-`root` users to access and manage Litra devices.

To allow all users that are part of the `video` group to access the Litra devices, copy the [`99-litra.rules`](99-litra.rules) file into `/etc/udev/rules.d`.
Next, reboot your computer or run the following commands as `root`:

    # udevadm control --reload-rules
    # udevadm trigger

## Usage

### From the command line

The following commands are available for controlling your devices:

- `litra on`: Turn your Logitech Litra device on
- `litra off`: Turn your Logitech Litra device off
- `litra toggle`: Toggles your Logitech Litra device on or off
- `litra brightness`: Sets the brightness of your Logitech Litra device, using either `--value` (measured in lumens) or `--percentage` (as a percentage of the device's maximum brightness). The brightness can be set to any value between the minimum and maximum for the device returned by the `devices` command.
- `litra brightness-up`: Increases the brightness of your Logitech Litra device, using either `--value` (measured in lumens) or `--percentage` (with a number of percentage points to add to the device's brightness)
- `litra brightness-down`: Decreases the brightness of your Logitech Litra device, using either `--value` (measured in lumens) or `--percentage` (with a number of percentage points to subtract from the device's brightness)
- `litra temperature`: Sets the temperature of your Logitech Litra device, using a `--value` measured in kelvin (K). The temperature can be set to any multiple of 100 between the minimum and maximum for the device returned by the `devices` command.
- `litra temperature-up`: Increases the temperature of your Logitech Litra device, using a `--value` measured in kelvin (K). The value must be a multiple of 100.
- `litra temperature-down`: Decreases the temperature of your Logitech Litra device, using a `--value` measured in kelvin (K). The value must be a multiple of 100.

All of the these commands support a `--serial-number`/`-s` argument to specify the serial number of the device you want to target. If you only have one Litra device, you can omit this argument. If you have multiple devices, we recommend specifying it. If it isn't specified, the "first" device will be picked, but this isn't guaranteed to be stable between command runs.

The following commands are also included:

- `litra devices`: List Logitech Litra devices connected to your computer. This will be returned in human-readable format by default, or you can get JSON output with the `--json` flag.

Each CLI command can also be called with `--help` for more detailed documentation.

### From a Model Context Protocol (MCP) client

Running the `litra mcp` command starts a local Model Context Protocol (MCP) server, exposing tools to allow you to control your Litra devices from AI applications and agents.

#### Usage with Claude Desktop

To use the MCP server with Claude Desktop:

1. From the Claude app, open the "Developer" menu, then click "Open App Config File...".
1. Add the MCP server to the `mcpServers` key in your config:

```json
{
  "mcpServers": {
    "litra": {
      "command": "litra",
      "args": [
        "mcp"
      ]
    }
  }
}
```

1. Back in the Claude app, open the "Developer" menu, then click "Reload MCP Configuration".
1. To check that the MCP server is running, start a chat, then click the "Search and tools" button under the chat input, and check for a "litra" item in the menu.

#### Available Tools

The following tools are available:

- `litra_devices`: List available Logitech Litra devices
- `litra_on`: Turn your Logitech Litra device on
- `litra_off`: Turn your Logitech Litra device off
- `litra_toggle`: Toggles your Logitech Litra device on or off
- `litra_brightness`: Sets the brightness of your Logitech Litra device to either a specific value measured in lumens (lm) or a percentage of the device's maximum brightness. The brightness can be set to any value between the minimum and maximum for the device returned by the `litra_devices` tool.
- `litra_brightness_up`: Increases the brightness of your Logitech Litra device, using either a specific value (measured in lumens) or a percentage of the device's maximum brightness
- `litra_brightness_down`: Decreases the brightness of your Logitech Litra device, using either a specific value (measured in lumens) or a percentage of the device's maximum brightness
- `litra_temperature`: Sets the temperature of your Logitech Litra device to a specific value measured in kelvin (K). The temperature can be set to any multiple of 100 between the minimum and maximum for the device returned by the `litra_devices` tool.
- `litra_temperature_up`: Increases the temperature of your Logitech Litra device, using a specific value measured in kelvin (K). The value must be a multiple of 100.
- `litra_temperature_down`: Decreases the temperature of your Logitech Litra device, using a specific measured in kelvin (K). The value must be a multiple of 100.

### From a Rust application

The `litra` crate includes functions for interacting with Litra devices from your Rust applications.

To see the full API, check out the documentation on [Docs.rs](https://docs.rs/litra/) or read through [`src/lib.rs`](src/lib.rs).



================================================
FILE: 99-litra.rules
================================================
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c900", GROUP="video", MODE="0660"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c901", GROUP="video", MODE="0660"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="b901", GROUP="video", MODE="0660"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="046d", ATTRS{idProduct}=="c903", GROUP="video", MODE="0660"



================================================
FILE: Cargo.toml
================================================
[package]
name = "litra"
version = "2.3.1"
edition = "2021"
authors = ["Tim Rogers <timrogers@github.com>"]
description = "Control Logitech Litra lights from the command line, Model Context Protocol (MCP) clients and Rust applications"
repository = "https://github.com/timrogers/litra-rs"
license = "MIT"
readme = "README.md"
categories = ["hardware-support", "command-line-utilities"]
keywords = ["logitech", "litra", "glow", "beam", "light"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hidapi = "2.6.3"
clap = { version = "4.5.40", features = ["derive"], optional = true }
serde = { version = "1.0.219", features = ["derive"], optional = true }
serde_json = { version = "1.0.140", optional = true }
rmcp = { version = "0.2.1", features = ["server", "transport-io", "macros"], optional = true }
tokio = { version = "1.0", features = ["rt", "rt-multi-thread", "macros"], optional = true }
tracing = { version = "0.1", optional = true }
tracing-subscriber = { version = "0.3", features = ["env-filter"], optional = true }

[features]
default = ["cli", "mcp"]
cli = ["dep:clap", "dep:serde", "dep:serde_json"]
mcp = ["cli", "dep:rmcp", "dep:tokio", "dep:tracing", "dep:tracing-subscriber"]

[[bin]]
name = "litra"
path = "src/main.rs"



================================================
FILE: LICENSE.md
================================================
The MIT License (MIT)

Copyright (c) 2024 Tim Rogers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.



================================================
FILE: rust-toolchain.toml
================================================
[toolchain]
channel = "1.85.1"
components = [ "rustc", "rustfmt", "clippy" ]
targets = [ "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "x86_64-apple-darwin", "aarch64-apple-darwin", "x86_64-pc-windows-msvc" ]
profile = "minimal"



================================================
FILE: .codespellignore
================================================
crate



================================================
FILE: .pre-commit-config.yaml
================================================
default_language_version:
  python: python3
repos:
- repo: https://github.com/pre-commit/pre-commit-hooks
  rev: v4.5.0
  hooks:
  - id: check-case-conflict
  - id: check-json
  - id: check-merge-conflict
  - id: check-symlinks
  - id: check-toml
  - id: check-xml
  - id: check-yaml
  - id: destroyed-symlinks
  - id: detect-private-key
  - id: end-of-file-fixer
  - id: fix-byte-order-marker
  - id: forbid-new-submodules
  - id: mixed-line-ending
  - id: trailing-whitespace
- repo: https://github.com/codespell-project/codespell
  rev: v2.2.6
  hooks:
  - id: codespell
    args: [
      --ignore-words=.codespellignore
    ]
- repo: https://github.com/doublify/pre-commit-rust
  rev: v1.0
  hooks:
  - id: fmt
    args: [
      --all,
      --,
    ]
  - id: cargo-check
    args: [
      --locked,
      --workspace,
      --all-features,
      --all-targets,
    ]
  - id: clippy
    args: [
      --locked,
      --workspace,
      --all-features,
      --all-targets,
      --,
      -D,
      warnings,
    ]



================================================
FILE: src/lib.rs
================================================
//! Library to query and control your Logitech Litra lights.
//!
//! # Usage
//!
//! ```
//! use litra::Litra;
//!
//! let context = Litra::new().expect("Failed to initialize litra.");
//! for device in context.get_connected_devices() {
//!     println!("Device {:?}", device.device_type());
//!     if let Ok(handle) = device.open(&context) {
//!         println!("| - Is on: {}", handle.is_on()
//!             .map(|on| if on { "yes" } else { "no" })
//!             .unwrap_or("unknown"));
//!     }
//! }
//! ```

#![warn(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::all)]
#![deny(clippy::explicit_deref_methods)]
#![deny(clippy::explicit_into_iter_loop)]
#![deny(clippy::explicit_iter_loop)]
#![deny(clippy::must_use_candidate)]
#![cfg_attr(not(test), deny(clippy::panic_in_result_fn))]
#![cfg_attr(not(debug_assertions), deny(clippy::used_underscore_binding))]

use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};
use std::error::Error;
use std::fmt;

/// Litra context.
///
/// This can be used to list available devices.
pub struct Litra(HidApi);

impl fmt::Debug for Litra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Litra").finish()
    }
}

impl Litra {
    /// Initialize a new Litra context.
    pub fn new() -> DeviceResult<Self> {
        let hidapi = HidApi::new()?;
        #[cfg(target_os = "macos")]
        hidapi.set_open_exclusive(false);
        Ok(Litra(hidapi))
    }

    /// Returns an [`Iterator`] of cached connected devices supported by this library. To refresh the list of connected devices, use [`Litra::refresh_connected_devices`].
    pub fn get_connected_devices(&self) -> impl Iterator<Item = Device<'_>> {
        self.0
            .device_list()
            .filter_map(|device_info| Device::try_from(device_info).ok())
    }

    /// Refreshes the list of connected devices, returned by [`Litra::get_connected_devices`].
    pub fn refresh_connected_devices(&mut self) -> DeviceResult<()> {
        self.0.refresh_devices()?;
        Ok(())
    }

    /// Retrieve the underlying hidapi context.
    #[must_use]
    pub fn hidapi(&self) -> &HidApi {
        &self.0
    }
}

/// The model of the device.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    /// Logitech [Litra Glow][glow] streaming light with TrueSoft.
    ///
    /// [glow]: https://www.logitech.com/products/lighting/litra-glow.html
    LitraGlow,
    /// Logitech [Litra Beam][beam] LED streaming key light with TrueSoft.
    ///
    /// [beam]: https://www.logitechg.com/products/cameras-lighting/litra-beam-streaming-light.html
    LitraBeam,
    /// Logitech [Litra Beam LX][beamlx] dual-sided RGB streaming key light.
    ///
    /// [beamlx]: https://www.logitechg.com/products/cameras-lighting/litra-beam-lx-led-light.html
    LitraBeamLX,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::LitraGlow => write!(f, "Litra Glow"),
            DeviceType::LitraBeam => write!(f, "Litra Beam"),
            DeviceType::LitraBeamLX => write!(f, "Litra Beam LX"),
        }
    }
}

/// A device-relatred error.
#[derive(Debug)]
pub enum DeviceError {
    /// Tried to use a device that is not supported.
    Unsupported,
    /// Tried to set an invalid brightness value.
    InvalidBrightness(u16),
    /// Tried to set an invalid temperature value.
    InvalidTemperature(u16),
    /// A [`hidapi`] operation failed.
    HidError(HidError),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceError::Unsupported => write!(f, "Device is not supported"),
            DeviceError::InvalidBrightness(value) => {
                write!(f, "Brightness {} lm is not supported", value)
            }
            DeviceError::InvalidTemperature(value) => {
                write!(f, "Temperature {} K is not supported", value)
            }
            DeviceError::HidError(error) => write!(f, "HID error occurred: {}", error),
        }
    }
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let DeviceError::HidError(error) = self {
            Some(error)
        } else {
            None
        }
    }
}

impl From<HidError> for DeviceError {
    fn from(error: HidError) -> Self {
        DeviceError::HidError(error)
    }
}

/// The [`Result`] of a Litra device operation.
pub type DeviceResult<T> = Result<T, DeviceError>;

/// A device that can be used.
#[derive(Debug)]
pub struct Device<'a> {
    device_info: &'a DeviceInfo,
    device_type: DeviceType,
}

impl<'a> TryFrom<&'a DeviceInfo> for Device<'a> {
    type Error = DeviceError;

    fn try_from(device_info: &'a DeviceInfo) -> Result<Self, DeviceError> {
        if device_info.vendor_id() != VENDOR_ID || device_info.usage_page() != USAGE_PAGE {
            return Err(DeviceError::Unsupported);
        }
        device_type_from_product_id(device_info.product_id())
            .map(|device_type| Device {
                device_info,
                device_type,
            })
            .ok_or(DeviceError::Unsupported)
    }
}

impl Device<'_> {
    /// The model of the device.
    #[must_use]
    pub fn device_info(&self) -> &DeviceInfo {
        self.device_info
    }

    /// The model of the device.
    #[must_use]
    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    /// Opens the device and returns a [`DeviceHandle`] that can be used for getting and setting the
    /// device status. On macOS, this will open the device in non-exclusive mode.
    pub fn open(&self, context: &Litra) -> DeviceResult<DeviceHandle> {
        let hid_device = self.device_info.open_device(context.hidapi())?;
        Ok(DeviceHandle {
            hid_device,
            device_type: self.device_type,
        })
    }
}

/// The handle of an opened device that can be used for getting and setting the device status.
#[derive(Debug)]
pub struct DeviceHandle {
    hid_device: HidDevice,
    device_type: DeviceType,
}

impl DeviceHandle {
    /// The model of the device.
    #[must_use]
    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    /// The [`HidDevice`] for the device.
    #[must_use]
    pub fn hid_device(&self) -> &HidDevice {
        &self.hid_device
    }

    /// Returns the serial number of the device.
    pub fn serial_number(&self) -> DeviceResult<Option<String>> {
        match self.hid_device.get_device_info() {
            Ok(device_info) => Ok(device_info.serial_number().map(String::from)),
            Err(error) => Err(DeviceError::HidError(error)),
        }
    }

    /// Queries the current power status of the device. Returns `true` if the device is currently on.
    pub fn is_on(&self) -> DeviceResult<bool> {
        let message = generate_is_on_bytes(&self.device_type);

        self.hid_device.write(&message)?;

        let mut response_buffer = [0x00; 20];
        let response = self.hid_device.read(&mut response_buffer[..])?;

        Ok(response_buffer[..response][4] == 1)
    }

    /// Sets the power status of the device. Turns the device on if `true` is passed and turns it
    /// of on `false`.
    pub fn set_on(&self, on: bool) -> DeviceResult<()> {
        let message = generate_set_on_bytes(&self.device_type, on);

        self.hid_device.write(&message)?;
        Ok(())
    }

    /// Queries the device's current brightness in Lumen.
    pub fn brightness_in_lumen(&self) -> DeviceResult<u16> {
        let message = generate_get_brightness_in_lumen_bytes(&self.device_type);

        self.hid_device.write(&message)?;

        let mut response_buffer = [0x00; 20];
        let response = self.hid_device.read(&mut response_buffer[..])?;

        Ok(u16::from(response_buffer[..response][4]) * 256
            + u16::from(response_buffer[..response][5]))
    }

    /// Sets the device's brightness in Lumen.
    pub fn set_brightness_in_lumen(&self, brightness_in_lumen: u16) -> DeviceResult<()> {
        if brightness_in_lumen < self.minimum_brightness_in_lumen()
            || brightness_in_lumen > self.maximum_brightness_in_lumen()
        {
            return Err(DeviceError::InvalidBrightness(brightness_in_lumen));
        }

        let message =
            generate_set_brightness_in_lumen_bytes(&self.device_type, brightness_in_lumen);

        self.hid_device.write(&message)?;
        Ok(())
    }

    /// Returns the minimum brightness supported by the device in Lumen.
    #[must_use]
    pub fn minimum_brightness_in_lumen(&self) -> u16 {
        match self.device_type {
            DeviceType::LitraGlow => 20,
            DeviceType::LitraBeam | DeviceType::LitraBeamLX => 30,
        }
    }

    /// Returns the maximum brightness supported by the device in Lumen.
    #[must_use]
    pub fn maximum_brightness_in_lumen(&self) -> u16 {
        match self.device_type {
            DeviceType::LitraGlow => 250,
            DeviceType::LitraBeam | DeviceType::LitraBeamLX => 400,
        }
    }

    /// Queries the device's current color temperature in Kelvin.
    pub fn temperature_in_kelvin(&self) -> DeviceResult<u16> {
        let message = generate_get_temperature_in_kelvin_bytes(&self.device_type);

        self.hid_device.write(&message)?;

        let mut response_buffer = [0x00; 20];
        let response = self.hid_device.read(&mut response_buffer[..])?;
        Ok(u16::from(response_buffer[..response][4]) * 256
            + u16::from(response_buffer[..response][5]))
    }

    /// Sets the device's color temperature in Kelvin.
    pub fn set_temperature_in_kelvin(&self, temperature_in_kelvin: u16) -> DeviceResult<()> {
        if temperature_in_kelvin < self.minimum_temperature_in_kelvin()
            || temperature_in_kelvin > self.maximum_temperature_in_kelvin()
            || (temperature_in_kelvin % 100) != 0
        {
            return Err(DeviceError::InvalidTemperature(temperature_in_kelvin));
        }

        let message =
            generate_set_temperature_in_kelvin_bytes(&self.device_type, temperature_in_kelvin);

        self.hid_device.write(&message)?;
        Ok(())
    }

    /// Returns the minimum color temperature supported by the device in Kelvin.
    #[must_use]
    pub fn minimum_temperature_in_kelvin(&self) -> u16 {
        MINIMUM_TEMPERATURE_IN_KELVIN
    }

    /// Returns the maximum color temperature supported by the device in Kelvin.
    #[must_use]
    pub fn maximum_temperature_in_kelvin(&self) -> u16 {
        MAXIMUM_TEMPERATURE_IN_KELVIN
    }
}

const VENDOR_ID: u16 = 0x046d;
const USAGE_PAGE: u16 = 0xff43;

fn device_type_from_product_id(product_id: u16) -> Option<DeviceType> {
    match product_id {
        0xc900 => DeviceType::LitraGlow.into(),
        0xc901 => DeviceType::LitraBeam.into(),
        0xb901 => DeviceType::LitraBeam.into(),
        0xc903 => DeviceType::LitraBeamLX.into(),
        _ => None,
    }
}

const MINIMUM_TEMPERATURE_IN_KELVIN: u16 = 2700;
const MAXIMUM_TEMPERATURE_IN_KELVIN: u16 = 6500;

fn generate_is_on_bytes(device_type: &DeviceType) -> [u8; 20] {
    match device_type {
        DeviceType::LitraGlow | DeviceType::LitraBeam => [
            0x11, 0xff, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        DeviceType::LitraBeamLX => [
            0x11, 0xff, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    }
}

fn generate_get_brightness_in_lumen_bytes(device_type: &DeviceType) -> [u8; 20] {
    match device_type {
        DeviceType::LitraGlow | DeviceType::LitraBeam => [
            0x11, 0xff, 0x04, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        DeviceType::LitraBeamLX => [
            0x11, 0xff, 0x06, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    }
}

fn generate_get_temperature_in_kelvin_bytes(device_type: &DeviceType) -> [u8; 20] {
    match device_type {
        DeviceType::LitraGlow | DeviceType::LitraBeam => [
            0x11, 0xff, 0x04, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        DeviceType::LitraBeamLX => [
            0x11, 0xff, 0x06, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    }
}

fn generate_set_on_bytes(device_type: &DeviceType, on: bool) -> [u8; 20] {
    let on_byte = if on { 0x01 } else { 0x00 };
    match device_type {
        DeviceType::LitraGlow | DeviceType::LitraBeam => [
            0x11, 0xff, 0x04, 0x1c, on_byte, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        DeviceType::LitraBeamLX => [
            0x11, 0xff, 0x06, 0x1c, on_byte, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    }
}

fn generate_set_brightness_in_lumen_bytes(
    device_type: &DeviceType,
    brightness_in_lumen: u16,
) -> [u8; 20] {
    let brightness_bytes = brightness_in_lumen.to_be_bytes();

    match device_type {
        DeviceType::LitraGlow | DeviceType::LitraBeam => [
            0x11,
            0xff,
            0x04,
            0x4c,
            brightness_bytes[0],
            brightness_bytes[1],
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ],
        DeviceType::LitraBeamLX => [
            0x11,
            0xff,
            0x06,
            0x4c,
            brightness_bytes[0],
            brightness_bytes[1],
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ],
    }
}

fn generate_set_temperature_in_kelvin_bytes(
    device_type: &DeviceType,
    temperature_in_kelvin: u16,
) -> [u8; 20] {
    let temperature_bytes = temperature_in_kelvin.to_be_bytes();

    match device_type {
        DeviceType::LitraGlow | DeviceType::LitraBeam => [
            0x11,
            0xff,
            0x04,
            0x9c,
            temperature_bytes[0],
            temperature_bytes[1],
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ],
        DeviceType::LitraBeamLX => [
            0x11,
            0xff,
            0x06,
            0x9c,
            temperature_bytes[0],
            temperature_bytes[1],
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ],
    }
}



================================================
FILE: src/main.rs
================================================
use litra::{Device, DeviceError, DeviceHandle, Litra};
use serde::Serialize;
use std::fmt;
use std::num::TryFromIntError;
use std::process::ExitCode;

#[cfg(feature = "cli")]
use clap::{ArgGroup, Parser, Subcommand};

#[cfg(feature = "mcp")]
mod mcp;

/// Control your USB-connected Logitech Litra lights from the command line
#[cfg(feature = "cli")]
#[derive(Debug, Parser)]
#[clap(name = "litra", version)]
struct Cli {
    // Test
    #[clap(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Debug, Subcommand)]
enum Commands {
    /// Turn your Logitech Litra device on
    On {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
    },
    /// Turn your Logitech Litra device off
    Off {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
    },
    /// Toggles your Logitech Litra device on or off
    Toggle {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
    },
    /// Sets the brightness of your Logitech Litra device
    #[clap(group = ArgGroup::new("brightness").required(true).multiple(false))]
    Brightness {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
        #[clap(
            long,
            short,
            help = "The brightness to set, measured in lumens. This can be set to any value between the minimum and maximum for the device returned by the `devices` command.",
            group = "brightness"
        )]
        value: Option<u16>,
        #[clap(
            long,
            short,
            help = "The brightness to set, as a percentage of the maximum brightness",
            group = "brightness"
        )]
        percentage: Option<u8>,
    },
    /// Increases the brightness of your Logitech Litra device. The command will error if trying to increase the brightness beyond the device's maximum.
    #[clap(group = ArgGroup::new("brightness-up").required(true).multiple(false))]
    BrightnessUp {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
        #[clap(
            long,
            short,
            help = "The amount to increase the brightness by, measured in lumens.",
            group = "brightness-up"
        )]
        value: Option<u16>,
        #[clap(
            long,
            short,
            help = "The number of percentage points to increase the brightness by",
            group = "brightness-up"
        )]
        percentage: Option<u8>,
    },
    /// Decreases the brightness of your Logitech Litra device. The command will error if trying to decrease the brightness below the device's minimum.
    #[clap(group = ArgGroup::new("brightness-down").required(true).multiple(false))]
    BrightnessDown {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
        #[clap(
            long,
            short,
            help = "The amount to decrease the brightness by, measured in lumens.",
            group = "brightness-down"
        )]
        value: Option<u16>,
        #[clap(
            long,
            short,
            help = "The number of percentage points to reduce the brightness by",
            group = "brightness-down"
        )]
        percentage: Option<u8>,
    },
    /// Sets the temperature of your Logitech Litra device
    Temperature {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
        #[clap(
            long,
            short,
            help = "The temperature to set, measured in Kelvin. This can be set to any multiple of 100 between the minimum and maximum for the device returned by the `devices` command."
        )]
        value: u16,
    },
    /// Increases the temperature of your Logitech Litra device. The command will error if trying to increase the temperature beyond the device's maximum.
    TemperatureUp {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
        #[clap(
            long,
            short,
            help = "The amount to increase the temperature by, measured in Kelvin. This must be a multiple of 100."
        )]
        value: u16,
    },
    /// Decreases the temperature of your Logitech Litra device. The command will error if trying to decrease the temperature below the device's minimum.
    TemperatureDown {
        #[clap(long, short, help = "The serial number of the Logitech Litra device")]
        serial_number: Option<String>,
        #[clap(
            long,
            short,
            help = "The amount to decrease the temperature by, measured in Kelvin. This must be a multiple of 100."
        )]
        value: u16,
    },
    /// List Logitech Litra devices connected to your computer
    Devices {
        #[clap(long, short, action, help = "Return the results in JSON format")]
        json: bool,
    },
    /// Start a MCP (Model Context Protocol) server for controlling Litra devices
    #[cfg(feature = "mcp")]
    Mcp,
}

fn percentage_within_range(percentage: u32, start_range: u32, end_range: u32) -> u32 {
    let range = end_range as f64 - start_range as f64;
    let result = (percentage as f64 / 100.0) * range + start_range as f64;
    result.round() as u32
}

fn get_is_on_text(is_on: bool) -> &'static str {
    if is_on {
        "On"
    } else {
        "Off"
    }
}

fn get_is_on_emoji(is_on: bool) -> &'static str {
    if is_on {
        "💡"
    } else {
        "🌑"
    }
}

fn check_serial_number_if_some(serial_number: Option<&str>) -> impl Fn(&Device) -> bool + '_ {
    move |device| {
        serial_number.as_ref().is_none_or(|expected| {
            device
                .device_info()
                .serial_number()
                .is_some_and(|actual| &actual == expected)
        })
    }
}

#[derive(Debug)]
enum CliError {
    DeviceError(DeviceError),
    SerializationFailed(serde_json::Error),
    BrightnessPercentageCalculationFailed(TryFromIntError),
    InvalidBrightness(i16),
    DeviceNotFound,
    MCPError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::DeviceError(error) => error.fmt(f),
            CliError::SerializationFailed(error) => error.fmt(f),
            CliError::BrightnessPercentageCalculationFailed(error) => {
                write!(f, "Failed to calculate brightness: {}", error)
            }
            CliError::InvalidBrightness(brightness) => {
                write!(f, "Brightness {} lm is not supported", brightness)
            }
            CliError::DeviceNotFound => write!(f, "Device not found."),
            CliError::MCPError(message) => write!(f, "MCP server error: {}", message),
        }
    }
}

impl From<DeviceError> for CliError {
    fn from(error: DeviceError) -> Self {
        CliError::DeviceError(error)
    }
}

type CliResult = Result<(), CliError>;

fn get_first_supported_device(
    context: &Litra,
    serial_number: Option<&str>,
) -> Result<DeviceHandle, CliError> {
    context
        .get_connected_devices()
        .find(check_serial_number_if_some(serial_number))
        .ok_or(CliError::DeviceNotFound)
        .and_then(|dev| dev.open(context).map_err(CliError::DeviceError))
}

#[derive(Serialize, Debug)]
struct DeviceInfo {
    pub serial_number: String,
    pub device_type: String,
    pub is_on: bool,
    pub brightness_in_lumen: u16,
    pub temperature_in_kelvin: u16,
    pub minimum_brightness_in_lumen: u16,
    pub maximum_brightness_in_lumen: u16,
    pub minimum_temperature_in_kelvin: u16,
    pub maximum_temperature_in_kelvin: u16,
}

fn get_connected_devices() -> Result<Vec<DeviceInfo>, CliError> {
    let context = Litra::new()?;
    let litra_devices: Vec<DeviceInfo> = context
        .get_connected_devices()
        .filter_map(|device| {
            let device_handle = device.open(&context).ok()?;
            Some(DeviceInfo {
                serial_number: device
                    .device_info()
                    .serial_number()
                    .unwrap_or("")
                    .to_string(),
                device_type: device.device_type().to_string(),
                is_on: device_handle.is_on().ok()?,
                brightness_in_lumen: device_handle.brightness_in_lumen().ok()?,
                temperature_in_kelvin: device_handle.temperature_in_kelvin().ok()?,
                minimum_brightness_in_lumen: device_handle.minimum_brightness_in_lumen(),
                maximum_brightness_in_lumen: device_handle.maximum_brightness_in_lumen(),
                minimum_temperature_in_kelvin: device_handle.minimum_temperature_in_kelvin(),
                maximum_temperature_in_kelvin: device_handle.maximum_temperature_in_kelvin(),
            })
        })
        .collect();
    Ok(litra_devices)
}

fn handle_devices_command(json: bool) -> CliResult {
    let litra_devices = get_connected_devices()?;

    if json {
        println!(
            "{}",
            serde_json::to_string(&litra_devices).map_err(CliError::SerializationFailed)?
        );
        Ok(())
    } else {
        if litra_devices.is_empty() {
            println!("No Logitech Litra devices found");
        } else {
            for device_info in &litra_devices {
                println!(
                    "- {} ({}): {} {}",
                    device_info.device_type,
                    device_info.serial_number,
                    get_is_on_text(device_info.is_on),
                    get_is_on_emoji(device_info.is_on)
                );

                println!("  - Brightness: {} lm", device_info.brightness_in_lumen);
                println!(
                    "    - Minimum: {} lm",
                    device_info.minimum_brightness_in_lumen
                );
                println!(
                    "    - Maximum: {} lm",
                    device_info.maximum_brightness_in_lumen
                );
                println!("  - Temperature: {} K", device_info.temperature_in_kelvin);
                println!(
                    "    - Minimum: {} K",
                    device_info.minimum_temperature_in_kelvin
                );
                println!(
                    "    - Maximum: {} K",
                    device_info.maximum_temperature_in_kelvin
                );
            }
        }

        Ok(())
    }
}

fn handle_on_command(serial_number: Option<&str>) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    device_handle.set_on(true)?;
    Ok(())
}

fn handle_off_command(serial_number: Option<&str>) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    device_handle.set_on(false)?;
    Ok(())
}

fn handle_toggle_command(serial_number: Option<&str>) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    let is_on = device_handle.is_on()?;
    device_handle.set_on(!is_on)?;
    Ok(())
}

fn handle_brightness_command(
    serial_number: Option<&str>,
    value: Option<u16>,
    percentage: Option<u8>,
) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;

    match (value, percentage) {
        (Some(_), None) => {
            let brightness_in_lumen = value.unwrap();
            device_handle.set_brightness_in_lumen(brightness_in_lumen)?;
        }
        (None, Some(_)) => {
            let brightness_in_lumen = percentage_within_range(
                percentage.unwrap().into(),
                device_handle.minimum_brightness_in_lumen().into(),
                device_handle.maximum_brightness_in_lumen().into(),
            )
            .try_into()
            .map_err(CliError::BrightnessPercentageCalculationFailed)?;

            device_handle.set_brightness_in_lumen(brightness_in_lumen)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn handle_brightness_up_command(
    serial_number: Option<&str>,
    value: Option<u16>,
    percentage: Option<u8>,
) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    let current_brightness = device_handle.brightness_in_lumen()?;

    match (value, percentage) {
        (Some(_), None) => {
            let brightness_to_add = value.unwrap();
            let new_brightness = current_brightness + brightness_to_add;
            device_handle.set_brightness_in_lumen(new_brightness)?;
        }
        (None, Some(_)) => {
            let brightness_to_add = percentage_within_range(
                percentage.unwrap().into(),
                device_handle.minimum_brightness_in_lumen().into(),
                device_handle.maximum_brightness_in_lumen().into(),
            ) as u16
                - device_handle.minimum_brightness_in_lumen();

            let new_brightness = current_brightness + brightness_to_add;

            device_handle.set_brightness_in_lumen(new_brightness)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn handle_brightness_down_command(
    serial_number: Option<&str>,
    value: Option<u16>,
    percentage: Option<u8>,
) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    let current_brightness = device_handle.brightness_in_lumen()?;

    match (value, percentage) {
        (Some(_), None) => {
            let brightness_to_subtract = value.unwrap();
            let new_brightness = current_brightness - brightness_to_subtract;
            device_handle.set_brightness_in_lumen(new_brightness)?;
        }
        (None, Some(_)) => {
            let brightness_to_subtract = percentage_within_range(
                percentage.unwrap().into(),
                device_handle.minimum_brightness_in_lumen().into(),
                device_handle.maximum_brightness_in_lumen().into(),
            ) as u16
                - device_handle.minimum_brightness_in_lumen();

            let new_brightness = current_brightness as i16 - brightness_to_subtract as i16;

            if new_brightness < 0 {
                Err(CliError::InvalidBrightness(new_brightness))?;
            }

            device_handle.set_brightness_in_lumen(new_brightness as u16)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn handle_temperature_command(serial_number: Option<&str>, value: u16) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;

    device_handle.set_temperature_in_kelvin(value)?;
    Ok(())
}

fn handle_temperature_up_command(serial_number: Option<&str>, value: u16) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    let current_temperature = device_handle.temperature_in_kelvin()?;
    let new_temperature = current_temperature + value;

    device_handle.set_temperature_in_kelvin(new_temperature)?;
    Ok(())
}

fn handle_temperature_down_command(serial_number: Option<&str>, value: u16) -> CliResult {
    let context = Litra::new()?;
    let device_handle = get_first_supported_device(&context, serial_number)?;
    let current_temperature = device_handle.temperature_in_kelvin()?;
    let new_temperature = current_temperature - value;

    device_handle.set_temperature_in_kelvin(new_temperature)?;
    Ok(())
}

#[cfg(feature = "mcp")]
fn handle_mcp_command() -> CliResult {
    mcp::handle_mcp_command()
}

#[cfg(feature = "cli")]
fn main() -> ExitCode {
    let args = Cli::parse();

    let result = match &args.command {
        Commands::Devices { json } => handle_devices_command(*json),
        Commands::On { serial_number } => handle_on_command(serial_number.as_deref()),
        Commands::Off { serial_number } => handle_off_command(serial_number.as_deref()),
        Commands::Toggle { serial_number } => handle_toggle_command(serial_number.as_deref()),
        Commands::Brightness {
            serial_number,
            value,
            percentage,
        } => handle_brightness_command(serial_number.as_deref(), *value, *percentage),
        Commands::BrightnessUp {
            serial_number,
            value,
            percentage,
        } => handle_brightness_up_command(serial_number.as_deref(), *value, *percentage),
        Commands::BrightnessDown {
            serial_number,
            value,
            percentage,
        } => handle_brightness_down_command(serial_number.as_deref(), *value, *percentage),
        Commands::Temperature {
            serial_number,
            value,
        } => handle_temperature_command(serial_number.as_deref(), *value),
        Commands::TemperatureUp {
            serial_number,
            value,
        } => handle_temperature_up_command(serial_number.as_deref(), *value),
        Commands::TemperatureDown {
            serial_number,
            value,
        } => handle_temperature_down_command(serial_number.as_deref(), *value),
        #[cfg(feature = "mcp")]
        Commands::Mcp => handle_mcp_command(),
    };

    if let Err(error) = result {
        eprintln!("{}", error);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}



================================================
FILE: src/mcp.rs
================================================
use std::future::Future;

use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    Error as McpError, ServerHandler, ServiceExt,
};

use crate::{
    get_connected_devices, handle_brightness_command, handle_brightness_down_command,
    handle_brightness_up_command, handle_off_command, handle_on_command,
    handle_temperature_command, handle_temperature_down_command, handle_temperature_up_command,
    handle_toggle_command, CliError, CliResult,
};

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct LitraToolParams {
    pub serial_number: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct LitraBrightnessParams {
    pub serial_number: Option<String>,
    pub value: Option<u16>,
    pub percentage: Option<u8>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct LitraTemperatureParams {
    pub serial_number: Option<String>,
    pub value: u16,
}

#[derive(Clone)]
pub struct LitraMcpServer {
    tool_router: ToolRouter<LitraMcpServer>,
}

#[tool_router]
impl LitraMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Turn your Logitech Litra device on")]
    async fn litra_on(
        &self,
        Parameters(params): Parameters<LitraToolParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_on_command(params.serial_number.as_deref()) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Device turned on successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Turn your Logitech Litra device off")]
    async fn litra_off(
        &self,
        Parameters(params): Parameters<LitraToolParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_off_command(params.serial_number.as_deref()) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Device turned off successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Toggle your Logitech Litra device on or off")]
    async fn litra_toggle(
        &self,
        Parameters(params): Parameters<LitraToolParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_toggle_command(params.serial_number.as_deref()) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Device toggled successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Set the brightness of your Logitech Litra device")]
    async fn litra_brightness(
        &self,
        Parameters(params): Parameters<LitraBrightnessParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_brightness_command(
            params.serial_number.as_deref(),
            params.value,
            params.percentage,
        ) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Brightness set successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Increase the brightness of your Logitech Litra device")]
    async fn litra_brightness_up(
        &self,
        Parameters(params): Parameters<LitraBrightnessParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_brightness_up_command(
            params.serial_number.as_deref(),
            params.value,
            params.percentage,
        ) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Brightness increased successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Decrease the brightness of your Logitech Litra device")]
    async fn litra_brightness_down(
        &self,
        Parameters(params): Parameters<LitraBrightnessParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_brightness_down_command(
            params.serial_number.as_deref(),
            params.value,
            params.percentage,
        ) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Brightness decreased successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Set the temperature of your Logitech Litra device")]
    async fn litra_temperature(
        &self,
        Parameters(params): Parameters<LitraTemperatureParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_temperature_command(params.serial_number.as_deref(), params.value) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Temperature set successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Increase the temperature of your Logitech Litra device")]
    async fn litra_temperature_up(
        &self,
        Parameters(params): Parameters<LitraTemperatureParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_temperature_up_command(params.serial_number.as_deref(), params.value) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Temperature increased successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Decrease the temperature of your Logitech Litra device")]
    async fn litra_temperature_down(
        &self,
        Parameters(params): Parameters<LitraTemperatureParams>,
    ) -> Result<CallToolResult, McpError> {
        match handle_temperature_down_command(params.serial_number.as_deref(), params.value) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                "Temperature decreased successfully",
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "List Logitech Litra devices connected to your computer")]
    async fn litra_devices(&self) -> Result<CallToolResult, McpError> {
        let litra_devices =
            get_connected_devices().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let json_str = serde_json::to_string_pretty(&litra_devices)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(json_str)]))
    }
}

#[tool_handler]
impl ServerHandler for LitraMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides tools to control Logitech Litra light devices. You can turn devices on/off, adjust brightness and temperature, and list connected devices. Most tools accept an optional serial_number parameter to target a specific device.".to_string()),
        }
    }
}

pub fn handle_mcp_command() -> CliResult {
    // Set up tracing for the MCP server
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::DEBUG.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    // Create the MCP server
    let rt = tokio::runtime::Runtime::new().map_err(|_| CliError::DeviceNotFound)?;
    rt.block_on(async {
        tracing::info!("Starting Litra MCP server");

        let service = LitraMcpServer::new()
            .serve(stdio())
            .await
            .map_err(|e| CliError::MCPError(format!("{e}")))?;
        service
            .waiting()
            .await
            .map_err(|e| CliError::MCPError(format!("{e}")))?;
        Ok(())
    })
}



================================================
FILE: .github/dependabot.yml
================================================
version: 2
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"



================================================
FILE: .github/FUNDING.yml
================================================
github: timrogers



================================================
FILE: .github/workflows/build_and_release.yml
================================================
name: Build, test and release

on: push

jobs:
  build:
    name: Build and test
    strategy:
      fail-fast: false
      matrix:
        job:
          - {
              target: x86_64-unknown-linux-gnu,
              binary_name: linux-amd64,
              runs_on: ubuntu-latest,
            }
          - {
              target: aarch64-unknown-linux-gnu,
              binary_name: linux-aarch64,
              runs_on: self-hosted,
            }
          - {
              target: x86_64-apple-darwin,
              binary_name: darwin-amd64,
              runs_on: macos-latest,
            }
          - {
              target: aarch64-apple-darwin,
              binary_name: darwin-aarch64,
              runs_on: macos-latest,
            }
          - {
              target: x86_64-pc-windows-msvc,
              binary_name: windows-amd64.exe,
              runs_on: windows-latest,
            }
    runs-on: ${{ matrix.job.runs_on }}
    steps:
      - name: Install rustup (self-hosted runners only)
        run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        if: matrix.job.runs_on == 'self-hosted'
      - name: Add $HOME/.cargo/bin to PATH (self-hosted runners only)
        run: echo "$HOME/.cargo/bin" >> $GITHUB_PATH
        if: matrix.job.runs_on == 'self-hosted'
      - name: Install libudev-dev
        run: sudo apt-get update && sudo apt-get install -y libudev-dev
        if: runner.os == 'Linux'
      - uses: actions/checkout@v4
      - name: Use Rust 1.85.1 with target ${{ matrix.job.target }}
        run: rustup override set 1.85.1-${{ matrix.job.target }}
      - uses: Swatinem/rust-cache@v2
      - name: Build in release mode
        run: cargo build --release --target=${{ matrix.job.target }}
      - name: Sanitise Git ref for use in filenames
        id: sanitise_ref
        run: echo "::set-output name=value::$(echo "${{ github.ref_name }}" | tr '/' '_')"
      - name: Rename Windows binary to use structured filename
        run: |
          cp target/${{ matrix.job.target }}/release/litra.exe litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}
        if: runner.os == 'Windows'
      - name: Rename Unix binary to use structured filename
        run: |
          rm target/${{ matrix.job.target }}/release/litra.d
          cp target/${{ matrix.job.target }}/release/litra* litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}
        if: runner.os != 'Windows'
      - name: Write Apple signing key to a file (macOS only)
        env:
          APPLE_SIGNING_KEY_P12: ${{ secrets.APPLE_SIGNING_KEY_P12 }}
        run: echo "$APPLE_SIGNING_KEY_P12" | base64 -d -o key.p12
        if: runner.os == 'macOS'
      - name: Write App Store Connect API key to a file (macOS only)
        env:
          APP_STORE_CONNECT_API_KEY: ${{ secrets.APP_STORE_CONNECT_API_KEY }}
        run: echo "$APP_STORE_CONNECT_API_KEY" > app_store_connect_api_key.json
        if: runner.os == 'macOS'
      - name: Sign macOS binary (macOS only)
        uses: indygreg/apple-code-sign-action@v1
        with:
          input_path: litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}
          p12_file: key.p12
          p12_password: ${{ secrets.APPLE_SIGNING_KEY_PASSWORD }}
          sign: true
          sign_args: "--code-signature-flags=runtime"
        if: runner.os == 'macOS'
      - name: Upload binary as artifact
        uses: actions/upload-artifact@v4
        with:
          path: litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}
          name: litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}
      - name: Archive macOS binary for notarisation (macOS only)
        run: zip litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}.zip litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}
        if: runner.os == 'macOS'
      - name: Notarise signed macOS binary (macOS only)
        uses: indygreg/apple-code-sign-action@v1
        with:
          input_path: litra_${{ steps.sanitise_ref.outputs.value }}_${{ matrix.job.binary_name }}.zip
          sign: false
          notarize: true
          app_store_connect_api_key_json_file: app_store_connect_api_key.json
        if: runner.os == 'macOS'
  create_and_sign_macos_universal_binary:
    name: Create and sign macOS universal binary (macOS only)
    runs-on: macos-latest
    needs: build
    steps:
      - name: Sanitise Git ref for use in filenames
        id: sanitise_ref
        run: echo "::set-output name=value::$(echo "${{ github.ref_name }}" | tr '/' '_')"
      - name: Download macOS amd64 binary
        uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-amd64
      - name: Download macOS arm64 binary
        uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-aarch64
      - name: Create universal macOS binary
        run: lipo -create -output litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal litra_${{ steps.sanitise_ref.outputs.value }}_darwin-amd64 litra_${{ steps.sanitise_ref.outputs.value }}_darwin-aarch64
      - name: Write Apple signing key to a file (macOS only)
        env:
          APPLE_SIGNING_KEY_P12: ${{ secrets.APPLE_SIGNING_KEY_P12 }}
        run: echo "$APPLE_SIGNING_KEY_P12" | base64 -d -o key.p12
      - name: Write App Store Connect API key to a file (macOS only)
        env:
          APP_STORE_CONNECT_API_KEY: ${{ secrets.APP_STORE_CONNECT_API_KEY }}
        run: echo "$APP_STORE_CONNECT_API_KEY" > app_store_connect_api_key.json
      - name: Sign macOS binary (macOS only)
        uses: indygreg/apple-code-sign-action@v1
        with:
          input_path: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal
          p12_file: key.p12
          p12_password: ${{ secrets.APPLE_SIGNING_KEY_PASSWORD }}
          sign: true
          sign_args: "--code-signature-flags=runtime"
      - name: Upload binary as artifact
        uses: actions/upload-artifact@v4
        with:
          path: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal
          name: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal
      - name: Archive macOS binary for notarisation (macOS only)
        run: zip litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal.zip litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal
      - name: Notarise signed macOS binary (macOS only)
        uses: indygreg/apple-code-sign-action@v1
        with:
          input_path: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal.zip
          sign: false
          notarize: true
          app_store_connect_api_key_json_file: app_store_connect_api_key.json

  cargo_publish_dry_run:
    name: Publish with Cargo in dry-run mode
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      - name: Install libudev-dev
        run: sudo apt-get update && sudo apt-get install -y libudev-dev
      - name: Use Rust 1.85.1
        run: rustup override set 1.85.1
      - uses: Swatinem/rust-cache@v2
      - name: Install cargo-edit
        run: cargo install cargo-edit
      - name: Set the version to a dummy version to allow publishing
        run: cargo set-version 9.9.9
      - name: Publish to Crates.io
        run: cargo publish --dry-run --allow-dirty
  create_github_release:
    name: Create release with binary assets
    runs-on: ubuntu-latest
    needs:
      - build
      - create_and_sign_macos_universal_binary
    if: startsWith(github.event.ref, 'refs/tags/v')
    steps:
      - name: Sanitise Git ref for use in filenames
        id: sanitise_ref
        run: echo "::set-output name=value::$(echo "${{ github.ref_name }}" | tr '/' '_')"
      - uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_linux-amd64
      - uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_linux-aarch64
      - uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-amd64
      - uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-aarch64
      - uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal
      - uses: actions/download-artifact@v4
        with:
          name: litra_${{ steps.sanitise_ref.outputs.value }}_windows-amd64.exe
      - name: Create release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            litra_${{ steps.sanitise_ref.outputs.value }}_windows-amd64.exe
            litra_${{ steps.sanitise_ref.outputs.value }}_darwin-amd64
            litra_${{ steps.sanitise_ref.outputs.value }}_darwin-aarch64
            litra_${{ steps.sanitise_ref.outputs.value }}_linux-amd64
            litra_${{ steps.sanitise_ref.outputs.value }}_linux-aarch64
            litra_${{ steps.sanitise_ref.outputs.value }}_darwin-universal
  cargo_publish:
    name: Publish with Cargo to Crates.io
    runs-on: ubuntu-latest
    needs:
      - create_github_release
      - cargo_publish_dry_run
    if: startsWith(github.event.ref, 'refs/tags/v')
    steps:
      - uses: actions/checkout@v4
      - name: Install libudev-dev
        run: sudo apt-get update && sudo apt-get install -y libudev-dev
      - name: Use Rust 1.85.1 with target ${{ matrix.job.target }}
        run: rustup override set 1.85.1-${{ matrix.job.target }}
      - uses: Swatinem/rust-cache@v2
      - name: Publish to Crates.io
        run: cargo publish --token ${{ secrets.CRATES_IO_API_TOKEN }}



================================================
FILE: .github/workflows/pre-commit.yml
================================================
name: pre-commit

on:
  push:
  pull_request:

jobs:
  pre-commit:
    runs-on: ubuntu-latest
    steps:
      - name: Install libudev-dev
        run: sudo apt-get update && sudo apt-get install -y libudev-dev

      - name: Check out repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 2

      - name: Set up Python
        uses: actions/setup-python@v5

      - name: Use Rust 1.85.1
        run: rustup override set 1.85.1

      - run: rustup component add clippy rustfmt

      - uses: Swatinem/rust-cache@v2

      - name: Detect code style issues
        uses: pre-commit/action@v3.0.1
        env:
          SKIP: no-commit-to-branch

      - name: Generate patch file
        if: failure()
        run: |
          git diff-index -p HEAD > "${PATCH_FILE}"
          [ -s "${PATCH_FILE}" ] && echo "UPLOAD_PATCH_FILE=${PATCH_FILE}" >> "${GITHUB_ENV}"
        env:
          PATCH_FILE: pre-commit.patch

      - name: Upload patch artifact
        if: failure() && env.UPLOAD_PATCH_FILE != null
        uses: actions/upload-artifact@v4
        with:
          name: ${{ env.UPLOAD_PATCH_FILE }}
          path: ${{ env.UPLOAD_PATCH_FILE }}


