<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

interface Props {
  modelValue: number
  min?: number
  max?: number
  step?: number
  disabled?: boolean
  gradientType?: 'temperature' | 'brightness' | 'none'
  trackClass?: string
  progressClass?: string
  thumbClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  min: 0,
  max: 100,
  step: 1,
  disabled: false,
  gradientType: 'none',
})

const emit = defineEmits<{
  'update:modelValue': [value: number]
}>()

const sliderContainer = ref<HTMLElement>()
const track = ref<HTMLElement>()
const isDragging = ref(false)

// Calculate progress percentage
const progressPercentage = computed(() => {
  if (props.max === props.min)
    return 0
  return ((props.modelValue - props.min) / (props.max - props.min)) * 100
})

// Gradient classes based on type
const gradientClass = computed(() => {
  switch (props.gradientType) {
    case 'temperature':
      return 'bg-gradient-to-r from-orange-500 via-yellow-300 to-blue-400'
    case 'brightness':
      return 'bg-gradient-to-r from-gray-700 to-white'
    default:
      return ''
  }
})

// Update value based on position
function updateValue(clientX: number) {
  if (!track.value || props.disabled)
    return

  const rect = track.value.getBoundingClientRect()
  const percentage = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width))
  const rawValue = props.min + percentage * (props.max - props.min)
  const steppedValue = Math.round(rawValue / props.step) * props.step
  const clampedValue = Math.max(props.min, Math.min(props.max, steppedValue))

  emit('update:modelValue', clampedValue)
}

// Handle track click
function handleTrackClick(event: MouseEvent) {
  if (props.disabled)
    return
  updateValue(event.clientX)
}

// Handle drag start
function startDrag(event: MouseEvent | TouchEvent) {
  if (props.disabled)
    return

  isDragging.value = true
  event.preventDefault()

  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX
  updateValue(clientX)
}

// Handle mouse/touch move
function handleMove(event: MouseEvent | TouchEvent) {
  if (!isDragging.value || props.disabled)
    return

  event.preventDefault()
  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX
  updateValue(clientX)
}

// Handle drag end
function endDrag() {
  isDragging.value = false
}

// Handle keyboard navigation
function handleKeyDown(event: KeyboardEvent) {
  if (props.disabled)
    return

  let newValue = props.modelValue
  const stepSize = props.step
  const largeStep = (props.max - props.min) / 10

  switch (event.key) {
    case 'ArrowLeft':
    case 'ArrowDown':
      newValue = Math.max(props.min, props.modelValue - stepSize)
      break
    case 'ArrowRight':
    case 'ArrowUp':
      newValue = Math.min(props.max, props.modelValue + stepSize)
      break
    case 'PageDown':
      newValue = Math.max(props.min, props.modelValue - largeStep)
      break
    case 'PageUp':
      newValue = Math.min(props.max, props.modelValue + largeStep)
      break
    case 'Home':
      newValue = props.min
      break
    case 'End':
      newValue = props.max
      break
    default:
      return
  }

  event.preventDefault()
  emit('update:modelValue', Math.round(newValue / props.step) * props.step)
}

// Setup event listeners
onMounted(() => {
  document.addEventListener('mousemove', handleMove)
  document.addEventListener('mouseup', endDrag)
  document.addEventListener('touchmove', handleMove, { passive: false })
  document.addEventListener('touchend', endDrag)
})

onUnmounted(() => {
  document.removeEventListener('mousemove', handleMove)
  document.removeEventListener('mouseup', endDrag)
  document.removeEventListener('touchmove', handleMove)
  document.removeEventListener('touchend', endDrag)
})
</script>

<template>
  <div ref="sliderContainer" class="custom-slider-container">
    <!-- Track -->
    <div
      ref="track"
      class="slider-track"
      :class="trackClass"
      @click="handleTrackClick"
    >
      <!-- Background gradient (for temperature and brightness) -->
      <div
        v-if="gradientType"
        class="slider-gradient"
        :class="gradientClass"
      />

      <!-- Progress fill -->
      <div
        class="slider-progress"
        :style="{ width: `${progressPercentage}%` }"
        :class="progressClass"
      />

      <!-- Thumb -->
      <div
        class="slider-thumb"
        :class="thumbClass"
        :style="{ left: `${progressPercentage}%` }"
        tabindex="0"
        role="slider"
        :aria-valuenow="modelValue"
        :aria-valuemin="min"
        :aria-valuemax="max"
        :aria-disabled="disabled"
        @mousedown="startDrag"
        @touchstart="startDrag"
        @keydown="handleKeyDown"
      />
    </div>
  </div>
</template>

<style scoped>
@reference "../index.css";

.custom-slider-container {
  @apply relative w-full py-2;
}

.slider-track {
  @apply relative w-full h-1.5 bg-muted rounded-full cursor-pointer;
}

.slider-gradient {
  @apply absolute inset-0 rounded-full;
}

.slider-progress {
  @apply absolute left-0 top-0 h-full bg-primary rounded-full transition-all duration-200;
}

.slider-thumb {
  @apply absolute top-1/2 w-5 h-5 bg-background border-2 border-primary rounded-full cursor-grab transform -translate-y-1/2 -translate-x-1/2 transition-all duration-200 shadow-lg;
}

.slider-thumb:hover {
  @apply scale-110 shadow-xl;
}

.slider-thumb:active {
  @apply cursor-grabbing scale-105;
}

.slider-thumb:focus {
  @apply outline-none ring-2 ring-ring/50;
}

/* Disabled state */
.slider-track.disabled {
  @apply opacity-50 cursor-not-allowed;
}

.slider-thumb.disabled {
  @apply opacity-50 cursor-not-allowed;
}

.slider-thumb.disabled:hover {
  @apply scale-100 shadow-lg;
}
</style>
