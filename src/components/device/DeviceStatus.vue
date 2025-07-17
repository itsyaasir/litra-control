<template>
  <div class="flex items-center gap-3">
    <!-- Connection Status -->
    <div class="flex items-center gap-2 px-2 py-1 bg-emerald-500/10 rounded-md">
      <div 
        :class="[
          'w-2 h-2 rounded-full transition-all duration-300',
          isConnected 
            ? 'bg-emerald-500 shadow-emerald-500/50 shadow-sm' 
            : 'bg-muted-foreground'
        ]"
      />
      <span 
        :class="[
          'text-xs font-medium transition-colors',
          isConnected 
            ? 'text-emerald-700 dark:text-emerald-300' 
            : 'text-muted-foreground'
        ]"
      >
        {{ isConnected ? 'Connected' : 'Disconnected' }}
      </span>
    </div>

    <!-- Power Status -->
    <div class="flex items-center gap-2 px-2 py-1 bg-primary/10 rounded-md">
      <component 
        :is="isPoweredOn ? Power : PowerOff" 
        :class="[
          'w-3 h-3 transition-all duration-300',
          isPoweredOn 
            ? 'text-primary' 
            : 'text-muted-foreground'
        ]"
      />
      <span 
        :class="[
          'text-xs font-medium transition-colors',
          isPoweredOn 
            ? 'text-primary' 
            : 'text-muted-foreground'
        ]"
      >
        {{ isPoweredOn ? 'On' : 'Off' }}
      </span>
    </div>

    <!-- Device Type Badge -->
    <div class="px-2 py-1 rounded-md text-xs font-medium bg-secondary text-secondary-foreground">
      {{ deviceType }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { Power, PowerOff } from 'lucide-vue-next'

interface Props {
  isConnected: boolean
  isPoweredOn: boolean
  deviceType: string
}

defineProps<Props>()
</script>