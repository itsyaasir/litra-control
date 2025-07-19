# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- App version display in Settings page
- Settings access button in no-devices overlay, allowing users to configure app when no Litra devices are connected

### Changed
- Improved back button design in Settings view with ChevronLeft icon
- Removed hot-reload configuration in favor of simpler implementation
- Added toast notifications to inform users to restart app after config changes

## [0.1-alpha.3] - 2025-07-19

### Added

- Added a "Start at Login" toggle in settings to enable/disable automatic startup
- Uses tauri-plugin-autostart to enable/disable automatic startup
- Starts minimized when launched at login
- Added a "Start minimized" toggle in settings to start the application minimized to system tray

### Fixed

- Fixed Control Strategy variant naming mismatch (PascalCase to camelCase) that caused configuration save errors

## [0.1-alpha.2] - 2025-07-19

### Added

- `--minimized` CLI argument to start the application minimized to system tray

### Changed

- Refactored CLI argument parsing logic into a separate `cli` module in `src-tauri/src/cli.rs`
- Updated GitHub Actions release workflow
- Simplified bundle targets in Tauri configuration
- Renamed package and updated dependencies

### Fixed

- Removed conflicting packages

## [0.1-alpha] - 2025-07-18

### Added

- Initial alpha release
- Basic Litra device control functionality
- System tray integration
- Camera monitoring capabilities
- Power, brightness, and temperature controls
- Desktop application with Tauri framework
