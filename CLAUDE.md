# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri application that combines a Rust backend with a Vue 3 + TypeScript frontend. The application currently implements a basic greeting functionality that demonstrates communication between the frontend and backend.

## Architecture

- **Frontend**: Vue 3 with TypeScript, built with Vite
  - Located in `src/` directory
  - Main components: `App.vue`, `main.ts`
  - Uses Tauri API for backend communication via `@tauri-apps/api`

- **Backend**: Rust with Tauri framework
  - Located in `src-tauri/` directory
  - Main files: `src/main.rs` (entry point), `src/lib.rs` (core logic)
  - Exposes commands to frontend via Tauri's invoke system
  - Currently has a `greet` command that demonstrates Rust-to-frontend communication

## Development Commands

### Frontend Development
- `npm run dev` or `pnpm dev` - Start development server
- `npm run build` or `pnpm build` - Build frontend for production
- `npm run preview` or `pnpm preview` - Preview production build

### Tauri Development
- `npm run tauri dev` - Start Tauri development mode (runs both frontend and backend)
- `npm run tauri build` - Build the complete application

### TypeScript
- `vue-tsc --noEmit` - Type checking (included in build process)

## Key Configuration Files

- `src-tauri/tauri.conf.json` - Tauri configuration including window settings, build commands, and security
- `src-tauri/Cargo.toml` - Rust dependencies and build configuration
- `package.json` - Frontend dependencies and npm scripts
- `vite.config.ts` - Vite build configuration

## Adding New Tauri Commands

1. Define the command function in `src-tauri/src/lib.rs`
2. Add it to the `invoke_handler` in the `run()` function
3. Call it from the frontend using `invoke("command_name", { params })`

## Frontend-Backend Communication

The frontend communicates with the Rust backend using Tauri's `invoke` API. Commands are defined in Rust with the `#[tauri::command]` attribute and called from the frontend using `invoke()`.
