<template>
  <div 
    :class="[
      'relative bg-card rounded-xl border border-border',
      'shadow-sm hover:shadow-lg transition-all duration-300 ease-out',
      'p-6 space-y-6',
      isSelected ? 'ring-2 ring-primary shadow-primary/20' : '',
      !device.is_connected ? 'opacity-75' : ''
    ]"
  >
    <!-- Header -->
    <div class="flex items-start justify-between">
      <div class="space-y-1">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary/10 rounded-lg">
            <component 
              :is="deviceIcon" 
              :class="[
                'w-5 h-5',
                device.is_connected 
                  ? 'text-primary' 
                  : 'text-muted-foreground'
              ]"
            />
          </div>
          <div>
            <h3 class="font-semibold text-foreground">
              {{ device.device_type }}
            </h3>
            <p class="text-xs text-muted-foreground font-mono">
              {{ device.serial_number }}
            </p>
          </div>
        </div>
      </div>

      <!-- Selection Button -->
      <button
        @click="$emit('select')"
        :class="[
          'p-2 rounded-lg transition-colors duration-200',
          'hover:bg-accent',
          isSelected 
            ? 'bg-primary/10 text-primary' 
            : 'text-muted-foreground'
        ]"
      >
        <Check v-if="isSelected" class="w-4 h-4" />
        <Circle v-else class="w-4 h-4" />
      </button>
    </div>

    <!-- Status -->
    <DeviceStatus 
      :is-connected="device.is_connected"
      :is-powered-on="device.is_on"
      :device-type="deviceTypeBadge"
    />

    <!-- Controls -->
    <div v-if="device.is_connected" class="space-y-6">
      <!-- Power Control -->
      <PowerToggle
        :is-on="device.is_on"
        :loading="isPowerChanging"
        @toggle="handlePowerToggle"
      />

      <!-- Brightness Control -->
      <BrightnessSlider
        :current-lumens="device.brightness_lumens"
        :min-lumens="device.min_brightness_lumens"
        :max-lumens="device.max_brightness_lumens"
        :loading="isBrightnessChanging"
        :disabled="!device.is_on"
        @change="handleBrightnessChange"
      />

      <!-- Temperature Control -->
      <TemperatureSlider
        :current-kelvin="device.temperature_kelvin"
        :min-kelvin="device.min_temperature_kelvin"
        :max-kelvin="device.max_temperature_kelvin"
        :loading="isTemperatureChanging"
        :disabled="!device.is_on"
        @change="handleTemperatureChange"
      />
    </div>

    <!-- Disconnected State -->
    <div 
      v-else 
      class="flex items-center justify-center py-8"
    >
      <div class="text-center space-y-3">
        <div class="p-3 bg-muted rounded-full w-fit mx-auto">
          <WifiOff class="w-8 h-8 text-muted-foreground" />
        </div>
        <div>
          <p class="text-sm font-medium text-foreground">Device Disconnected</p>
          <p class="text-xs text-muted-foreground">Reconnect device to control</p>
        </div>
      </div>
    </div>

    <!-- Loading Overlay -->
    <div 
      v-if="isLoading"
      class="absolute inset-0 bg-card/90 backdrop-blur-sm rounded-xl flex items-center justify-center"
    >
      <div class="flex items-center gap-3 text-sm text-foreground bg-card border border-border rounded-lg px-4 py-2 shadow-lg">
        <div class="w-4 h-4 border-2 border-primary border-t-transparent rounded-full animate-spin" />
        <span class="font-medium">Updating device...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  Lightbulb, 
  Circle, 
  Check, 
  WifiOff,
  Lamp,
  Flashlight
} from 'lucide-vue-next'
import type { DeviceInfo } from '../../types'
import DeviceStatus from './DeviceStatus.vue'
import PowerToggle from './PowerToggle.vue'
import BrightnessSlider from './BrightnessSlider.vue'
import TemperatureSlider from './TemperatureSlider.vue'

interface Props {
  device: DeviceInfo
  isSelected?: boolean
  isPowerChanging?: boolean
  isBrightnessChanging?: boolean
  isTemperatureChanging?: boolean
}

interface Emits {
  (e: 'select'): void
  (e: 'power-toggle'): void
  (e: 'brightness-change', lumens: number): void
  (e: 'temperature-change', kelvin: number): void
}

const props = withDefaults(defineProps<Props>(), {
  isSelected: false,
  isPowerChanging: false,
  isBrightnessChanging: false,
  isTemperatureChanging: false
})

const emit = defineEmits<Emits>()

const deviceIcon = computed(() => {
  switch (props.device.device_type) {
    case 'Litra Glow':
      return Lightbulb
    case 'Litra Beam':
      return Lamp
    case 'Litra Beam LX':
      return Flashlight
    default:
      return Lightbulb
  }
})

const deviceTypeBadge = computed(() => {
  return props.device.device_type.replace('Litra ', '')
})

const isLoading = computed(() => 
  props.isPowerChanging || props.isBrightnessChanging || props.isTemperatureChanging
)

const handlePowerToggle = () => {
  emit('power-toggle')
}

const handleBrightnessChange = (lumens: number) => {
  emit('brightness-change', lumens)
}

const handleTemperatureChange = (kelvin: number) => {
  emit('temperature-change', kelvin)
}
</script>