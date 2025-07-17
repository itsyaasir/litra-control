/// Composable for device operations.
///
/// This composable provides a convenient interface for device operations
/// with automatic error handling and state management.

import type { DeviceOperation } from '../types'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useDeviceStore, useSettingsStore } from '../stores'

/**
 * Composable for managing device operations with automatic state management.
 *
 * @param serialNumber - Optional serial number to focus on a specific device
 * @returns Device operations and state
 */
export function useDevice(serialNumber?: string) {
  const deviceStore = useDeviceStore()
  const settingsStore = useSettingsStore()

  // Local reactive state
  const localError = ref<string | null>(null)
  const localSuccess = ref<string | null>(null)

  // Computed properties
  const devices = computed(() => deviceStore.filteredDevices)
  const selectedDevice = computed(() => deviceStore.selectedDevice)
  const deviceStats = computed(() => deviceStore.deviceStats)
  const temperaturePresets = computed(() => deviceStore.temperaturePresets)

  // Get specific device if serialNumber provided
  const device = computed(() => {
    if (serialNumber) {
      return deviceStore.devices.find(d => d.serial_number === serialNumber) || null
    }
    return selectedDevice.value
  })

  // Operation states for the current device
  const isDiscovering = computed(() =>
    deviceStore.getOperationState('global', 'discovery').loading,
  )

  const isRefreshing = computed(() =>
    deviceStore.getOperationState('global', 'refresh').loading,
  )

  const getDeviceOperationState = (operation: DeviceOperation) => {
    if (!device.value)
      return { loading: false, error: null, success: null }
    return deviceStore.getOperationState(device.value.serial_number, operation)
  }

  const isPowerChanging = computed(() =>
    device.value ? getDeviceOperationState('power').loading : false,
  )

  const isBrightnessChanging = computed(() =>
    device.value ? getDeviceOperationState('brightness').loading : false,
  )

  const isTemperatureChanging = computed(() =>
    device.value ? getDeviceOperationState('temperature').loading : false,
  )

  // Error and success states
  const hasError = computed(() =>
    localError.value !== null
    || deviceStore.getOperationState('global', 'discovery').error !== null
    || deviceStore.getOperationState('global', 'refresh').error !== null
    || (device.value && Object.values(['power', 'brightness', 'temperature'] as const)
      .some(op => getDeviceOperationState(op).error !== null)),
  )

  const hasSuccess = computed(() =>
    localSuccess.value !== null
    || deviceStore.getOperationState('global', 'discovery').success !== null
    || deviceStore.getOperationState('global', 'refresh').success !== null
    || (device.value && Object.values(['power', 'brightness', 'temperature'] as const)
      .some(op => getDeviceOperationState(op).success !== null)),
  )

  // Utility functions
  const clearMessages = () => {
    localError.value = null
    localSuccess.value = null

    if (device.value) {
      ['power', 'brightness', 'temperature'].forEach((op) => {
        deviceStore.clearOperationState(device.value!.serial_number, op as DeviceOperation)
      })
    }

    deviceStore.clearOperationState('global', 'discovery')
    deviceStore.clearOperationState('global', 'refresh')
  }

  const showSuccess = (message: string) => {
    localSuccess.value = message
    setTimeout(() => {
      localSuccess.value = null
    }, 3000)
  }

  const showError = (message: string) => {
    localError.value = message
    setTimeout(() => {
      localError.value = null
    }, 5000)
  }

  // Device operations
  const discoverDevices = async () => {
    try {
      await deviceStore.discoverDevices()
    }
    catch (error: any) {
      showError(error.message || 'Failed to discover devices')
    }
  }

  const refreshDevices = async () => {
    try {
      await deviceStore.refreshDevices()
    }
    catch (error: any) {
      showError(error.message || 'Failed to refresh devices')
    }
  }

  const selectDevice = (serialNumber: string | null) => {
    deviceStore.selectDevice(serialNumber)
  }

  const selectFirstDevice = () => {
    deviceStore.selectFirstDevice()
  }

  // Power operations
  const setPowerState = async (powerOn: boolean, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      showError('No device selected')
      return
    }

    const serial = targetSerial || device.value!.serial_number

    try {
      await deviceStore.setPowerState(serial, powerOn)
    }
    catch (error: any) {
      showError(error.message || `Failed to ${powerOn ? 'turn on' : 'turn off'} device`)
    }
  }

  const togglePower = async (targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      showError('No device selected')
      return
    }

    const serial = targetSerial || device.value!.serial_number

    try {
      await deviceStore.togglePower(serial)
    }
    catch (error: any) {
      showError(error.message || 'Failed to toggle device power')
    }
  }

  // Brightness operations
  const setBrightness = async (lumens: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      showError('No device selected')
      return
    }

    const serial = targetSerial || device.value!.serial_number

    try {
      await deviceStore.setBrightness(serial, lumens)
    }
    catch (error: any) {
      showError(error.message || 'Failed to set brightness')
    }
  }

  const setBrightnessPercentage = async (percentage: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      showError('No device selected')
      return
    }

    const serial = targetSerial || device.value!.serial_number

    try {
      await deviceStore.setBrightnessPercentage(serial, percentage)
    }
    catch (error: any) {
      showError(error.message || 'Failed to set brightness')
    }
  }

  // Temperature operations
  const setTemperature = async (kelvin: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      showError('No device selected')
      return
    }

    const serial = targetSerial || device.value!.serial_number

    try {
      await deviceStore.setTemperature(serial, kelvin)
    }
    catch (error: any) {
      showError(error.message || 'Failed to set temperature')
    }
  }

  const setTemperaturePreset = async (preset: string, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      showError('No device selected')
      return
    }

    const serial = targetSerial || device.value!.serial_number

    try {
      await deviceStore.setTemperaturePreset(serial, preset)
    }
    catch (error: any) {
      showError(error.message || 'Failed to set temperature preset')
    }
  }

  // Auto-refresh management
  const startAutoRefresh = () => {
    deviceStore.startAutoRefresh(settingsStore.settings.autoRefreshInterval)
  }

  const stopAutoRefresh = () => {
    deviceStore.stopAutoRefresh()
  }

  const toggleAutoRefresh = () => {
    if (deviceStore.autoRefreshEnabled) {
      stopAutoRefresh()
    }
    else {
      startAutoRefresh()
    }
    deviceStore.autoRefreshEnabled = !deviceStore.autoRefreshEnabled
  }

  // Lifecycle
  onMounted(async () => {
    // Load temperature presets
    await deviceStore.loadTemperaturePresets()

    // Initial device discovery
    await discoverDevices()

    // Auto-select first device if none selected
    if (!deviceStore.selectedDeviceSerial && deviceStore.devices.length > 0) {
      selectFirstDevice()
    }

    // Start auto-refresh if enabled
    if (settingsStore.settings.autoRefreshInterval > 0) {
      startAutoRefresh()
    }
  })

  onUnmounted(() => {
    stopAutoRefresh()
  })

  // Watch for settings changes
  watch(
    () => settingsStore.settings.autoRefreshInterval,
    (newInterval) => {
      if (deviceStore.autoRefreshEnabled) {
        stopAutoRefresh()
        if (newInterval > 0) {
          startAutoRefresh()
        }
      }
    },
  )

  return {
    // State
    devices,
    selectedDevice,
    device,
    deviceStats,
    temperaturePresets,

    // Loading states
    isDiscovering,
    isRefreshing,
    isPowerChanging,
    isBrightnessChanging,
    isTemperatureChanging,

    // Error and success states
    hasError,
    hasSuccess,
    localError,
    localSuccess,

    // Utility functions
    clearMessages,
    showSuccess,
    showError,

    // Device operations
    discoverDevices,
    refreshDevices,
    selectDevice,
    selectFirstDevice,

    // Power operations
    setPowerState,
    togglePower,

    // Brightness operations
    setBrightness,
    setBrightnessPercentage,

    // Temperature operations
    setTemperature,
    setTemperaturePreset,

    // Auto-refresh
    startAutoRefresh,
    stopAutoRefresh,
    toggleAutoRefresh,

    // Store access for advanced usage
    deviceStore,
    settingsStore,
  }
}
