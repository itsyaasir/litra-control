<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core'
import { Camera, ChevronDown, Edit3, Lightbulb, Menu, Moon, RefreshCw, Settings, Sun, X, Zap } from 'lucide-vue-next'
import { computed, onMounted, ref } from 'vue'
import CameraPermissionHelp from './components/CameraPermissionHelp.vue'
import CameraPreview from './components/CameraPreview.vue'

import CustomSlider from './components/CustomSlider.vue'
// Components
import { Button } from './components/ui/button'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './components/ui/select'
import { Switch } from './components/ui/switch'
import { useCamera, useDevice } from './composables'
import { useSettingsStore } from './stores'

const device = useDevice()
const settingsStore = useSettingsStore()
const camera = useCamera()

// Theme management with VueUse
const isDark = useDark()
const toggleDark = useToggle(isDark)

// Initialize the app
onMounted(() => {
  settingsStore.initialize()
  window.addEventListener('resize', closeSidebarOnResize)
  camera.initialize()
})

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

// Sidebar toggle for mobile
const sidebarOpen = ref(false)

// Close sidebar when clicking outside on mobile
function closeSidebarOnResize() {
  if (window.innerWidth >= 768) {
    sidebarOpen.value = false
  }
}

// Listen for window resize

function toggleSidebar() {
  sidebarOpen.value = !sidebarOpen.value
}

// Camera control functions
async function startCamera() {
  await camera.startStream(camera.selectedCameraId.value)
}

function stopCamera() {
  camera.stopStream()
}

async function handleCameraChange(deviceId: any) {
  if (deviceId && typeof deviceId === 'string') {
    await camera.switchCamera(deviceId)
  }
}

// Permission help modal
const showPermissionHelp = ref(false)

async function handlePermissionRequest() {
  const hasPermission = await camera.requestPermission()
  if (hasPermission) {
    showPermissionHelp.value = false
    await camera.initialize()
  }
  else {
    showPermissionHelp.value = true
  }
}

async function handlePermissionGranted() {
  showPermissionHelp.value = false
  await camera.initialize()
}

// Camera modal state
const showCameraModal = ref(false)
</script>

<template>
  <div class="min-h-screen bg-background text-foreground transition-colors flex flex-col">
    <!-- Top Header -->
    <header class="bg-card border-b border-border/50 shadow-sm">
      <div class="px-6 py-4 flex items-center justify-between">
        <!-- Left: App Title -->
        <div class="flex items-center gap-4">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-gradient-to-br from-primary to-primary/80 rounded-lg shadow-sm">
              <Lightbulb class="w-6 h-6 text-primary-foreground" />
            </div>
            <h1 class="text-xl font-bold tracking-wider bg-gradient-to-r from-primary to-primary/80 bg-clip-text text-transparent">
              LITRA CONTROL
            </h1>
          </div>
        </div>

        <!-- Right: Controls -->
        <div class="flex items-center gap-4">
          <!-- Device Selector -->
          <div class="flex items-center gap-2">
            <span class="text-sm text-muted-foreground font-medium">Device:</span>
            <Select
              :model-value="selectedDevice?.serial_number || ''"
              @update:model-value="handleDeviceSelect"
            >
              <SelectTrigger class="w-48 text-sm bg-muted/50 border-input/50 hover:bg-muted hover:border-input transition-colors">
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
            <span class="text-muted-foreground">{{ device.deviceStats.value.connected }} connected</span>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-2">
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
              variant="ghost"
              size="icon"
              class="text-muted-foreground hover:text-foreground hover:bg-primary/10 transition-colors bg-muted/30 hover:bg-muted/50 border border-border/30 hover:border-border/50 shadow-sm"
              title="Settings"
            >
              <Settings class="w-4 h-4" />
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
                class="w-4 h-4" :class="[
                  device.isRefreshing.value ? 'animate-spin' : '',
                ]"
              />
            </Button>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 p-6 overflow-y-auto">
      <div class="max-w-7xl mx-auto">
        <!-- Device Info Card -->
        <div v-if="selectedDevice" class="bg-card/50 backdrop-blur-sm rounded-lg p-4 border border-border/50 mb-6">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="p-2 bg-gradient-to-br from-primary to-primary/80 rounded-lg shadow-sm">
                <Lightbulb class="w-5 h-5 text-primary-foreground" />
              </div>
              <div>
                <h2 class="font-semibold text-lg">
                  {{ selectedDevice.device_type }}
                </h2>
                <p class="text-sm text-muted-foreground">
                  {{ selectedDevice.serial_number }}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-4">
              <div class="flex items-center gap-2">
                <div class="w-2 h-2 rounded-full" :class="selectedDevice.is_connected ? 'bg-emerald-500' : 'bg-red-500'" />
                <span class="text-sm text-muted-foreground">{{ selectedDevice.is_connected ? 'Connected' : 'Disconnected' }}</span>
              </div>
              <div class="flex items-center gap-2">
                <div class="w-2 h-2 rounded-full" :class="selectedDevice.is_on ? 'bg-emerald-500 animate-pulse' : 'bg-gray-500'" />
                <span class="text-sm font-medium">{{ selectedDevice.is_on ? 'ON' : 'OFF' }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Controls Grid -->
        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
          <!-- Power Control -->
          <div class="bg-card/50 backdrop-blur-sm rounded-lg p-6 border border-border/50">
            <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Zap class="w-5 h-5 text-primary" />
              Power Control
            </h3>
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Power State</span>
                <Switch
                  :model-value="selectedDevice?.is_on || false"
                  :disabled="!selectedDevice?.is_connected || device.isPowerChanging.value"
                  class="data-[state=checked]:bg-primary data-[state=unchecked]:bg-muted shadow-sm"
                  @update:model-value="handlePowerToggle"
                />
              </div>
              <div class="flex items-center gap-2">
                <input
                  id="camera-activate"
                  type="checkbox"
                  class="w-4 h-4 rounded border bg-input border-border text-primary"
                >
                <label for="camera-activate" class="text-sm text-foreground">
                  Activate with camera
                </label>
              </div>
            </div>
          </div>

          <!-- Temperature Control -->
          <div v-if="selectedDevice" class="bg-card/50 backdrop-blur-sm rounded-lg p-6 border border-border/50">
            <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Sun class="w-5 h-5 text-primary" />
              Temperature
            </h3>
            <div class="space-y-4">
              <div class="text-center">
                <span class="text-3xl font-bold text-primary">{{ selectedDevice.temperature_kelvin }}K</span>
                <p class="text-sm text-muted-foreground">
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
                <span>Cool</span>
                <span>Warm</span>
              </div>
            </div>
          </div>

          <!-- Brightness Control -->
          <div v-if="selectedDevice" class="bg-card/50 backdrop-blur-sm rounded-lg p-6 border border-border/50">
            <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Lightbulb class="w-5 h-5 text-primary" />
              Brightness
            </h3>
            <div class="space-y-4">
              <div class="text-center">
                <span class="text-3xl font-bold text-primary">{{ brightnessPercentage }}%</span>
                <p class="text-sm text-muted-foreground">
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

          <!-- Camera Control -->
          <div v-if="!camera.isLinux.value" class="bg-card/50 backdrop-blur-sm rounded-lg p-6 border border-border/50">
            <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Camera class="w-5 h-5 text-primary" />
              Camera
            </h3>
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Status</span>
                <div class="flex items-center gap-2">
                  <div class="w-2 h-2 rounded-full" :class="camera.isStreaming.value ? 'bg-emerald-500 animate-pulse' : 'bg-gray-500'" />
                  <span class="text-sm font-medium">{{ camera.isStreaming.value ? 'Streaming' : 'Stopped' }}</span>
                </div>
              </div>

              <div v-if="camera.availableCameras.value.length > 0" class="space-y-2">
                <label class="text-sm text-muted-foreground">Device</label>
                <Select
                  :model-value="camera.selectedCameraId.value"
                  :disabled="camera.isStreaming.value"
                  @update:model-value="handleCameraChange"
                >
                  <SelectTrigger class="bg-input/50 border-border/50 hover:bg-input hover:border-border transition-colors">
                    <SelectValue placeholder="Select Camera" />
                  </SelectTrigger>
                  <SelectContent class="bg-popover border-border">
                    <SelectItem
                      v-for="cameraDevice in camera.availableCameras.value"
                      :key="cameraDevice.deviceId"
                      :value="cameraDevice.deviceId"
                      class="text-popover-foreground hover:bg-accent hover:text-accent-foreground"
                    >
                      {{ cameraDevice.label }}
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div class="flex gap-2">
                <Button
                  v-if="!camera.isStreaming.value"
                  size="sm"
                  class="flex-1 bg-primary hover:bg-primary/90 text-primary-foreground"
                  @click="camera.availableCameras.value.length > 0 ? startCamera() : handlePermissionRequest()"
                >
                  <Camera class="w-4 h-4 mr-2" />
                  {{ camera.availableCameras.value.length > 0 ? 'Start' : 'Enable' }}
                </Button>
                <Button
                  v-else
                  size="sm"
                  variant="outline"
                  class="flex-1 border-destructive text-destructive hover:bg-destructive hover:text-destructive-foreground"
                  @click="stopCamera"
                >
                  Stop
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  class="border-primary text-primary hover:bg-primary hover:text-primary-foreground"
                  @click="showCameraModal = true"
                >
                  Preview
                </Button>
              </div>

              <div v-if="camera.error.value" class="p-3 bg-destructive/10 border border-destructive/20 rounded-lg">
                <p class="text-sm text-destructive">
                  {{ camera.error.value }}
                </p>
              </div>
            </div>
          </div>

          <!-- Presets -->
          <div class="bg-card/50 backdrop-blur-sm rounded-lg p-6 border border-border/50">
            <h3 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Edit3 class="w-5 h-5 text-primary" />
              Presets
            </h3>
            <div class="space-y-3">
              <div class="flex items-center gap-2 p-3 bg-muted/30 rounded-lg">
                <div class="w-2 h-2 bg-primary rounded-full" />
                <span class="text-sm font-medium">Manual Control</span>
                <ChevronDown class="w-4 h-4 ml-auto text-muted-foreground" />
              </div>
              <Button variant="outline" size="sm" class="w-full">
                Save Current Settings
              </Button>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- Camera Modal -->
    <div
      v-if="showCameraModal && !camera.isLinux.value"
      class="fixed inset-0 bg-background/50 backdrop-blur-sm flex items-center justify-center z-50"
      @click="showCameraModal = false"
    >
      <div class="bg-card/90 backdrop-blur-sm rounded-lg border border-border/50 shadow-xl max-w-2xl w-full mx-4" @click.stop>
        <div class="p-4 border-b border-border/50 flex items-center justify-between">
          <h3 class="text-lg font-semibold">
            Camera Preview
          </h3>
          <Button
            variant="ghost"
            size="icon"
            class="text-muted-foreground hover:text-foreground"
            @click="showCameraModal = false"
          >
            <X class="w-4 h-4" />
          </Button>
        </div>
        <div class="p-4">
          <CameraPreview
            :video-ref="camera.videoRef"
            :is-streaming="camera.isStreaming.value"
            :error="camera.error.value"
          />
        </div>
      </div>
    </div>

    <!-- Mobile Menu Button -->
    <Button
      variant="ghost"
      size="icon"
      class="md:hidden fixed top-4 left-4 z-20 text-muted-foreground hover:text-foreground hover:bg-primary/10 transition-colors bg-card/80 backdrop-blur-sm border border-border/50 shadow-sm"
      @click="toggleSidebar"
    >
      <Menu class="w-5 h-5" />
    </Button>

    <!-- Overlay for mobile -->
    <div
      v-if="sidebarOpen"
      class="md:hidden fixed inset-0 bg-background/50 backdrop-blur-sm z-5"
      @click="toggleSidebar"
    />

    <!-- Right Content Area -->
    <div class="flex-1 md:ml-72 lg:ml-80 flex items-center justify-center relative bg-gradient-to-br from-muted/20 to-muted/40 min-h-screen">
      <!-- Camera Preview -->
      <CameraPreview
        v-if="!camera.isLinux.value"
        :video-ref="camera.videoRef"
        :is-streaming="camera.isStreaming.value"
        :error="camera.error.value"
      />

      <!-- Linux Placeholder -->
      <div v-if="camera.isLinux.value" class="w-full max-w-md mx-4 md:w-96 rounded-lg border border-border bg-card/80 backdrop-blur-sm shadow-lg overflow-hidden">
        <div class="aspect-video bg-muted/30 flex items-center justify-center">
          <div class="text-center space-y-3">
            <div class="w-16 h-16 rounded-full bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center mx-auto">
              <Camera class="w-8 h-8 text-primary" />
            </div>
            <div>
              <p class="text-sm font-medium text-foreground">
                Camera Preview
              </p>
              <p class="text-xs text-muted-foreground mt-1">
                Not available on Linux systems
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Loading State -->
      <div
        v-if="device.isDiscovering.value"
        class="absolute inset-0 flex items-center justify-center bg-background/50 backdrop-blur-sm"
      >
        <div class="text-center space-y-3 bg-card/80 backdrop-blur-sm p-4 md:p-6 rounded-lg border border-border/50 mx-4 max-w-sm">
          <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto" />
          <p class="text-sm text-muted-foreground">
            Discovering devices...
          </p>
        </div>
      </div>

      <!-- No Devices State -->
      <div
        v-if="!device.isDiscovering.value && device.devices.value.length === 0"
        class="absolute inset-0 flex items-center justify-center"
      >
        <div class="text-center space-y-4 max-w-sm mx-4 md:max-w-md p-6 md:p-8 bg-card/80 backdrop-blur-sm rounded-lg border border-border/50 shadow-lg">
          <div class="p-4 md:p-6 bg-gradient-to-br from-primary/10 to-primary/5 rounded-full w-fit mx-auto">
            <Lightbulb class="w-12 h-12 md:w-16 md:h-16 text-primary" />
          </div>
          <div>
            <h3 class="text-lg font-medium text-foreground">
              No devices found
            </h3>
            <p class="text-sm mt-2 text-muted-foreground">
              Make sure your Litra devices are connected via USB and try refreshing.
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
    </div>

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
            Make sure your Litra devices are connected via USB and try refreshing.
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
            {{ device.localError.value || 'An error occurred while communicating with the device.' }}
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
            {{ device.localSuccess.value || 'Operation completed successfully.' }}
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

    <!-- Camera Permission Help Modal -->
    <CameraPermissionHelp
      v-if="showPermissionHelp"
      @close="showPermissionHelp = false"
      @permission-granted="handlePermissionGranted"
    />
  </div>
</template>
