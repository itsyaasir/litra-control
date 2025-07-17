<template>
  <div class="space-y-3">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Thermometer class="w-4 h-4 text-orange-500" />
        <label :for="sliderId" class="text-sm font-medium text-gray-700 dark:text-gray-300">
          Temperature
        </label>
      </div>
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <div 
          :class="[
            'w-4 h-4 rounded-full border border-gray-300 dark:border-gray-600',
            'shadow-sm'
          ]"
          :style="{ backgroundColor: currentColor }"
        />
        <span>{{ currentKelvin }}K</span>
        <span>•</span>
        <span>{{ temperatureLabel }}</span>
      </div>
    </div>

    <!-- Slider Container -->
    <div class="relative">
      <!-- Slider Track with Gradient -->
      <div 
        :class="[
          'h-2 rounded-full overflow-hidden',
          'bg-gradient-to-r from-orange-400 via-yellow-200 to-blue-400'
        ]"
      />

      <!-- Slider Input -->
      <input
        :id="sliderId"
        type="range"
        :min="minKelvin"
        :max="maxKelvin"
        :step="100"
        :value="currentKelvin"
        :disabled="loading || disabled"
        @input="handleInput"
        @change="handleChange"
        :class="[
          'absolute inset-0 w-full h-2 opacity-0 cursor-pointer',
          'disabled:cursor-not-allowed'
        ]"
      />

      <!-- Slider Thumb -->
      <div 
        :class="[
          'absolute top-1/2 w-5 h-5 -translate-y-1/2 -translate-x-1/2',
          'bg-white border-2 rounded-full shadow-sm',
          'transition-all duration-200 ease-out',
          'hover:scale-110 focus-within:scale-110',
          loading ? 'animate-pulse' : '',
          disabled ? 'opacity-50' : ''
        ]"
        :style="{ 
          left: `${currentPercentage}%`,
          borderColor: currentColor
        }"
      >
        <!-- Loading indicator -->
        <div 
          v-if="loading"
          :class="[
            'absolute inset-0.5 border border-t-transparent rounded-full animate-spin'
          ]"
          :style="{ borderColor: currentColor }"
        />
      </div>
    </div>

    <!-- Range Labels -->
    <div class="flex justify-between text-xs text-gray-400 dark:text-gray-500">
      <span>{{ minKelvin }}K (Warm)</span>
      <span>{{ maxKelvin }}K (Cool)</span>
    </div>

    <!-- Temperature Presets -->
    <div class="grid grid-cols-3 gap-2">
      <button
        v-for="preset in temperaturePresets"
        :key="preset.kelvin"
        type="button"
        @click="setTemperature(preset.kelvin)"
        :disabled="loading || disabled"
        :class="[
          'px-2 py-1.5 text-xs rounded-md transition-all duration-200',
          'border border-gray-200 dark:border-gray-700',
          'hover:border-gray-300 dark:hover:border-gray-600',
          'disabled:opacity-50 disabled:cursor-not-allowed',
          currentKelvin === preset.kelvin
            ? 'bg-blue-50 dark:bg-blue-950 border-blue-300 dark:border-blue-700 text-blue-700 dark:text-blue-300'
            : 'bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-750'
        ]"
      >
        <div class="flex items-center gap-1.5">
          <div 
            class="w-2 h-2 rounded-full"
            :style="{ backgroundColor: kelvinToColor(preset.kelvin) }"
          />
          <span class="font-medium">{{ preset.name }}</span>
        </div>
        <div class="text-xs opacity-75 mt-0.5">{{ preset.kelvin }}K</div>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { Thermometer } from 'lucide-vue-next'

interface TemperaturePreset {
  name: string
  kelvin: number
  description: string
}

interface Props {
  currentKelvin: number
  minKelvin: number
  maxKelvin: number
  loading?: boolean
  disabled?: boolean
  id?: string
}

interface Emits {
  (e: 'update:kelvin', value: number): void
  (e: 'change', value: number): void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  disabled: false,
  id: undefined
})

const emit = defineEmits<Emits>()

const localValue = ref(props.currentKelvin)
const sliderId = computed(() => props.id || `temperature-slider-${Math.random().toString(36).substr(2, 9)}`)

// Temperature presets
const temperaturePresets: TemperaturePreset[] = [
  { name: 'Warm', kelvin: 2700, description: 'Cozy warm light' },
  { name: 'Soft', kelvin: 3200, description: 'Soft white light' },
  { name: 'Natural', kelvin: 4000, description: 'Natural daylight' },
  { name: 'Cool', kelvin: 5000, description: 'Cool daylight' },
  { name: 'Bright', kelvin: 5500, description: 'Bright daylight' },
  { name: 'Cold', kelvin: 6500, description: 'Cold blue light' }
]

const currentPercentage = computed(() => {
  const range = props.maxKelvin - props.minKelvin
  if (range === 0) return 0
  return ((props.currentKelvin - props.minKelvin) / range) * 100
})

const currentColor = computed(() => kelvinToColor(props.currentKelvin))

const temperatureLabel = computed(() => {
  if (props.currentKelvin <= 3000) return 'Warm'
  if (props.currentKelvin <= 3500) return 'Soft White'
  if (props.currentKelvin <= 4500) return 'Natural'
  if (props.currentKelvin <= 5500) return 'Cool'
  return 'Cold'
})

// Convert Kelvin to RGB color
function kelvinToColor(kelvin: number): string {
  // Simplified color temperature to RGB conversion
  let r, g, b

  // Red component
  if (kelvin >= 6600) {
    r = 255
  } else {
    r = kelvin / 100
    r = 329.698727446 * Math.pow(r - 60, -0.1332047592)
    r = Math.max(0, Math.min(255, r))
  }

  // Green component
  if (kelvin >= 6600) {
    g = kelvin / 100
    g = 288.1221695283 * Math.pow(g - 60, -0.0755148492)
  } else {
    g = kelvin / 100
    g = 99.4708025861 * Math.log(g) - 161.1195681661
  }
  g = Math.max(0, Math.min(255, g))

  // Blue component
  if (kelvin >= 6600) {
    b = 255
  } else if (kelvin <= 1900) {
    b = 0
  } else {
    b = kelvin / 100
    b = 138.5177312231 * Math.log(b - 10) - 305.0447927307
    b = Math.max(0, Math.min(255, b))
  }

  return `rgb(${Math.round(r)}, ${Math.round(g)}, ${Math.round(b)})`
}

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = parseInt(target.value)
  // Round to nearest 100
  const rounded = Math.round(value / 100) * 100
  localValue.value = rounded
  emit('update:kelvin', rounded)
}

const handleChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = parseInt(target.value)
  // Round to nearest 100
  const rounded = Math.round(value / 100) * 100
  emit('change', rounded)
}

const setTemperature = (kelvin: number) => {
  emit('change', kelvin)
}
</script>