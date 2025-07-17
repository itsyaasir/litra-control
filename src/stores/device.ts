/// Device store for managing Litra device state with Pinia.
///
/// This store handles all device-related state management including device discovery,
/// power control, brightness and temperature management, and UI operation states.

import type {
  AppError,
  BrightnessInfo,
  DeviceInfo,
  DeviceOperation,
  DeviceOperationState,
  OperationState,
  TemperatureInfo,
} from '../types'
import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useDeviceStore = defineStore('device', () => {
  // State
  const devices = ref<DeviceInfo[]>([])
  const selectedDeviceSerial = ref<string | null>(null)
  const operationStates = ref<DeviceOperationState>({})
  // Computed
  const connectedDevices = computed(() =>
    devices.value.filter(device => device.is_connected),
  )

  const disconnectedDevices = computed(() =>
    devices.value.filter(device => !device.is_connected),
  )

  const poweredOnDevices = computed(() =>
    devices.value.filter(device => device.is_on),
  )

  const selectedDevice = computed(() =>
    devices.value.find(device => device.serial_number === selectedDeviceSerial.value) || null,
  )

  const deviceStats = computed(() => ({
    total: devices.value.length,
    connected: connectedDevices.value.length,
    disconnected: disconnectedDevices.value.length,
    poweredOn: poweredOnDevices.value.length,
    poweredOff: devices.value.length - poweredOnDevices.value.length,
  }))

  // Helper functions
  const getOperationState = (serialNumber: string, operation: DeviceOperation): OperationState => {
    return operationStates.value[serialNumber]?.[operation] || {
      loading: false,
      error: null,
      success: null,
    }
  }

  const setOperationState = (
    serialNumber: string,
    operation: DeviceOperation,
    state: Partial<OperationState>,
  ) => {
    if (!operationStates.value[serialNumber]) {
      operationStates.value[serialNumber] = {}
    }
    if (!operationStates.value[serialNumber][operation]) {
      operationStates.value[serialNumber][operation] = {
        loading: false,
        error: null,
        success: null,
      }
    }
    Object.assign(operationStates.value[serialNumber][operation]!, state)
  }

  const clearOperationState = (serialNumber: string, operation: DeviceOperation) => {
    setOperationState(serialNumber, operation, {
      loading: false,
      error: null,
      success: null,
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
        success: `Discovered ${discoveredDevices.length} device(s)`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState('global', 'discovery', {
        loading: false,
        error: appError,
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
        success: 'Devices refreshed successfully',
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState('global', 'refresh', {
        loading: false,
        error: appError,
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
    }
    catch (error) {
      const appError = error as AppError
      throw appError
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
        success: `Device ${powerOn ? 'turned on' : 'turned off'} successfully`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'power', {
        loading: false,
        error: appError,
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
        success: `Device ${newState ? 'turned on' : 'turned off'} successfully`,
      })

      return newState
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'power', {
        loading: false,
        error: appError,
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
          ((lumens - device.min_brightness_lumens) / range) * 100,
        )
      }

      setOperationState(serialNumber, 'brightness', {
        loading: false,
        success: `Brightness set to ${lumens} lumens`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'brightness', {
        loading: false,
        error: appError,
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
          device.min_brightness_lumens + (range * percentage / 100),
        )
      }

      setOperationState(serialNumber, 'brightness', {
        loading: false,
        success: `Brightness set to ${percentage}%`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'brightness', {
        loading: false,
        error: appError,
      })
      throw error
    }
  }

  const setBrightnessInLumen = async (serialNumber: string, lumens: number): Promise<void> => {
    try {
      setOperationState(serialNumber, 'brightness', { loading: true, error: null })

      await invoke('set_brightness_in_lumen', { serialNumber, lumens })

      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.brightness_lumens = lumens
      }

      setOperationState(serialNumber, 'brightness', {
        loading: false,
        success: `Brightness set to ${lumens} lumens`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'brightness', {
        loading: false,
        error: appError,
      })
      throw error
    }
  }

  const getBrightnessInfo = async (serialNumber: string): Promise<BrightnessInfo> => {
    try {
      return await invoke<BrightnessInfo>('get_device_brightness', { serialNumber })
    }
    catch (error) {
      const appError = error as AppError
      throw appError
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
        success: `Temperature set to ${kelvin}K`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'temperature', {
        loading: false,
        error: appError,
      })
      throw error
    }
  }

  const setTemperatureInKelvin = async (serialNumber: string, kelvin: number): Promise<void> => {
    try {
      setOperationState(serialNumber, 'temperature', { loading: true, error: null })

      await invoke('set_temperature_in_kelvin', { serialNumber, kelvin })

      // Update device state locally
      const device = devices.value.find(d => d.serial_number === serialNumber)
      if (device) {
        device.temperature_kelvin = kelvin
      }

      setOperationState(serialNumber, 'temperature', {
        loading: false,
        success: `Temperature set to ${kelvin}K`,
      })
    }
    catch (error) {
      const appError = error as AppError
      setOperationState(serialNumber, 'temperature', {
        loading: false,
        error: appError,
      })
      throw error
    }
  }

  const getTemperatureInfo = async (serialNumber: string): Promise<TemperatureInfo> => {
    try {
      return await invoke<TemperatureInfo>('get_device_temperature', { serialNumber })
    }
    catch (error) {
      const appError = error as AppError
      throw appError
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

  return {
    // State
    devices,
    selectedDeviceSerial,
    operationStates,

    // Computed
    connectedDevices,
    disconnectedDevices,
    poweredOnDevices,
    selectedDevice,
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
    getTemperatureInfo,
    setBrightnessInLumen,
    setTemperatureInKelvin,
    selectDevice,
    selectFirstDevice,
  }
})
