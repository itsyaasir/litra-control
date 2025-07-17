/// TypeScript interfaces for Litra device data structures.
/// 
/// These interfaces match the Rust backend structures to ensure type safety
/// across the frontend-backend boundary.

/**
 * Complete device information structure from the backend.
 * 
 * Contains all necessary information about a Litra device including
 * its current state, capabilities, and configuration limits.
 */
export interface DeviceInfo {
  /** Device serial number (unique identifier) */
  serial_number: string;
  
  /** Device model name (e.g., "Litra Glow", "Litra Beam", "Litra Beam LX") */
  device_type: string;
  
  /** Current connection status */
  is_connected: boolean;
  
  /** Current power state (on/off) */
  is_on: boolean;
  
  /** Current brightness level in lumens */
  brightness_lumens: number;
  
  /** Current brightness as percentage (0-100) */
  brightness_percentage: number;
  
  /** Current color temperature in Kelvin */
  temperature_kelvin: number;
  
  /** Minimum brightness supported by this device */
  min_brightness_lumens: number;
  
  /** Maximum brightness supported by this device */
  max_brightness_lumens: number;
  
  /** Minimum color temperature supported (typically 2700K) */
  min_temperature_kelvin: number;
  
  /** Maximum color temperature supported (typically 6500K) */
  max_temperature_kelvin: number;
}

/**
 * Comprehensive brightness information structure from the backend.
 */
export interface BrightnessInfo {
  /** Current brightness level in lumens */
  current_lumens: number;
  
  /** Current brightness as percentage (0-100) */
  current_percentage: number;
  
  /** Minimum brightness supported by the device */
  min_lumens: number;
  
  /** Maximum brightness supported by the device */
  max_lumens: number;
}

/**
 * Comprehensive temperature information structure from the backend.
 */
export interface TemperatureInfo {
  /** Current temperature in Kelvin */
  current_kelvin: number;
  
  /** Current temperature as percentage of range (0-100) */
  current_percentage: number;
  
  /** Minimum temperature supported by the device */
  min_kelvin: number;
  
  /** Maximum temperature supported by the device */
  max_kelvin: number;
  
  /** Temperature step increment (always 100K for Litra devices) */
  step_kelvin: number;
}

/**
 * Temperature preset information structure from the backend.
 */
export interface TemperaturePreset {
  /** Human-readable name of the preset */
  name: string;
  
  /** Description of the lighting scenario */
  description: string;
  
  /** Temperature value in Kelvin */
  kelvin: number;
}

/**
 * Application error structure from the backend.
 */
export interface AppError {
  /** Human-readable error message */
  message: string;
  
  /** Categorized error type for programmatic handling */
  error_type: string;
}

/**
 * Device control operation types for UI state management.
 */
export type DeviceOperation = 
  | 'power'
  | 'brightness'
  | 'temperature'
  | 'discovery'
  | 'refresh';

/**
 * Device operation state for tracking async operations.
 */
export interface OperationState {
  /** Whether an operation is currently in progress */
  loading: boolean;
  
  /** Error from the last operation, if any */
  error: AppError | null;
  
  /** Success message from the last operation, if any */
  success: string | null;
}

/**
 * UI state for device operations.
 */
export type DeviceOperationState = Record<string, Partial<Record<DeviceOperation, OperationState>>>;

/**
 * Device filter and sort options for the UI.
 */
export interface DeviceFilters {
  /** Filter by device type */
  deviceType?: string;
  
  /** Filter by connection status */
  connectionStatus?: 'all' | 'connected' | 'disconnected';
  
  /** Filter by power state */
  powerState?: 'all' | 'on' | 'off';
  
  /** Sort field */
  sortBy?: 'name' | 'type' | 'brightness' | 'temperature';
  
  /** Sort direction */
  sortDirection?: 'asc' | 'desc';
}

/**
 * Application settings structure.
 */
export interface AppSettings {
  /** Theme preference */
  theme: 'light' | 'dark' | 'system';
  
  /** Auto-refresh interval in seconds */
  autoRefreshInterval: number;
  
  /** Whether to show notifications */
  showNotifications: boolean;
  
  /** Default brightness for new devices */
  defaultBrightness: number;
  
  /** Default temperature for new devices */
  defaultTemperature: number;
}