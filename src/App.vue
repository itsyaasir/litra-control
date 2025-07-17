<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useDevice } from './composables'
import { useSettingsStore } from './stores'
import { RefreshCw, Settings, Lightbulb, Power, PowerOff, Sun, Thermometer } from 'lucide-vue-next'

// Shadcn Components
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './components/ui/card'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './components/ui/select'
import { Button } from './components/ui/button'
import { Switch } from './components/ui/switch'
import { Slider } from './components/ui/slider'

const device = useDevice()
const settingsStore = useSettingsStore()

// Initialize the app
onMounted(() => {
  settingsStore.initialize()
})

// Get selected device
const selectedDevice = computed(() => device.selectedDevice.value)

// Handle device selection from dropdown
const handleDeviceSelect = (value: any) => {
  if (value && typeof value === 'string') {
    device.selectDevice(value)
  }
}

// Handle device operations
const handlePowerToggle = (checked: boolean) => {
  if (selectedDevice.value) {
    device.setPowerState(checked, selectedDevice.value.serial_number)
  }
}

const handleBrightnessChange = (value: number[] | undefined) => {
  if (selectedDevice.value && value && value[0] !== undefined) {
    device.setBrightness(value[0], selectedDevice.value.serial_number)
  }
}

const handleTemperatureChange = (value: number[] | undefined) => {
  if (selectedDevice.value && value && value[0] !== undefined) {
    device.setTemperature(value[0], selectedDevice.value.serial_number)
  }
}

</script>

<template>
  <div class="min-h-screen bg-background transition-colors">
    <!-- Header -->
    <header class="bg-card border-b border-border">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <!-- Logo and Title -->
          <div class="flex items-center gap-3">
            <div class="p-2 bg-primary/10 rounded-lg">
              <Lightbulb class="w-6 h-6 text-primary" />
            </div>
            <div>
              <h1 class="text-xl font-bold text-foreground">
                Litra Control
              </h1>
              <p class="text-sm text-muted-foreground">
                {{ device.deviceStats.value.total }} device{{ device.deviceStats.value.total !== 1 ? 's' : '' }} found
              </p>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-3">
            <!-- Device Stats -->
            <div class="hidden sm:flex items-center gap-4 text-sm">
              <div class="flex items-center gap-2 px-2 py-1 bg-emerald-500/10 rounded-md">
                <div class="w-2 h-2 bg-emerald-500 rounded-full"></div>
                <span class="text-emerald-700 dark:text-emerald-300 font-medium">{{ device.deviceStats.value.connected }} connected</span>
              </div>
              <div class="flex items-center gap-2 px-2 py-1 bg-primary/10 rounded-md">
                <div class="w-2 h-2 bg-primary rounded-full"></div>
                <span class="text-primary font-medium">{{ device.deviceStats.value.poweredOn }} powered on</span>
              </div>
            </div>

            <!-- Refresh Button -->
            <Button
              @click="device.refreshDevices"
              :disabled="device.isRefreshing.value"
              variant="ghost"
              size="icon"
            >
              <RefreshCw 
                :class="[
                  'w-5 h-5',
                  device.isRefreshing.value ? 'animate-spin' : ''
                ]"
              />
            </Button>

            <!-- Settings Button -->
            <Button
              variant="ghost"
              size="icon"
            >
              <Settings class="w-5 h-5" />
            </Button>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- Loading State -->
      <div 
        v-if="device.isDiscovering.value" 
        class="flex items-center justify-center py-12"
      >
        <div class="text-center space-y-3">
          <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto"></div>
          <p class="text-sm text-muted-foreground">Discovering devices...</p>
        </div>
      </div>

      <!-- No Devices State -->
      <div 
        v-else-if="device.devices.value.length === 0" 
        class="text-center py-12"
      >
        <Card class="max-w-md mx-auto">
          <CardContent class="pt-6">
            <div class="space-y-4">
              <div class="p-4 bg-muted rounded-full w-fit mx-auto">
                <Lightbulb class="w-16 h-16 text-muted-foreground" />
              </div>
              <div>
                <h3 class="text-lg font-medium text-foreground">No devices found</h3>
                <p class="text-sm text-muted-foreground max-w-sm mx-auto mt-2">
                  Make sure your Litra devices are connected via USB and try refreshing.
                </p>
              </div>
              <Button
                @click="device.refreshDevices"
                class="mt-4"
              >
                Refresh Devices
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- Device Controls -->
      <div v-else class="space-y-6">
        <!-- Device Selector -->
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2">
              <Lightbulb class="w-5 h-5 text-primary" />
              Device Selection
            </CardTitle>
            <CardDescription>
              Choose which Litra device to control
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Select 
              :model-value="selectedDevice?.serial_number || ''"
              @update:model-value="handleDeviceSelect"
            >
              <SelectTrigger>
                <SelectValue placeholder="Select a device..." />
              </SelectTrigger>
              <SelectContent>
                <SelectItem 
                  v-for="deviceItem in device.devices.value"
                  :key="deviceItem.serial_number"
                  :value="deviceItem.serial_number"
                >
                  <div class="flex items-center gap-3">
                    <div class="flex items-center gap-1">
                      <div 
                        :class="[
                          'w-2 h-2 rounded-full',
                          deviceItem.is_connected ? 'bg-emerald-500' : 'bg-gray-400'
                        ]"
                      />
                      <span class="font-medium">{{ deviceItem.device_type }}</span>
                    </div>
                    <span class="text-xs text-muted-foreground font-mono">{{ deviceItem.serial_number }}</span>
                  </div>
                </SelectItem>
              </SelectContent>
            </Select>
          </CardContent>
        </Card>

        <!-- Device Controls -->
        <div v-if="selectedDevice" class="space-y-6">
          <!-- Device Status Card -->
          <Card>
            <CardHeader>
              <CardTitle class="flex items-center gap-2">
                <component 
                  :is="selectedDevice.is_on ? Power : PowerOff"
                  class="w-5 h-5 text-primary"
                />
                {{ selectedDevice.device_type }}
              </CardTitle>
              <CardDescription>
                Serial: {{ selectedDevice.serial_number }}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div class="flex items-center justify-between">
                <div>
                  <p class="font-medium">Power</p>
                  <p class="text-sm text-muted-foreground">
                    Device is {{ selectedDevice.is_on ? 'on' : 'off' }}
                  </p>
                </div>
                <Switch
                  :model-value="selectedDevice.is_on"
                  @update:model-value="handlePowerToggle"
                  :disabled="!selectedDevice.is_connected || device.isPowerChanging.value"
                />
              </div>
            </CardContent>
          </Card>

          <!-- Brightness Control -->
          <Card v-if="selectedDevice.is_connected">
            <CardHeader>
              <CardTitle class="flex items-center gap-2">
                <Sun class="w-5 h-5 text-yellow-500" />
                Brightness
              </CardTitle>
              <CardDescription>
                {{ selectedDevice.brightness_lumens }} lm • {{ Math.round(((selectedDevice.brightness_lumens - selectedDevice.min_brightness_lumens) / (selectedDevice.max_brightness_lumens - selectedDevice.min_brightness_lumens)) * 100) }}%
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
              <Slider
                :model-value="[selectedDevice.brightness_lumens]"
                @update:model-value="handleBrightnessChange"
                :min="selectedDevice.min_brightness_lumens"
                :max="selectedDevice.max_brightness_lumens"
                :step="1"
                :disabled="!selectedDevice.is_on || device.isBrightnessChanging.value"
                class="w-full"
              />
              <div class="flex justify-between text-xs text-muted-foreground">
                <span>{{ selectedDevice.min_brightness_lumens }} lm</span>
                <span>{{ selectedDevice.max_brightness_lumens }} lm</span>
              </div>
            </CardContent>
          </Card>

          <!-- Temperature Control -->
          <Card v-if="selectedDevice.is_connected">
            <CardHeader>
              <CardTitle class="flex items-center gap-2">
                <Thermometer class="w-5 h-5 text-orange-500" />
                Temperature
              </CardTitle>
              <CardDescription>
                {{ selectedDevice.temperature_kelvin }}K • {{ selectedDevice.temperature_kelvin <= 3000 ? 'Warm' : selectedDevice.temperature_kelvin <= 4500 ? 'Natural' : 'Cool' }}
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
              <Slider
                :model-value="[selectedDevice.temperature_kelvin]"
                @update:model-value="handleTemperatureChange"
                :min="selectedDevice.min_temperature_kelvin"
                :max="selectedDevice.max_temperature_kelvin"
                :step="100"
                :disabled="!selectedDevice.is_on || device.isTemperatureChanging.value"
                class="w-full"
              />
              <div class="flex justify-between text-xs text-muted-foreground">
                <span>{{ selectedDevice.min_temperature_kelvin }}K (Warm)</span>
                <span>{{ selectedDevice.max_temperature_kelvin }}K (Cool)</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </main>

    <!-- Error Messages -->
    <div 
      v-if="device.hasError.value"
      class="fixed bottom-4 right-4 max-w-sm bg-destructive/5 border border-destructive/20 rounded-lg p-4 shadow-lg backdrop-blur-sm"
    >
      <div class="flex items-start gap-3">
        <div class="w-5 h-5 text-destructive flex-shrink-0 mt-0.5">
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-destructive">
            Operation Failed
          </p>
          <p class="text-sm text-destructive/80 mt-1">
            {{ device.localError.value || 'An error occurred while communicating with the device.' }}
          </p>
        </div>
        <Button
          @click="device.clearMessages"
          variant="ghost"
          size="icon"
          class="text-destructive/60 hover:text-destructive h-auto w-auto p-1"
        >
          <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </Button>
      </div>
    </div>

    <!-- Success Messages -->
    <div 
      v-if="device.hasSuccess.value"
      class="fixed bottom-4 right-4 max-w-sm bg-emerald-500/10 border border-emerald-500/20 rounded-lg p-4 shadow-lg backdrop-blur-sm"
    >
      <div class="flex items-start gap-3">
        <div class="w-5 h-5 text-emerald-600 dark:text-emerald-400 flex-shrink-0 mt-0.5">
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-emerald-700 dark:text-emerald-300">
            Success
          </p>
          <p class="text-sm text-emerald-600 dark:text-emerald-400 mt-1">
            {{ device.localSuccess.value || 'Operation completed successfully.' }}
          </p>
        </div>
        <Button
          @click="device.clearMessages"
          variant="ghost"
          size="icon"
          class="text-emerald-500/60 hover:text-emerald-600 dark:hover:text-emerald-400 h-auto w-auto p-1"
        >
          <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </Button>
      </div>
    </div>
  </div>
</template>