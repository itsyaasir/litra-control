<template>
  <div class="w-full max-w-md mx-4 md:w-96 rounded-lg border border-border bg-card/80 backdrop-blur-sm shadow-lg overflow-hidden">
    <!-- Video Preview -->
    <div class="relative aspect-video bg-muted/30 flex items-center justify-center">
      <video
        :ref="videoRef"
        v-show="isStreaming && !error"
        class="w-full h-full object-cover"
        autoplay
        muted
        playsinline
      />
      
      <!-- Placeholder when not streaming -->
      <div v-if="!isStreaming && !error" class="text-center space-y-3">
        <div class="w-16 h-16 rounded-full bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center mx-auto">
          <Camera class="w-8 h-8 text-primary" />
        </div>
        <p class="text-sm text-muted-foreground">Use camera controls in the sidebar</p>
      </div>

      <!-- Error State -->
      <div v-if="error" class="text-center space-y-3 p-4">
        <div class="w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center mx-auto">
          <Zap class="w-8 h-8 text-destructive" />
        </div>
        <div>
          <p class="text-sm font-medium text-destructive">Camera Error</p>
          <p class="text-xs text-muted-foreground mt-1">{{ error }}</p>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="isStreaming && !videoRef?.srcObject" class="text-center space-y-3">
        <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto"></div>
        <p class="text-sm text-muted-foreground">Starting camera...</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Camera, Zap } from 'lucide-vue-next'

interface Props {
  videoRef?: any
  isStreaming?: boolean
  error?: string
}

defineProps<Props>()

</script>