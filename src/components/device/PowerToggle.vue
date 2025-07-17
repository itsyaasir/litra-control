<template>
  <div class="flex items-center gap-3">
    <div class="flex flex-col gap-1">
      <label :for="toggleId" class="text-sm font-medium text-gray-700 dark:text-gray-300">
        Power
      </label>
      <span class="text-xs text-gray-500 dark:text-gray-400">
        {{ isOn ? 'Device is on' : 'Device is off' }}
      </span>
    </div>
    
    <button
      :id="toggleId"
      type="button"
      role="switch"
      :aria-checked="isOn"
      :disabled="loading || disabled"
      @click="handleToggle"
      :class="[
        'relative inline-flex h-6 w-11 items-center rounded-full transition-all duration-300 ease-in-out',
        'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900',
        'disabled:opacity-50 disabled:cursor-not-allowed',
        isOn 
          ? 'bg-blue-600 hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600' 
          : 'bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600'
      ]"
    >
      <!-- Toggle Circle -->
      <span
        :class="[
          'inline-block h-4 w-4 transform rounded-full transition-all duration-300 ease-in-out',
          'bg-white shadow-sm ring-0',
          isOn ? 'translate-x-6' : 'translate-x-1'
        ]"
      />
      
      <!-- Loading Spinner -->
      <div
        v-if="loading"
        :class="[
          'absolute inset-0 flex items-center justify-center',
          'transition-opacity duration-200'
        ]"
      >
        <div 
          class="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin"
        />
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  isOn: boolean
  loading?: boolean
  disabled?: boolean
  id?: string
}

interface Emits {
  (e: 'toggle'): void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  disabled: false,
  id: undefined
})

const emit = defineEmits<Emits>()

const toggleId = computed(() => props.id || `power-toggle-${Math.random().toString(36).substr(2, 9)}`)

const handleToggle = () => {
  if (!props.loading && !props.disabled) {
    emit('toggle')
  }
}
</script>