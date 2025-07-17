<script setup lang="ts">
import { Camera } from 'lucide-vue-next'
import { ref } from 'vue'
import { Button } from '@/components/ui/button'

const emit = defineEmits<{
  close: []
  permissionGranted: []
}>()

const loading = ref(false)

async function requestPermission() {
  loading.value = true

  try {
    const stream = await navigator.mediaDevices.getUserMedia({
      video: true,
      audio: false,
    })

    // Stop the stream immediately
    stream.getTracks().forEach(track => track.stop())

    emit('permissionGranted')
  }
  catch (err: any) {
    if (err.name === 'NotAllowedError') {
      // Show system-specific instructions
      showSystemInstructions()
    }
  }
  finally {
    loading.value = false
  }
}

function showSystemInstructions() {
  const platform = navigator.platform.toLowerCase()
  let instructions = ''

  if (platform.includes('mac')) {
    instructions = 'macOS: Go to System Preferences → Security & Privacy → Camera → Enable for "Litra Control"'
  }
  else if (platform.includes('win')) {
    instructions = 'Windows: Go to Settings → Privacy → Camera → Allow desktop apps to access camera'
  }
  else if (platform.includes('linux')) {
    instructions = 'Linux: Camera access is usually granted by default. Check your system settings if needed.'
  }
  else {
    instructions = 'Check your system settings to allow camera access for desktop applications'
  }

  alert(instructions)
}
</script>

<template>
  <div class="fixed inset-0 bg-background/50 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="bg-card/90 backdrop-blur-sm rounded-lg border border-border/50 shadow-lg p-6 max-w-md mx-4">
      <div class="flex items-start gap-4">
        <div class="p-3 bg-primary/10 rounded-full">
          <Camera class="w-6 h-6 text-primary" />
        </div>
        <div class="flex-1">
          <h3 class="text-lg font-semibold text-foreground mb-2">
            Camera Permission Required
          </h3>
          <p class="text-sm text-muted-foreground mb-4">
            To use the camera preview, you need to allow camera access in your system settings.
          </p>

          <div class="space-y-4">
            <div>
              <h4 class="text-sm font-medium text-foreground mb-2">
                How to enable camera access:
              </h4>
              <div class="space-y-2">
                <div class="bg-muted/30 rounded-lg p-3">
                  <p class="text-xs font-medium text-foreground">
                    macOS:
                  </p>
                  <p class="text-xs text-muted-foreground">
                    System Preferences → Security & Privacy → Camera → Enable for "Litra Control"
                  </p>
                </div>
                <div class="bg-muted/30 rounded-lg p-3">
                  <p class="text-xs font-medium text-foreground">
                    Windows:
                  </p>
                  <p class="text-xs text-muted-foreground">
                    Settings → Privacy → Camera → Allow desktop apps to access camera
                  </p>
                </div>
                <div class="bg-muted/30 rounded-lg p-3">
                  <p class="text-xs font-medium text-foreground">
                    Linux:
                  </p>
                  <p class="text-xs text-muted-foreground">
                    Camera access is usually granted by default, or check system settings
                  </p>
                </div>
              </div>
            </div>

            <div class="bg-muted/30 rounded-lg p-3">
              <p class="text-xs text-muted-foreground">
                <strong>Note:</strong> Your camera feed stays on your device and is never sent to any server.
              </p>
            </div>
          </div>

          <div class="flex items-center gap-2 mt-6">
            <Button
              class="bg-primary hover:bg-primary/90 text-primary-foreground"
              :disabled="loading"
              @click="requestPermission"
            >
              <Camera class="w-4 h-4 mr-2" />
              {{ loading ? 'Requesting...' : 'Request Permission' }}
            </Button>
            <Button
              variant="outline"
              class="border-border/50"
              @click="$emit('close')"
            >
              Cancel
            </Button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
