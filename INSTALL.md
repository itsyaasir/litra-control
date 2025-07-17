# Litra Control - Installation Guide

Professional lighting control for Logitech Litra devices on Linux.

## Supported Devices

- Logitech Litra Glow
- Logitech Litra Beam
- Logitech Litra Beam LX

## Installation

### Option 1: DEB Package (Ubuntu/Debian)

1. Download the latest `.deb` package from [Releases](https://github.com/your-repo/litra-control/releases)
2. Install using:
   ```bash
   sudo dpkg -i litra-control_*.deb
   sudo apt-get install -f  # Fix any missing dependencies
   ```

### Option 2: RPM Package (Fedora/RHEL/openSUSE)

1. Download the latest `.rpm` package from [Releases](https://github.com/your-repo/litra-control/releases)
2. Install using:
   ```bash
   sudo rpm -i litra-control-*.rpm
   ```

### Option 3: AppImage (Universal)

1. Download the latest `.AppImage` from [Releases](https://github.com/your-repo/litra-control/releases)
2. Make it executable and run:
   ```bash
   chmod +x litra-control-*.AppImage
   ./litra-control-*.AppImage
   ```

## Device Permissions

### Automatic Setup (DEB/RPM packages)

The DEB and RPM packages automatically set up device permissions. After installation:

1. Log out and log back in, OR
2. Run: `sudo udevadm control --reload-rules && sudo udevadm trigger`

### Manual Setup (AppImage or development)

1. Copy the udev rules file:
   ```bash
   sudo cp packaging/99-litra-control.rules /etc/udev/rules.d/
   ```

2. Reload udev rules:
   ```bash
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```

3. Add your user to the `plugdev` group:
   ```bash
   sudo usermod -a -G plugdev $USER
   ```

4. Log out and log back in for changes to take effect.

## Building from Source

### Prerequisites

- Rust 1.70+
- Node.js 18+
- pnpm
- System dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt update
  sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libudev-dev

  # Fedora
  sudo dnf install webkit2gtk3-devel openssl-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel systemd-devel

  # Arch Linux
  sudo pacman -S webkit2gtk openssl gtk3 libappindicator-gtk3 librsvg systemd
  ```

### Build Commands

```bash
# Install dependencies
pnpm install

# Development mode
pnpm tauri dev

# Build for production
pnpm tauri build

# The built packages will be in src-tauri/target/release/bundle/
```

## Usage

1. Connect your Logitech Litra device via USB
2. Launch Litra Control from your applications menu
3. The device should be automatically detected
4. Use the controls to adjust brightness, temperature, and apply presets

## Features

- **Device Control**: Brightness and color temperature adjustment
- **Lighting Presets**: 5 pre-configured lighting scenarios
- **Auto Camera Activation**: Automatically turn on lights when camera is detected
- **Modern UI**: Clean, responsive interface with dark/light mode support
- **Multiple Device Support**: Control multiple Litra devices

## Troubleshooting

### Device not detected

1. Ensure the device is connected via USB
2. Check that udev rules are installed:
   ```bash
   ls -la /etc/udev/rules.d/99-litra-control.rules
   ```
3. Verify you're in the `plugdev` group:
   ```bash
   groups $USER
   ```
4. Try unplugging and reconnecting the device

### Permission errors

1. Reload udev rules:
   ```bash
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```
2. Log out and log back in
3. Check device permissions:
   ```bash
   ls -la /dev/hidraw*
   ```

## Uninstallation

### DEB Package

```bash
sudo apt remove litra-control
```

### RPM Package

```bash
sudo rpm -e litra-control
```

### AppImage

Simply delete the AppImage file and any desktop shortcuts.

## Support

For issues and support, please visit the [GitHub Issues](https://github.com/itsyaasir/litra-control/issues) page.
