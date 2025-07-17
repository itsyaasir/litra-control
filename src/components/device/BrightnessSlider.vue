<template>
  <div class="space-y-3">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Sun class="w-4 h-4 text-yellow-500" />
        <label :for="sliderId" class="text-sm font-medium text-gray-700 dark:text-gray-300">
          Brightness
        </label>
      </div>
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <span>{{ currentLumens }} lm</span>
        <span>•</span>
        <span>{{ currentPercentage }}%</span>
      </div>
    </div>

    <!-- Slider Container -->
    <div class="relative">
      <!-- Slider Track -->
      <div 
        class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden"
      >
        <!-- Progress Fill -->
        <div 
          :class="[
            'h-full rounded-full transition-all duration-300 ease-out',
            'bg-gradient-to-r from-yellow-400 to-yellow-500'
          ]"
          :style="{ width: `${currentPercentage}%` }"
        />
      </div>

      <!-- Slider Input -->
      <input
        :id="sliderId"
        type="range"
        :min="minLumens"
        :max="maxLumens"
        :value="currentLumens"
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
          'bg-white border-2 border-yellow-500 rounded-full shadow-sm',
          'transition-all duration-200 ease-out',
          'hover:scale-110 focus-within:scale-110',
          loading ? 'animate-pulse' : '',
          disabled ? 'opacity-50' : ''
        ]"
        :style="{ left: `${currentPercentage}%` }"
      >
        <!-- Loading indicator -->
        <div 
          v-if="loading"
          class="absolute inset-0.5 border border-yellow-500 border-t-transparent rounded-full animate-spin"
        />
      </div>
    </div>

    <!-- Range Labels -->
    <div class="flex justify-between text-xs text-gray-400 dark:text-gray-500">
      <span>{{ minLumens }} lm</span>
      <span>{{ maxLumens }} lm</span>
    </div>

    <!-- Quick Controls -->
    <div class="flex justify-center gap-2">
      <button
        type="button"
        @click="setPercentage(25)"
        :disabled="loading || disabled"
        class="px-2 py-1 text-xs rounded-md bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        25%
      </button>
      <button
        type="button"
        @click="setPercentage(50)"
        :disabled="loading || disabled"
        class="px-2 py-1 text-xs rounded-md bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        50%
      </button>
      <button
        type="button"
        @click="setPercentage(75)"
        :disabled="loading || disabled"
        class="px-2 py-1 text-xs rounded-md bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        75%
      </button>
      <button
        type="button"
        @click="setPercentage(100)"
        :disabled="loading || disabled"
        class="px-2 py-1 text-xs rounded-md bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        100%
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { Sun } from 'lucide-vue-next'

interface Props {
  currentLumens: number
  minLumens: number
  maxLumens: number
  loading?: boolean
  disabled?: boolean
  id?: string
}

interface Emits {
  (e: 'update:lumens', value: number): void
  (e: 'change', value: number): void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  disabled: false,
  id: undefined
})

const emit = defineEmits<Emits>()

const localValue = ref(props.currentLumens)
const sliderId = computed(() => props.id || `brightness-slider-${Math.random().toString(36).substr(2, 9)}`)

const currentPercentage = computed(() => {
  const range = props.maxLumens - props.minLumens
  if (range === 0) return 0
  return Math.round(((props.currentLumens - props.minLumens) / range) * 100)
})

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = parseInt(target.value)
  localValue.value = value
  emit('update:lumens', value)
}

const handleChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = parseInt(target.value)
  emit('change', value)
}

const setPercentage = (percentage: number) => {
  const range = props.maxLumens - props.minLumens
  const lumens = Math.round(props.minLumens + (range * percentage / 100))
  emit('change', lumens)
}
</script>