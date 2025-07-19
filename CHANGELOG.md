# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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