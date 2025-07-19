/// Composable for device operations.
///
/// This composable provides a convenient interface for device operations
/// with automatic error handling and state management.

import type { DeviceOperation } from '../types'
import { computed, onMounted } from 'vue'
import { useDeviceStore } from '../stores'

/**
 * Composable for managing device operations with automatic state management.
 *
 * @param serialNumber - Optional serial number to focus on a specific device
 * @returns Device operations and state
 */
export function useDevice(serialNumber?: string) {
  const deviceStore = useDeviceStore()

  // Local reactive state

  const devices = computed(() => deviceStore.devices)
  const selectedDevice = computed(() => deviceStore.selectedDevice)
  const deviceStats = computed(() => deviceStore.deviceStats)

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

  // Utility functions

  // Device operations
  const discoverDevices = async () => {
    await deviceStore.discoverDevices()
  }

  const refreshDevices = async () => {
    await deviceStore.refreshDevices()
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
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number
    await deviceStore.setPowerState(serial, powerOn)
  }

  const togglePower = async (targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number
    await deviceStore.togglePower(serial)
  }

  // Brightness operations
  const setBrightness = async (lumens: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number

    await deviceStore.setBrightness(serial, lumens)
  }

  const setBrightnessPercentage = async (percentage: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number

    await deviceStore.setBrightnessPercentage(serial, percentage)
  }

  // Temperature operations
  const setTemperature = async (kelvin: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number

    await deviceStore.setTemperature(serial, kelvin)
  }

  const setTemperatureInKelvin = async (kelvin: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number

    await deviceStore.setTemperatureInKelvin(serial, kelvin)
  }

  const setBrightnessInLumen = async (lumens: number, targetSerial?: string) => {
    if (!device.value && !targetSerial) {
      throw new Error('No device selected')
    }

    const serial = targetSerial || device.value!.serial_number

    await deviceStore.setBrightnessInLumen(serial, lumens)
  }

  // Lifecycle
  onMounted(async () => {
    // Initial device discovery
    await discoverDevices()

    // Auto-select first device if none selected
    if (!deviceStore.selectedDeviceSerial && deviceStore.devices.length > 0) {
      selectFirstDevice()
    }
  })

  return {
    // State
    devices,
    selectedDevice,
    device,
    deviceStats,

    // Loading states
    isDiscovering,
    isRefreshing,
    isPowerChanging,
    isBrightnessChanging,
    isTemperatureChanging,

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
    setBrightnessInLumen,

    // Temperature operations
    setTemperature,
    setTemperatureInKelvin,

    // Store access for advanced usage
    deviceStore,
  }
}
