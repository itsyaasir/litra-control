<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core'
import { Edit3, Lightbulb, Moon, Palette, RefreshCw, Sun, Video, Zap } from 'lucide-vue-next'
import { computed, onMounted } from 'vue'

import CustomSlider from './components/CustomSlider.vue'
import CustomTitlebar from './components/CustomTitlebar.vue'
// Components
import { Button } from './components/ui/button'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './components/ui/select'
import { Switch } from './components/ui/switch'
import { useDevice, usePresets } from './composables'

const device = useDevice()
const presets = usePresets()

// Theme management with VueUse
const isDark = useDark()
const toggleDark = useToggle(isDark)

// Get selected device
const selectedDevice = computed(() => device.selectedDevice.value)

// Handle device selection from dropdown
function handleDeviceSelect(value: any) {
  if (value && typeof value === 'string') {
    device.selectDevice(value)
  }
}

// Handle device operations
function handlePowerToggle(checked: boolean) {
  if (selectedDevice.value) {
    device.setPowerState(checked, selectedDevice.value.serial_number)
  }
}

function handleBrightnessChange(value: number) {
  if (selectedDevice.value) {
    device.setBrightness(value, selectedDevice.value.serial_number)
  }
}

function handleTemperatureChange(value: number) {
  if (selectedDevice.value) {
    device.setTemperature(value, selectedDevice.value.serial_number)
  }
}

// Calculate percentage values
const brightnessPercentage = computed(() => {
  if (!selectedDevice.value)
    return 0
  const { brightness_lumens, min_brightness_lumens, max_brightness_lumens } = selectedDevice.value
  return Math.round(((brightness_lumens - min_brightness_lumens) / (max_brightness_lumens - min_brightness_lumens)) * 100)
})

// Convert color temperature to RGB values
function kelvinToRGB(kelvin: number) {
  const temp = kelvin / 100
  let red, green, blue

  if (temp <= 66) {
    red = 255
    green = temp
    green = 99.4708025861 * Math.log(green) - 161.1195681661

    if (temp >= 19) {
      blue = temp - 10
      blue = 138.5177312231 * Math.log(blue) - 305.0447927307
    }
    else {
      blue = 0
    }
  }
  else {
    red = temp - 60
    red = 329.698727446 * red ** -0.1332047592

    green = temp - 60
    green = 288.1221695283 * green ** -0.0755148492

    blue = 255
  }

  return {
    r: Math.max(0, Math.min(255, red)),
    g: Math.max(0, Math.min(255, green)),
    b: Math.max(0, Math.min(255, blue)),
  }
}

// Get color representation for a preset
function getPresetColor(preset: any) {
  const rgb = kelvinToRGB(preset.temperature)
  const brightness = preset.brightness / 100

  // Apply brightness scaling
  const r = Math.round(rgb.r * brightness)
  const g = Math.round(rgb.g * brightness)
  const b = Math.round(rgb.b * brightness)

  return `rgb(${r}, ${g}, ${b})`
}

// Initialize the app
onMounted(() => {
  // Close dropdown when clicking outside
  const handleClickOutside = (event: MouseEvent) => {
    const dropdown = document.querySelector('.preset-dropdown')
    if (dropdown && !dropdown.contains(event.target as Node)) {
      presets.showPresetDropdown.value = false
    }
  }

  document.addEventListener('click', handleClickOutside)

  // Cleanup
  return () => {
    document.removeEventListener('click', handleClickOutside)
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
            <span class="text-sm text-muted-foreground font-medium">Device:</span>
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
          <!-- Actions -->
          <div class="flex items-center gap-1.5">
            <Button
              variant="ghost"
              size="icon"
              class="text-muted-foreground hover:text-foreground hover:bg-primary/10 transition-colors bg-muted/30 hover:bg-muted/50 border border-border/30 hover:border-border/50 shadow-sm"
              title="Toggle theme"
              @click="toggleDark()"
            >
              <Sun v-if="isDark" class="w-4 h-4" />
              <Moon v-else class="w-4 h-4" />
            </Button>
            <Button
              :disabled="device.isRefreshing.value"
              variant="ghost"
              size="icon"
              class="text-muted-foreground hover:text-foreground hover:bg-primary/10 transition-colors bg-muted/30 hover:bg-muted/50 border border-border/30 hover:border-border/50 shadow-sm"
              title="Refresh devices"
              @click="device.refreshDevices"
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
                    class="data-[state=checked]:bg-primary data-[state=unchecked]:bg-muted shadow-sm"
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
                @click="presets.applyPreset(preset)"
              >
                <div
                  class="w-6 h-6 rounded-full border border-border/30"
                  :style="
                    {
                      backgroundColor: getPresetColor(preset),
                      boxShadow: `0 0 12px ${getPresetColor(preset)}40`,
                    }
                  "
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
      class="fixed inset-0 bg-background/50 backdrop-blur-sm flex items-center justify-center z-50"
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
          @click="device.refreshDevices"
        >
          Refresh Devices
        </Button>
      </div>
    </div>

    <!-- Error Messages -->
    <div
      v-if="device.hasError.value"
      class="fixed bottom-4 right-4 left-4 md:left-auto max-w-sm md:max-w-sm rounded-lg p-4 shadow-lg backdrop-blur-sm border bg-destructive/5 border-destructive/20 animate-in slide-in-from-bottom-2"
    >
      <div class="flex items-start gap-3">
        <div class="w-5 h-5 flex-shrink-0 mt-0.5 text-destructive">
          <Zap class="w-5 h-5" />
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-destructive">
            Operation Failed
          </p>
          <p class="text-sm mt-1 text-destructive/80">
            {{
              device.localError.value || 'An error occurred while communicating with the device.'
            }}
          </p>
        </div>
        <Button
          variant="ghost"
          size="icon"
          class="h-auto w-auto p-1 text-destructive/60 hover:text-destructive"
          @click="device.clearMessages"
        >
          <span class="text-sm">×</span>
        </Button>
      </div>
    </div>

    <!-- Success Messages -->
    <div
      v-if="device.hasSuccess.value"
      class="fixed bottom-4 right-4 left-4 md:left-auto max-w-sm md:max-w-sm rounded-lg p-4 shadow-lg backdrop-blur-sm border bg-emerald-500/10 border-emerald-500/20 animate-in slide-in-from-bottom-2"
    >
      <div class="flex items-start gap-3">
        <div class="w-5 h-5 flex-shrink-0 mt-0.5 text-emerald-600 dark:text-emerald-400">
          <Lightbulb class="w-5 h-5" />
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-emerald-700 dark:text-emerald-300">
            Success
          </p>
          <p class="text-sm mt-1 text-emerald-600 dark:text-emerald-400">
            {{
              device.localSuccess.value || 'Operation completed successfully.'
            }}
          </p>
        </div>
        <Button
          variant="ghost"
          size="icon"
          class="h-auto w-auto p-1 text-emerald-500/60 hover:text-emerald-600 dark:hover:text-emerald-400"
          @click="device.clearMessages"
        >
          <span class="text-sm">×</span>
        </Button>
      </div>
    </div>
  </div>
</template>
