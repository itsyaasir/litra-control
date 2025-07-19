<script setup lang="ts">
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { useColorMode, useDebounceFn } from '@vueuse/core'
import { ArrowLeft, Camera, Monitor, Moon, RefreshCw, Settings, Sun, Video } from 'lucide-vue-next'
import { computed, onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { toast } from 'vue-sonner'
import CustomTitlebar from '@/components/CustomTitlebar.vue'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Separator } from '@/components/ui/separator'
import { Switch } from '@/components/ui/switch'
import { useCameraMonitor, useDevice } from '@/composables'

const router = useRouter()
const cameraMonitor = useCameraMonitor()
const device = useDevice()

// Theme management
const colorMode = useColorMode()
// Theme options
const themeOptions = [
  { value: 'light', label: 'Light', icon: Sun },
  { value: 'dark', label: 'Dark', icon: Moon },
]

// Autostart state
const autostartEnabled = ref(false)

// Local config state
const localConfig = ref({ ...cameraMonitor.config.value })

// Debug state
const debugInfo = ref({
  isLoading: false,
  lastRefresh: null as Date | null,
  rawStatus: {} as any,
})

// Available strategies
const strategies = [
  {
    value: 'allDevices',
    label: 'All Devices',
    description: 'Control all connected Litra devices',
  },
  {
    value: 'selectedDevice',
    label: 'Selected Device',
    description: 'Control only a specific device',
  },
]

// Available devices for selection
const availableDevices = computed(() => device.devices.value)

// Auto-save flag to prevent saving when config is being loaded
const isLoadingConfig = ref(false)

// Auto-save configuration with debouncing using VueUse
const autoSaveConfig = useDebounceFn(async () => {
  if (isLoadingConfig.value)
    return

  try {
    await cameraMonitor.updateConfig(localConfig.value)
    toast.success('Config updated')
  }
  catch (error) {
    toast.error(`Failed to auto-save config: ${error}`)
  }
}, 500)

// Handle strategy change
async function handleStrategyChange(value: string) {
  if (value === 'allDevices') {
    localConfig.value.strategy = 'allDevices'
  }
  else if (value === 'selectedDevice') {
    // If no device is selected yet, pick the first available device
    const currentSerial = (typeof localConfig.value.strategy === 'object' && 'selectedDevice' in localConfig.value.strategy)
      ? localConfig.value.strategy.selectedDevice.serialNumber
      : availableDevices.value[0]?.serial_number
    const selectedSerial = currentSerial
    if (selectedSerial) {
      localConfig.value.strategy = {
        selectedDevice: { serialNumber: selectedSerial },
      }
    }
    else {
      // If no devices available, keep it as allDevices for now
      localConfig.value.strategy = 'allDevices'
      toast.error('No devices available for selection')
      return
    }
  }

  // Auto-save after strategy change
  await autoSaveConfig()
}

// Handle device selection
async function handleDeviceSelect(serialNumber: any) {
  // Update the strategy if it's currently selectedDevice
  if (typeof localConfig.value.strategy === 'object' && 'selectedDevice' in localConfig.value.strategy) {
    localConfig.value.strategy = {
      selectedDevice: { serialNumber },
    }
  }
  await autoSaveConfig()
}

// Handle autostart toggle
async function handleToggleAutostart(checked: boolean) {
  try {
    if (checked) {
      await enable()
      toast.success('Autostart enabled')
    }
    else {
      await disable()
      toast.success('Autostart disabled')
    }
    autostartEnabled.value = checked
  }
  catch (error: any) {
    toast.error(`Failed to ${checked ? 'enable' : 'disable'} autostart: ${error.message}`)
    // Check the actual state on error
    autostartEnabled.value = await isEnabled()
  }
}

// Handle camera auto-toggle switch
async function handleToggleAutoToggle(checked: boolean) {
  try {
    localConfig.value.enabled = checked
    await cameraMonitor.updateConfig({ enabled: checked })

    if (checked) {
      await cameraMonitor.startMonitoring()
      toast.success('Camera auto-toggle enabled and monitoring started')
    }
    else {
      await cameraMonitor.stopMonitoring()
      toast.success('Camera auto-toggle disabled and monitoring stopped')
    }
  }
  catch (error: any) {
    // Revert the local config change on error
    localConfig.value.enabled = !checked
    toast.error(error.message || `Failed to ${checked ? 'enable' : 'disable'} camera auto-toggle`)
  }
}

// Debug functions
async function refreshDebugInfo() {
  debugInfo.value.isLoading = true
  try {
    await cameraMonitor.refreshStatus()
    debugInfo.value.rawStatus = {
      isMonitoring: cameraMonitor.state.value.isMonitoring,
      deviceCount: cameraMonitor.state.value.deviceCount,
      controlledDevices: cameraMonitor.state.value.controlledDevices,
      config: cameraMonitor.config.value,
    }
    debugInfo.value.lastRefresh = new Date()
  }
  catch (error) {
    console.error('Debug refresh failed:', error)
  }
  finally {
    debugInfo.value.isLoading = false
  }
}

// Test camera monitoring
async function testCameraDetection() {
  try {
    toast.success('Testing camera detection... Check console for logs')

    // Basic test - try to start monitoring
    if (!cameraMonitor.isMonitoring.value) {
      await cameraMonitor.startMonitoring()
    }

    // Show debug info
    await refreshDebugInfo()
  }
  catch (error) {
    toast.error(`Test failed: ${error}`)
  }
}

// Debug system
async function debugSystem() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const debugInfoRes = await invoke<string>('debug_camera_system')
    toast.success('Debug info logged to console')

    // Also show in the debug panel
    debugInfo.value.rawStatus.debugInfo = debugInfoRes
    await refreshDebugInfo()
  }
  catch (error) {
    toast.error(`Debug failed: ${error}`)
  }
}

// Status indicators
const monitoringStatus = computed(() => {
  if (cameraMonitor.isMonitoring.value) {
    return {
      text: 'Active',
      color: 'bg-emerald-500',
      badge: 'default',
    }
  }
  else if (localConfig.value.enabled) {
    return {
      text: 'Configured',
      color: 'bg-blue-500',
      badge: 'secondary',
    }
  }
  else {
    return {
      text: 'Disabled',
      color: 'bg-gray-500',
      badge: 'outline',
    }
  }
})

// Helper functions for monitoring status
function getMonitoringStatusClass() {
  if (cameraMonitor.state.value.isLoading) {
    return 'bg-blue-500 animate-pulse'
  }
  else if (localConfig.value.enabled && cameraMonitor.isMonitoring.value) {
    return 'bg-emerald-500 animate-pulse'
  }
  else if (localConfig.value.enabled && !cameraMonitor.isMonitoring.value) {
    return 'bg-yellow-500'
  }
  else {
    return 'bg-gray-400'
  }
}

function getMonitoringStatusText() {
  if (cameraMonitor.state.value.isLoading) {
    return 'Starting monitoring...'
  }
  else if (localConfig.value.enabled && cameraMonitor.isMonitoring.value) {
    return 'Monitoring active'
  }
  else if (localConfig.value.enabled && !cameraMonitor.isMonitoring.value) {
    return 'Enabled but failed to start monitoring (check permissions)'
  }
  else {
    return 'Disabled'
  }
}

// Watch for config changes and auto-save
watch(
  () => localConfig.value,
  async (_, oldConfig) => {
    // Skip auto-save if we're loading config or if it's the initial load
    if (isLoadingConfig.value || !oldConfig)
      return

    // Auto-save when config changes
    await autoSaveConfig()
  },
  { deep: true },
)

// Initialize debug info and config loading
onMounted(async () => {
  isLoadingConfig.value = true

  // Wait for the composable to load config from backend first
  await new Promise(resolve => setTimeout(resolve, 100))

  // Sync local config with composable config
  localConfig.value = { ...cameraMonitor.config.value }

  // Check autostart status
  try {
    autostartEnabled.value = await isEnabled()
  }
  catch (error) {
    console.error('Failed to check autostart status:', error)
  }

  await refreshDebugInfo()
  isLoadingConfig.value = false
})
</script>

<template>
  <div class="h-screen bg-background flex flex-col">
    <!-- Fixed Headers -->
    <CustomTitlebar title="Settings" class="flex-shrink-0" />
    <header class="bg-card border-b border-border/50 shadow-sm flex-shrink-0">
      <div class="max-w-7xl mx-auto px-4 py-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-4">
            <Button
              variant="ghost"
              size="sm"
              @click="router.back()"
            >
              <ArrowLeft class="w-4 h-4 mr-2" />
              Back
            </Button>

            <div class="flex items-center gap-3">
              <div class="p-2 bg-primary/10 rounded-lg">
                <Settings class="w-5 h-5 text-primary" />
              </div>
              <div>
                <h1 class="text-xl font-semibold">
                  Settings
                </h1>
                <p class="text-sm text-muted-foreground">
                  Configure Litra Control application settings
                </p>
              </div>
            </div>
          </div>

          <Badge :variant="monitoringStatus.badge as any">
            <div
              class="w-2 h-2 rounded-full mr-2"
              :class="monitoringStatus.color"
            />
            {{ monitoringStatus.text }}
          </Badge>
        </div>
      </div>
    </header>

    <!-- Scrollable Main Content -->
    <main class="flex-1 overflow-y-auto">
      <div class="max-w-7xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Configuration Panel -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Status Overview -->
            <Card>
              <CardHeader>
                <CardTitle class="flex items-center gap-2">
                  <Monitor class="w-5 h-5" />
                  System Status
                </CardTitle>
                <CardDescription>
                  Current monitoring status and detected devices
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <div class="text-center p-3 bg-muted/50 rounded-lg">
                    <div class="text-2xl font-bold text-primary">
                      {{ cameraMonitor.state.value.deviceCount }}
                    </div>
                    <div class="text-sm text-muted-foreground">
                      Camera Devices
                    </div>
                  </div>
                  <div class="text-center p-3 bg-muted/50 rounded-lg">
                    <div class="text-2xl font-bold text-emerald-600">
                      {{ cameraMonitor.controlledDeviceCount.value }}
                    </div>
                    <div class="text-sm text-muted-foreground">
                      Controlled
                    </div>
                  </div>
                  <div class="text-center p-3 bg-muted/50 rounded-lg">
                    <div class="text-2xl font-bold text-blue-600">
                      {{ availableDevices.length }}
                    </div>
                    <div class="text-sm text-muted-foreground">
                      Litra Devices
                    </div>
                  </div>
                  <div class="text-center p-3 bg-muted/50 rounded-lg">
                    <div
                      class="text-2xl font-bold"
                      :class="cameraMonitor.isMonitoring.value ? 'text-emerald-600' : 'text-gray-500'"
                    >
                      {{ cameraMonitor.isMonitoring.value ? 'ON' : 'OFF' }}
                    </div>
                    <div class="text-sm text-muted-foreground">
                      Monitoring
                    </div>
                  </div>
                </div>

                <div class="flex gap-2 mt-4">
                  <Button
                    variant="outline"
                    size="sm"
                    :disabled="cameraMonitor.state.value.isLoading"
                    class="cursor-pointer"
                    @click="refreshDebugInfo"
                  >
                    <RefreshCw class="w-4 h-4 mr-2" />
                    Refresh
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    class="cursor-pointer"
                    @click="testCameraDetection"
                  >
                    <Video class="w-4 h-4 mr-2" />
                    Test Detection
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    class="cursor-pointer"
                    @click="debugSystem"
                  >
                    Debug System
                  </Button>
                </div>
              </CardContent>
            </Card>

            <!-- General Settings -->
            <Card>
              <CardHeader>
                <CardTitle>General Settings</CardTitle>
                <CardDescription>
                  Application preferences
                </CardDescription>
              </CardHeader>
              <CardContent class="space-y-4">
                <div class="space-y-3">
                  <div>
                    <Label class="text-base font-medium">Theme</Label>
                    <p class="text-sm text-muted-foreground mt-1">
                      Choose your preferred color scheme
                    </p>
                  </div>
                  <div class="grid grid-cols-2 gap-2">
                    <button
                      v-for="theme in themeOptions"
                      :key="theme.value"
                      class="relative flex flex-col items-center gap-2 p-3 rounded-lg border transition-all cursor-pointer"
                      :class="
                        colorMode === theme.value
                        ? 'bg-primary/10 border-primary text-primary'
                        : 'bg-muted/30 border-border hover:bg-muted/50 hover:border-border/80'
                      "
                      @click="colorMode = theme.value as any"
                    >
                      <component
                        :is="theme.icon"
                        class="w-5 h-5"
                      />
                      <span class="text-xs font-medium">{{ theme.label }}</span>
                    </button>
                  </div>
                </div>

                <Separator />

                <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
                  <div>
                    <Label class="text-base font-medium">Start at Login</Label>
                    <p class="text-sm text-muted-foreground mt-1">
                      Automatically start Litra Control when you log in
                    </p>
                  </div>
                  <Switch
                    class="cursor-pointer"
                    :model-value="autostartEnabled"
                    @update:model-value="handleToggleAutostart"
                  />
                </div>
              </CardContent>
            </Card>

            <!-- Camera Auto-Toggle Section -->
            <Card>
              <CardHeader>
                <CardTitle class="flex items-center gap-2">
                  <Camera class="w-5 h-5" />
                  Camera Auto-Toggle
                </CardTitle>
                <CardDescription>
                  Automatically control Litra devices when camera activity is
                  detected
                </CardDescription>
              </CardHeader>
              <CardContent class="space-y-4">
                <div class="flex items-center justify-between p-4 bg-muted/50 rounded-lg">
                  <div>
                    <Label class="text-base font-medium"
                    >Enable Camera Auto-Toggle</Label>
                    <p class="text-sm text-muted-foreground mt-1">
                      Turn on lights when camera is detected, turn off when no
                      camera activity
                    </p>
                  </div>
                  <Switch
                    class="cursor-pointer"
                    :model-value="localConfig.enabled"
                    :disabled="cameraMonitor.state.value.isLoading"
                    @update:model-value="handleToggleAutoToggle"
                  />
                </div>

                <!-- Status indicator -->
                <div class="flex items-center gap-2 text-sm">
                  <div
                    class="w-2 h-2 rounded-full"
                    :class="getMonitoringStatusClass()"
                  />
                  <span class="text-muted-foreground">
                    {{ getMonitoringStatusText() }}
                  </span>
                </div>
              </CardContent>
            </Card>

            <!-- Strategy Selection -->
            <Card>
              <CardHeader>
                <CardTitle>Control Strategy</CardTitle>
                <CardDescription>
                  Choose which devices to control automatically
                </CardDescription>
              </CardHeader>
              <CardContent class="space-y-4">
                <Select
                  :model-value="typeof localConfig.strategy === 'string' ? localConfig.strategy : 'selectedDevice'"
                  class="cursor-pointer"
                  @update:model-value="(value: any) => handleStrategyChange(value)"
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select strategy">
                      {{
                        strategies.find(s => s.value === (typeof localConfig.strategy === 'string' ? localConfig.strategy : 'selectedDevice'))?.label || 'Select strategy'
                      }}
                    </SelectValue>
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem
                      v-for="strategy in strategies"
                      :key="strategy.value"
                      :value="strategy.value"
                    >
                      <div>
                        <div class="font-medium">
                          {{ strategy.label }}
                        </div>
                        <div class="text-sm text-muted-foreground">
                          {{ strategy.description }}
                        </div>
                      </div>
                    </SelectItem>
                  </SelectContent>
                </Select>

                <!-- Selected Device Configuration -->
                <div
                  v-if="typeof localConfig.strategy === 'object' && 'selectedDevice' in localConfig.strategy"
                  class="space-y-3"
                >
                  <Label>Select Device</Label>
                  <Select
                    :model-value="(typeof localConfig.strategy === 'object' && 'selectedDevice' in localConfig.strategy) ? localConfig.strategy.selectedDevice.serialNumber : ''"
                    class="cursor-pointer"
                    @update:model-value="(value: any) => handleDeviceSelect(value)"
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Choose a device" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem
                        v-for="deviceItem in availableDevices"
                        :key="deviceItem.serial_number"
                        :value="deviceItem.serial_number"
                      >
                        {{ deviceItem.device_type }} ({{
                          deviceItem.serial_number
                        }})
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </CardContent>
            </Card>

            <!-- Advanced Settings -->
            <Card>
              <CardHeader>
                <CardTitle>Advanced Settings</CardTitle>
                <CardDescription>
                  Fine-tune monitoring behavior
                </CardDescription>
              </CardHeader>
              <CardContent class="space-y-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="space-y-2">
                    <Label for="debounce">Debounce Time (ms)</Label>
                    <Input
                      id="debounce"
                      type="number"
                      :model-value="localConfig.debounceMs"
                      min="500"
                      max="10000"
                      step="100"
                      @update:model-value="(value: any) => localConfig.debounceMs = parseInt(String(value)) || 3000"
                    />
                    <p class="text-xs text-muted-foreground">
                      Delay before turning off devices when no cameras are
                      detected
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          <!-- Debug & Control Panel -->
          <div class="space-y-6">
            <!-- Debug Info -->
            <Card>
              <CardHeader>
                <CardTitle>Debug Information</CardTitle>
                <CardDescription>
                  Technical details for troubleshooting
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div class="space-y-3 text-sm">
                  <div>
                    <strong>Last Refresh:</strong>
                    <div class="text-muted-foreground">
                      {{
                        debugInfo.lastRefresh ? debugInfo.lastRefresh.toLocaleTimeString() : 'Never'
                      }}
                    </div>
                  </div>

                  <Separator />

                  <div>
                    <strong>Current Config:</strong>
                    <pre class="text-xs bg-muted/50 p-2 rounded mt-1 overflow-x-auto">{{ JSON.stringify(localConfig, null, 2) }}</pre>
                  </div>

                  <div>
                    <strong>Runtime Status:</strong>
                    <pre class="text-xs bg-muted/50 p-2 rounded mt-1 overflow-x-auto">{{ JSON.stringify(debugInfo.rawStatus, null, 2) }}</pre>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
