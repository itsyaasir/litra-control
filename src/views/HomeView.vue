<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { Edit3, Lightbulb, Palette, RefreshCw, Settings, Sun, Video, Zap } from 'lucide-vue-next'
import { computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { toast } from 'vue-sonner'

import CustomSlider from '@/components/CustomSlider.vue'
import CustomTitlebar from '@/components/CustomTitlebar.vue'
// Components
import { Button } from '@/components/ui/button'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Switch } from '@/components/ui/switch'
import { useCameraMonitor, useDevice, usePresets } from '@/composables'

const router = useRouter()
const device = useDevice()
const presets = usePresets()
const cameraMonitor = useCameraMonitor()

// Get selected device
const selectedDevice = computed(() => device.selectedDevice.value)

// Handle device selection from dropdown
function handleDeviceSelect(value: any) {
  if (value && typeof value === 'string') {
    device.selectDevice(value)
  }
}

// Handle device operations
async function handlePowerToggle(checked: boolean) {
  try {
    if (selectedDevice.value) {
      await device.setPowerState(checked, selectedDevice.value.serial_number)
      toast.success(`Device ${checked ? 'turned on' : 'turned off'} successfully`)
    }
  }
  catch (error: any) {
    toast.error(error.message || `Failed to ${checked ? 'turn on' : 'turn off'} device`)
  }
}

async function handleBrightnessChange(value: number) {
  try {
    if (selectedDevice.value) {
      await device.setBrightness(value, selectedDevice.value.serial_number)
    }
  }
  catch (error: any) {
    toast.error(error.message || 'Failed to set brightness')
  }
}

async function handleTemperatureChange(value: number) {
  try {
    if (selectedDevice.value) {
      await device.setTemperature(value, selectedDevice.value.serial_number)
    }
  }
  catch (error: any) {
    toast.error(error.message || 'Failed to set temperature')
  }
}

// Handle device refresh
async function handleRefreshDevices() {
  try {
    await device.refreshDevices()
    toast.success('Devices refreshed successfully')
  }
  catch (error: any) {
    toast.error(error.message || 'Failed to refresh devices')
  }
}

// Handle preset application
async function handleApplyPreset(preset: any) {
  try {
    await presets.applyPreset(preset)
    toast.success(`Applied preset: ${preset.name}`)
  }
  catch (error: any) {
    toast.error(error.message || `Failed to apply preset: ${preset.name}`)
  }
}

// Calculate percentage values
const brightnessPercentage = computed(() => {
  if (!selectedDevice.value)
    return 0
  const { brightness_lumens, min_brightness_lumens, max_brightness_lumens } = selectedDevice.value
  return Math.round(((brightness_lumens - min_brightness_lumens) / (max_brightness_lumens - min_brightness_lumens)) * 100)
})

// Get gradient style for preset based on temperature and brightness
function getPresetGradientStyle(preset: any) {
  // Calculate position on temperature gradient (2700K to 6500K range)
  const tempMin = 2700
  const tempMax = 6500
  const tempPercent = ((preset.temperature - tempMin) / (tempMax - tempMin)) * 100

  // Brightness affects opacity and gradient darkness
  const brightness = preset.brightness / 100

  // Create gradient based on temperature position
  let gradientColor = ''
  if (tempPercent <= 25) {
    // Warm (orange to yellow)
    gradientColor = `linear-gradient(135deg,
      rgba(251, 146, 60, ${brightness}) 0%,
      rgba(254, 215, 170, ${brightness * 0.8}) 100%)`
  }
  else if (tempPercent <= 50) {
    // Neutral warm (yellow to white)
    gradientColor = `linear-gradient(135deg,
      rgba(254, 240, 138, ${brightness}) 0%,
      rgba(255, 255, 255, ${brightness * 0.9}) 100%)`
  }
  else if (tempPercent <= 75) {
    // Neutral cool (white to light blue)
    gradientColor = `linear-gradient(135deg,
      rgba(255, 255, 255, ${brightness}) 0%,
      rgba(186, 230, 253, ${brightness * 0.9}) 100%)`
  }
  else {
    // Cool (light blue to blue)
    gradientColor = `linear-gradient(135deg,
      rgba(186, 230, 253, ${brightness}) 0%,
      rgba(96, 165, 250, ${brightness * 0.8}) 100%)`
  }

  return {
    background: gradientColor,
    boxShadow: `0 4px 12px rgba(0, 0, 0, ${0.1 + (1 - brightness) * 0.2})`,
  }
}

// Initialize the app
let unlistenDeviceRefresh: (() => void) | null = null

onMounted(async () => {
  // Close dropdown when clicking outside
  const handleClickOutside = (event: MouseEvent) => {
    const dropdown = document.querySelector('.preset-dropdown')
    if (dropdown && !dropdown.contains(event.target as Node)) {
      presets.showPresetDropdown.value = false
    }
  }

  document.addEventListener('click', handleClickOutside)

  // Listen for device refresh events from tray menu
  unlistenDeviceRefresh = await listen('device-refresh', () => {
    // Refresh device states to update UI
    device.refreshDevices()
  })

  // Cleanup
  return () => {
    document.removeEventListener('click', handleClickOutside)
  }
})

onUnmounted(() => {
  // Clean up device refresh event listener
  if (unlistenDeviceRefresh) {
    unlistenDeviceRefresh()
  }
})
</script>

<template>
  <div class="h-screen bg-background text-foreground transition-colors flex flex-col overflow-hidden">
    <!-- Custom Titlebar -->
    <CustomTitlebar />

    <!-- Top Header -->
    <header class="bg-card border-b border-border/50 shadow-sm flex-shrink-0">
      <div class="px-4 py-3 flex items-center justify-between">
        <!-- Left: Device Selection -->
        <div class="flex items-center gap-3">
          <!-- Device Selector -->
          <div class="flex items-center gap-2">
            <span class="text-sm text-muted-foreground font-medium"
            >Device:</span>
            <Select
              :model-value="selectedDevice?.serial_number || ''"
              @update:model-value="handleDeviceSelect"
            >
              <SelectTrigger
                class="w-44 text-sm bg-muted/50 border-input/50 hover:bg-muted hover:border-input transition-colors"
              >
                <SelectValue placeholder="Select Device" />
              </SelectTrigger>
              <SelectContent class="bg-popover border-border">
                <SelectItem
                  v-for="deviceItem in device.devices.value"
                  :key="deviceItem.serial_number"
                  :value="deviceItem.serial_number"
                  class="text-popover-foreground hover:bg-accent hover:text-accent-foreground"
                >
                  {{ deviceItem.device_type }}
                </SelectItem>
              </SelectContent>
            </Select>
          </div>

          <!-- Status -->
          <div class="flex items-center gap-2 text-sm">
            <div class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse" />
            <span class="text-muted-foreground">{{
                device.deviceStats.value.connected
              }} connected</span>
          </div>
        </div>

        <!-- Right: Controls -->
        <div class="flex items-center gap-3">
          <!-- Camera Monitor Status -->
          <div
            v-if="cameraMonitor.isMonitoring.value"
            class="flex items-center gap-2 text-sm"
          >
            <div class="w-2 h-2 bg-blue-500 rounded-full animate-pulse" />
            <span class="text-muted-foreground">Camera monitoring</span>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-1.5">
            <Button
              variant="ghost"
              size="icon"
              class="text-muted-foreground hover:text-foreground transition-colors bg-muted/30 hover:bg-muted/50 border border-border/30 hover:border-border/50 shadow-sm"
              title="Settings"
              @click="router.push('/settings')"
            >
              <Settings class="w-4 h-4" />
            </Button>
            <Button
              :disabled="device.isRefreshing.value"
              variant="ghost"
              size="icon"
              class="text-muted-foreground hover:text-foreground transition-colors bg-muted/30 hover:bg-muted/50 border border-border/30 hover:border-border/50 shadow-sm"
              title="Refresh devices"
              @click="handleRefreshDevices"
            >
              <RefreshCw
                class="w-4 h-4"
                :class="
                  [
                    device.isRefreshing.value ? 'animate-spin' : '',
                  ]
                "
              />
            </Button>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 p-4 overflow-y-auto">
      <div class="max-w-7xl mx-auto space-y-4">
        <!-- Device Info Card -->
        <div
          v-if="selectedDevice"
          class="bg-card/50 backdrop-blur-sm rounded-lg p-3 border border-border/50"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="p-1.5 bg-gradient-to-br from-primary to-primary/80 rounded-lg shadow-sm">
                <Lightbulb class="w-4 h-4 text-primary-foreground" />
              </div>
              <div>
                <h2 class="font-semibold text-base">
                  {{ selectedDevice.device_type }}
                </h2>
                <p class="text-xs text-muted-foreground">
                  {{ selectedDevice.serial_number }}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-4">
              <div class="flex items-center gap-2">
                <div
                  class="w-2 h-2 rounded-full"
                  :class="selectedDevice.is_connected ? 'bg-emerald-500' : 'bg-red-500'"
                />
                <span class="text-sm text-muted-foreground">{{
                  selectedDevice.is_connected ? 'Connected' : 'Disconnected'
                }}</span>
              </div>
              <div class="flex items-center gap-2">
                <div
                  class="w-2 h-2 rounded-full"
                  :class="selectedDevice.is_on ? 'bg-emerald-500 animate-pulse' : 'bg-gray-500'"
                />
                <span class="text-sm font-medium">{{
                  selectedDevice.is_on ? 'ON' : 'OFF'
                }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Controls Layout -->
        <div class="space-y-6">
          <!-- Top Row: Power, Temperature, Brightness -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <!-- Power Control -->
            <div class="bg-card/50 backdrop-blur-sm rounded-lg p-4 border border-border/50">
              <h3 class="text-base font-semibold mb-3 flex items-center gap-2">
                <Zap class="w-4 h-4 text-primary" />
                Power Control
              </h3>
              <div class="space-y-3">
                <div class="flex items-center justify-between">
                  <span class="text-sm text-muted-foreground">Power State</span>
                  <Switch
                    :model-value="selectedDevice?.is_on || false"
                    :disabled="!selectedDevice?.is_connected || device.isPowerChanging.value"
                    class="data-[state=checked]:bg-primary data-[state=unchecked]:bg-muted shadow-sm"
                    @update:model-value="handlePowerToggle"
                  />
                </div>
                <div class="flex items-center justify-between p-2 bg-muted/20 rounded-lg border border-border/30">
                  <div class="flex items-center gap-2">
                    <div class="p-1 bg-primary/10 rounded-md">
                      <Video class="w-3 h-3 text-primary" />
                    </div>
                    <div>
                      <label
                        for="camera-activate"
                        class="text-sm font-medium text-foreground cursor-pointer"
                      >
                        Auto-activate with camera
                      </label>
                      <p class="text-xs text-muted-foreground">
                        Turn on device when camera is detected
                      </p>
                    </div>
                  </div>
                  <Switch
                    id="camera-activate"
                    :model-value="cameraMonitor.isEnabled.value"
                    :disabled="cameraMonitor.state.value.isLoading"
                    class="data-[state=checked]:bg-primary data-[state=unchecked]:bg-muted shadow-sm"
                    @update:model-value="
                      async (checked) => {
                        if (checked) {
                          await cameraMonitor.startMonitoring()
                        }
                        else {
                          await cameraMonitor.stopMonitoring()
                        }
                        await cameraMonitor.updateConfig({ enabled: checked })
                      }
                    "
                  />
                </div>
              </div>
            </div>

            <!-- Temperature Control -->
            <div
              v-if="selectedDevice"
              class="bg-card/50 backdrop-blur-sm rounded-lg p-4 border border-border/50"
            >
              <h3 class="text-base font-semibold mb-3 flex items-center gap-2">
                <Sun class="w-4 h-4 text-primary" />
                Temperature
              </h3>
              <div class="space-y-3">
                <div class="text-center">
                  <span class="text-2xl font-bold text-primary">{{
                      selectedDevice.temperature_kelvin
                    }}K</span>
                  <p class="text-xs text-muted-foreground">
                    Color Temperature
                  </p>
                </div>
                <div class="px-2">
                  <CustomSlider
                    :model-value="selectedDevice.temperature_kelvin"
                    :min="selectedDevice.min_temperature_kelvin"
                    :max="selectedDevice.max_temperature_kelvin"
                    :step="100"
                    :disabled="!selectedDevice.is_on || device.isTemperatureChanging.value"
                    gradient-type="temperature"
                    @update:model-value="handleTemperatureChange"
                  />
                </div>
                <div class="flex justify-between text-xs text-muted-foreground">
                  <span>Warm</span>
                  <span>Cool</span>
                </div>
              </div>
            </div>

            <!-- Brightness Control -->
            <div
              v-if="selectedDevice"
              class="bg-card/50 backdrop-blur-sm rounded-lg p-4 border border-border/50"
            >
              <h3 class="text-base font-semibold mb-3 flex items-center gap-2">
                <Lightbulb class="w-4 h-4 text-primary" />
                Brightness
              </h3>
              <div class="space-y-3">
                <div class="text-center">
                  <span class="text-2xl font-bold text-primary">{{
                      brightnessPercentage
                    }}%</span>
                  <p class="text-xs text-muted-foreground">
                    Light Intensity
                  </p>
                </div>
                <div class="px-2">
                  <CustomSlider
                    :model-value="selectedDevice.brightness_lumens"
                    :min="selectedDevice.min_brightness_lumens"
                    :max="selectedDevice.max_brightness_lumens"
                    :step="1"
                    :disabled="!selectedDevice.is_on || device.isBrightnessChanging.value"
                    gradient-type="brightness"
                    @update:model-value="handleBrightnessChange"
                  />
                </div>
                <div class="flex justify-between text-xs text-muted-foreground">
                  <span>Dim</span>
                  <span>Bright</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Bottom Row: Presets -->
          <div class="bg-card/50 backdrop-blur-sm rounded-lg p-6 border border-border/50">
            <h3 class="text-lg font-semibold mb-6 flex items-center gap-2">
              <Palette class="w-5 h-5 text-primary" />
              Lighting Presets
            </h3>

            <!-- Current Preset Display -->
            <div class="bg-gradient-to-r from-primary/10 to-primary/5 rounded-lg p-4 border border-primary/20 mb-6">
              <div class="flex items-center gap-2 mb-2">
                <div class="w-2 h-2 bg-primary rounded-full animate-pulse" />
                <span class="text-sm font-medium text-primary">
                  Active Preset
                </span>
              </div>
              <div class="text-base font-semibold">
                {{
                  presets.currentPreset.value === 'manual' ? 'Manual Control' : presets.getPresetById(presets.currentPreset.value)?.name || 'Manual Control'
                }}
              </div>
              <div
                v-if="presets.currentPreset.value !== 'manual'"
                class="text-sm text-muted-foreground mt-1"
              >
                {{
                  presets.getPresetById(presets.currentPreset.value)?.description
                }}
              </div>
            </div>

            <!-- Preset Selection Grid -->
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
              <!-- Manual Control -->
              <button
                class="flex flex-col items-center gap-2 p-4 rounded-lg border border-border/50 hover:border-border transition-colors"
                :class="presets.currentPreset.value === 'manual' ? 'bg-primary/10 border-primary/30' : 'bg-muted/20 hover:bg-muted/30'"
                @click="presets.setManualMode()"
              >
                <Edit3
                  class="w-6 h-6"
                  :class="presets.currentPreset.value === 'manual' ? 'text-primary' : 'text-muted-foreground'"
                />
                <span
                  class="text-sm font-medium text-center"
                  :class="presets.currentPreset.value === 'manual' ? 'text-primary' : 'text-foreground'"
                >
                  Manual
                </span>
              </button>

              <!-- Preset Buttons -->
              <button
                v-for="preset in presets.presets"
                :key="preset.id"
                class="flex flex-col items-center gap-2 p-4 rounded-lg border border-border/50 hover:border-border transition-colors"
                :class="presets.currentPreset.value === preset.id ? 'bg-primary/10 border-primary/30' : 'bg-muted/20 hover:bg-muted/30'"
                @click="handleApplyPreset(preset)"
              >
                <div
                  class="w-6 h-6 rounded-full border border-border/30 relative overflow-hidden"
                  :style="getPresetGradientStyle(preset)"
                />
                <span
                  class="text-sm font-medium text-center leading-tight"
                  :class="presets.currentPreset.value === preset.id ? 'text-primary' : 'text-foreground'"
                >
                  {{ preset.name.split(' ')[0] }}
                </span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- Loading State -->
    <div
      v-if="device.isDiscovering.value"
      class="fixed inset-0 bg-background/50 backdrop-blur-sm flex items-center justify-center z-50"
    >
      <div class="text-center space-y-3 bg-card/80 backdrop-blur-sm p-6 rounded-lg border border-border/50 shadow-lg">
        <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto" />
        <p class="text-sm text-muted-foreground">
          Discovering devices...
        </p>
      </div>
    </div>

    <!-- No Devices State -->
    <div
      v-if="!device.isDiscovering.value && device.devices.value.length === 0"
      class="fixed inset-0 bg-background/50 backdrop-blur-xl flex items-center justify-center z-50"
    >
      <div class="text-center space-y-4 max-w-md p-8 bg-card/80 backdrop-blur-sm rounded-lg border border-border/50 shadow-lg">
        <div class="p-6 bg-gradient-to-br from-primary/10 to-primary/5 rounded-full w-fit mx-auto">
          <Lightbulb class="w-16 h-16 text-primary" />
        </div>
        <div>
          <h3 class="text-lg font-medium text-foreground">
            No devices found
          </h3>
          <p class="text-sm mt-2 text-muted-foreground">
            Make sure your Litra devices are connected via USB and try
            refreshing.
          </p>
        </div>
        <Button
          class="bg-primary hover:bg-primary/90 text-primary-foreground shadow-sm"
          @click="handleRefreshDevices"
        >
          Refresh Devices
        </Button>
      </div>
    </div>
  </div>
</template>
