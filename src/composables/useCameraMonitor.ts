import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'

export interface CameraMonitorState {
  isMonitoring: boolean
  deviceCount: number
  controlledDevices: string[]
  isLoading: boolean
}

export interface AutoToggleConfig {
  enabled: boolean
  strategy: 'allDevices' | { selectedDevice: { serialNumber: string } }
  debounceMs: number
}

export function useCameraMonitor() {
  // State
  const state = ref<CameraMonitorState>({
    isMonitoring: false,
    deviceCount: 0,
    controlledDevices: [],
    isLoading: false,
  })

  // Config state
  const config = ref<AutoToggleConfig>({
    enabled: false,
    strategy: 'allDevices',
    debounceMs: 3000,
  })

  // Computed properties
  const isEnabled = computed(() => config.value.enabled)
  const isMonitoring = computed(() => state.value.isMonitoring)
  const hasActiveDevices = computed(() => state.value.deviceCount > 0)
  const controlledDeviceCount = computed(() => state.value.controlledDevices.length)

  const refreshStatus = async () => {
    try {
      const [isMonitoringResult, deviceCount, controlledDevices] = await Promise.all([
        invoke<boolean>('is_camera_monitoring'),
        invoke<number>('get_camera_device_count'),
        invoke<string[]>('get_controlled_devices'),
      ])

      state.value.isMonitoring = isMonitoringResult
      state.value.deviceCount = deviceCount
      state.value.controlledDevices = controlledDevices
    }
    catch (error) {
      console.error('[Camera Monitor] Failed to refresh status:', error)
    }
  }

  // API calls
  const startMonitoring = async () => {
    try {
      state.value.isLoading = true
      await invoke('start_camera_monitoring')
      await refreshStatus()
    }
    catch (error: any) {
      const errorMessage = error?.message || error?.toString() || 'Unknown error'
      console.error('[Camera Monitor] Start monitoring failed:', error)
      throw new Error(`Failed to start monitoring: ${errorMessage}`)
    }
    finally {
      state.value.isLoading = false
    }
  }

  const stopMonitoring = async () => {
    try {
      state.value.isLoading = true
      await invoke('stop_camera_monitoring')
      await refreshStatus()
    }
    catch (error: any) {
      const errorMessage = error?.message || error?.toString() || 'Unknown error'
      console.error('[Camera Monitor] Stop monitoring failed:', error)
      throw new Error(`Failed to stop monitoring: ${errorMessage}`)
    }
    finally {
      state.value.isLoading = false
    }
  }

  // Toggle monitoring
  const toggleMonitoring = async () => {
    if (state.value.isMonitoring) {
      await stopMonitoring()
    }
    else {
      await startMonitoring()
    }
  }

  // Config management
  const updateConfig = async (newConfig: Partial<AutoToggleConfig>) => {
    const updatedConfig = { ...config.value, ...newConfig }

    try {
      // Save to backend
      await invoke('update_camera_config', { config: updatedConfig })

      // Update local state only if backend save succeeds
      config.value = updatedConfig
    }
    catch (error: any) {
      const errorMessage = error?.message || error?.toString() || 'Unknown error'
      console.error('[Camera Monitor] Failed to update config:', error)
      throw new Error(`Failed to save configuration: ${errorMessage}`)
    }
  }

  const resetConfig = () => {
    config.value = {
      enabled: false,
      strategy: 'allDevices',
      debounceMs: 3000,
    }
  }

  // Load config from backend
  const loadConfig = async () => {
    try {
      const backendConfig = await invoke<AutoToggleConfig>('get_camera_config')
      config.value = backendConfig
    }
    catch (error) {
      console.error('[Camera Monitor] Failed to load config from backend:', error)
      // Keep default config if backend fails
    }
  }

  // Lifecycle
  onMounted(async () => {
    await loadConfig()
    await refreshStatus()

    // Auto-start monitoring if enabled
    if (config.value.enabled && !state.value.isMonitoring) {
      await startMonitoring()
    }
  })

  return {
    // State
    state: computed(() => state.value),
    config: computed(() => config.value),

    // Computed
    isEnabled,
    isMonitoring,
    hasActiveDevices,
    controlledDeviceCount,

    // Actions
    startMonitoring,
    stopMonitoring,
    toggleMonitoring,
    refreshStatus,
    updateConfig,
    resetConfig,
  }
}
