/// Device store for managing Litra device state with Pinia.
/// 
/// This store handles all device-related state management including device discovery,
/// power control, brightness and temperature management, and UI operation states.

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { 
  DeviceInfo, 
  BrightnessInfo, 
  TemperatureInfo, 
  TemperaturePreset,
  AppError,
  DeviceOperation,
  OperationState,
  DeviceOperationState,
  DeviceFilters
} from '../types'

export const useDeviceStore = defineStore('device', () => {
  // State
  const devices = ref<DeviceInfo[]>([])
  const selectedDeviceSerial = ref<string | null>(null)
  const operationStates = ref<DeviceOperationState>({})
  const temperaturePresets = ref<TemperaturePreset[]>([])
  const filters = ref<DeviceFilters>({
    connectionStatus: 'all',
    powerState: 'all',
    sortBy: 'name',
    sortDirection: 'asc'
  })
  const autoRefreshEnabled = ref(true)
  const autoRefreshInterval = ref<number | null>(null)

  // Computed
  const connectedDevices = computed(() => 
    devices.value.filter(device => device.is_connected)
  )

  const disconnectedDevices = computed(() => 
    devices.value.filter(device => !device.is_connected)
  )

  const poweredOnDevices = computed(() => 
    devices.value.filter(device => device.is_on)
  )

  const selectedDevice = computed(() => 
    devices.value.find(device => device.serial_number === selectedDeviceSerial.value) || null
  )

  const filteredDevices = computed(() => {
    let filtered = [...devices.value]

    // Apply filters
    if (filters.value.deviceType) {
      filtered = filtered.filter(device => device.device_type === filters.value.deviceType)
    }

    if (filters.value.connectionStatus !== 'all') {
      filtered = filtered.filter(device => 
        filters.value.connectionStatus === 'connected' ? device.is_connected : !device.is_connected
      )
    }

    if (filters.value.powerState !== 'all') {
      filtered = filtered.filter(device => 
        filters.value.powerState === 'on' ? device.is_on : !device.is_on
      )
    }

    // Apply sorting
    filtered.sort((a, b) => {
      const direction = filters.value.sortDirection === 'asc' ? 1 : -1
      
      switch (filters.value.sortBy) {
        case 'name':
          return direction * a.device_type.localeCompare(b.device_type)
        case 'type':
          return direction * a.device_type.localeCompare(b.device_type)
        case 'brightness':
          return direction * (a.brightness_lumens - b.brightness_lumens)
        case 'temperature':
          return direction * (a.temperature_kelvin - b.temperature_kelvin)
        default:
          return 0
      }
    })

    return filtered
  })

  const deviceStats = computed(() => ({
    total: devices.value.length,
    connected: connectedDevices.value.length,
    disconnected: disconnectedDevices.value.length,
    poweredOn: poweredOnDevices.value.length,
    poweredOff: devices.value.length - poweredOnDevices.value.length
  }))

  // Helper functions
  const getOperationState = (serialNumber: string, operation: DeviceOperation): OperationState => {
    return operationStates.value[serialNumber]?.[operation] || {
      loading: false,
      error: null,
      success: null
    }
  }

  const setOperationState = (
    serialNumber: string, 
    operation: DeviceOperation, 
    state: Partial<OperationState>
  ) => {
    if (!operationStates.value[serialNumber]) {
      operationStates.value[serialNumber] = {}
    }
    if (!operationStates.value[serialNumber][operation]) {
      operationStates.value[serialNumber][operation] = {
        loading: false,
        error: null,
        success: null
      }
    }
    Object.assign(operationStates.value[serialNumber][operation]!, state)
  }

  const clearOperationState = (serialNumber: string, operation: DeviceOperation) => {
    setOperationState(serialNumber, operation, {
      loading: false,
      error: null,
      success: null
    })
  }

  // Actions
  const discoverDevices = async (): Promise<void> => {
    try {
      setOperationState('global', 'discovery', { loading: true, error: null })
      
      const discoveredDevices = await invoke<DeviceInfo[]>('discover_devices')
      devices.value = discoveredDevices
      
      setOperationState('global', 'discovery', { 
        loading: false, 
        success: `Discovered ${discoveredDevices.length} device(s)` 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState('global', 'discovery', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const refreshDevices = async (): Promise<void> => {
    try {
      setOperationState('global', 'refresh', { loading: true, error: null })
      
      await invoke('refresh_devices')
      await discoverDevices()
      
      setOperationState('global', 'refresh', { 
        loading: false, 
        success: 'Devices refreshed successfully' 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState('global', 'refresh', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const getDeviceInfo = async (serialNumber: string): Promise<DeviceInfo> => {
    try {
      const deviceInfo = await invoke<DeviceInfo>('get_device_info', { serialNumber })
      
      // Update the device in our store
      const index = devices.value.findIndex(d => d.serial_number === serialNumber)
      if (index >= 0) {
        devices.value[index] = deviceInfo
      }
      
      return deviceInfo
    } catch (error) {
      throw error as AppError
    }
  }

  // Power control actions
  const setPowerState = async (serialNumber: string, powerOn: boolean): Promise<void> => {
    try {
      setOperationState(serialNumber, 'power', { loading: true, error: null })
      
      await invoke('set_device_power', { serialNumber, powerOn })
      
      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.is_on = powerOn
      }
      
      setOperationState(serialNumber, 'power', { 
        loading: false, 
        success: `Device ${powerOn ? 'turned on' : 'turned off'} successfully` 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'power', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const togglePower = async (serialNumber: string): Promise<boolean> => {
    try {
      setOperationState(serialNumber, 'power', { loading: true, error: null })
      
      const newState = await invoke<boolean>('device_power_toggle', { serialNumber })
      
      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.is_on = newState
      }
      
      setOperationState(serialNumber, 'power', { 
        loading: false, 
        success: `Device ${newState ? 'turned on' : 'turned off'} successfully` 
      })
      
      return newState
    } catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'power', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  // Brightness control actions
  const setBrightness = async (serialNumber: string, lumens: number): Promise<void> => {
    try {
      setOperationState(serialNumber, 'brightness', { loading: true, error: null })
      
      await invoke('set_device_brightness', { serialNumber, lumens })
      
      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.brightness_lumens = lumens
        // Recalculate percentage
        const range = device.max_brightness_lumens - device.min_brightness_lumens
        device.brightness_percentage = Math.round(
          ((lumens - device.min_brightness_lumens) / range) * 100
        )
      }
      
      setOperationState(serialNumber, 'brightness', { 
        loading: false, 
        success: `Brightness set to ${lumens} lumens` 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'brightness', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const setBrightnessPercentage = async (serialNumber: string, percentage: number): Promise<void> => {
    try {
      setOperationState(serialNumber, 'brightness', { loading: true, error: null })
      
      await invoke('set_device_brightness_percentage', { serialNumber, percentage })
      
      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.brightness_percentage = percentage
        // Recalculate lumens
        const range = device.max_brightness_lumens - device.min_brightness_lumens
        device.brightness_lumens = Math.round(
          device.min_brightness_lumens + (range * percentage / 100)
        )
      }
      
      setOperationState(serialNumber, 'brightness', { 
        loading: false, 
        success: `Brightness set to ${percentage}%` 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'brightness', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const getBrightnessInfo = async (serialNumber: string): Promise<BrightnessInfo> => {
    try {
      return await invoke<BrightnessInfo>('get_device_brightness', { serialNumber })
    } catch (error) {
      throw error as AppError
    }
  }

  // Temperature control actions
  const setTemperature = async (serialNumber: string, kelvin: number): Promise<void> => {
    try {
      setOperationState(serialNumber, 'temperature', { loading: true, error: null })
      
      await invoke('set_device_temperature', { serialNumber, kelvin })
      
      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.temperature_kelvin = kelvin
      }
      
      setOperationState(serialNumber, 'temperature', { 
        loading: false, 
        success: `Temperature set to ${kelvin}K` 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'temperature', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const setTemperaturePreset = async (serialNumber: string, preset: string): Promise<void> => {
    try {
      setOperationState(serialNumber, 'temperature', { loading: true, error: null })
      
      const actualTemp = await invoke<number>('set_device_temperature_preset', { 
        serialNumber, 
        preset 
      })
      
      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.temperature_kelvin = actualTemp
      }
      
      setOperationState(serialNumber, 'temperature', { 
        loading: false, 
        success: `Temperature set to ${preset} (${actualTemp}K)` 
      })
    } catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'temperature', { 
        loading: false, 
        error: appError 
      })
      throw error
    }
  }

  const getTemperatureInfo = async (serialNumber: string): Promise<TemperatureInfo> => {
    try {
      return await invoke<TemperatureInfo>('get_device_temperature', { serialNumber })
    } catch (error) {
      throw error as AppError
    }
  }

  const loadTemperaturePresets = async (): Promise<void> => {
    try {
      const presets = await invoke<TemperaturePreset[]>('get_temperature_presets')
      temperaturePresets.value = presets
    } catch (error) {
      console.error('Failed to load temperature presets:', error)
    }
  }

  // Auto-refresh functionality
  const startAutoRefresh = (intervalMs: number = 5000): void => {
    stopAutoRefresh()
    autoRefreshInterval.value = window.setInterval(() => {
      if (autoRefreshEnabled.value) {
        refreshDevices().catch(console.error)
      }
    }, intervalMs)
  }

  const stopAutoRefresh = (): void => {
    if (autoRefreshInterval.value) {
      clearInterval(autoRefreshInterval.value)
      autoRefreshInterval.value = null
    }
  }

  // Selection management
  const selectDevice = (serialNumber: string | null): void => {
    selectedDeviceSerial.value = serialNumber
  }

  const selectFirstDevice = (): void => {
    if (devices.value.length > 0) {
      selectedDeviceSerial.value = devices.value[0].serial_number
    }
  }

  // Filter management
  const setFilters = (newFilters: Partial<DeviceFilters>): void => {
    filters.value = { ...filters.value, ...newFilters }
  }

  const resetFilters = (): void => {
    filters.value = {
      connectionStatus: 'all',
      powerState: 'all',
      sortBy: 'name',
      sortDirection: 'asc'
    }
  }

  return {
    // State
    devices,
    selectedDeviceSerial,
    operationStates,
    temperaturePresets,
    filters,
    autoRefreshEnabled,
    autoRefreshInterval,

    // Computed
    connectedDevices,
    disconnectedDevices,
    poweredOnDevices,
    selectedDevice,
    filteredDevices,
    deviceStats,

    // Helper functions
    getOperationState,
    setOperationState,
    clearOperationState,

    // Actions
    discoverDevices,
    refreshDevices,
    getDeviceInfo,
    setPowerState,
    togglePower,
    setBrightness,
    setBrightnessPercentage,
    getBrightnessInfo,
    setTemperature,
    setTemperaturePreset,
    getTemperatureInfo,
    loadTemperaturePresets,
    startAutoRefresh,
    stopAutoRefresh,
    selectDevice,
    selectFirstDevice,
    setFilters,
    resetFilters
  }
})