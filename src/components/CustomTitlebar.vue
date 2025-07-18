<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X } from 'lucide-vue-next'
import { onMounted, ref } from 'vue'

import LitraLogo from './LitraLogo.vue'
import { Button } from './ui/button'

const isMaximized = ref(false)
const appWindow = getCurrentWindow()

// Window control functions
async function minimizeWindow() {
  await appWindow.minimize()
}

async function maximizeWindow() {
  await appWindow.toggleMaximize()
}

async function closeWindow() {
  await appWindow.hide()
}

// Check if window is maximized
async function checkMaximized() {
  isMaximized.value = await appWindow.isMaximized()
}

onMounted(() => {
  checkMaximized()

  // Listen for window state changes
  appWindow.onResized(() => {
    checkMaximized()
  })
})
</script>

<template>
  <div class="custom-titlebar">
    <div class="titlebar-content" data-tauri-drag-region>
      <!-- Left side - App info -->
      <div class="titlebar-left">
        <div class="flex items-center gap-2">
          <LitraLogo :size="20" />
          <span class="text-sm font-semibold text-foreground">
            Litra Control
          </span>
        </div>
      </div>

      <!-- Right side - Window controls -->
      <div class="titlebar-right">
        <Button
          variant="ghost"
          size="icon"
          class="titlebar-button hover:bg-muted/50"
          @click="minimizeWindow"
        >
          <Minus class="w-4 h-4" />
        </Button>

        <Button
          variant="ghost"
          size="icon"
          class="titlebar-button hover:bg-muted/50"
          @click="maximizeWindow"
        >
          <Square class="w-4 h-4" />
        </Button>

        <Button
          variant="ghost"
          size="icon"
          class="titlebar-button hover:bg-destructive/10 hover:text-destructive"
          @click="closeWindow"
        >
          <X class="w-4 h-4" />
        </Button>
      </div>
    </div>
  </div>
</template>

<style scoped>
@reference "../index.css";
.custom-titlebar {
  @apply bg-background flex-shrink-0 select-none;
  height: 32px;
  width: 100%;
  position: relative;
  z-index: 1000;
}

.titlebar-content {
  @apply flex items-center justify-between h-full px-4;
}

.titlebar-left {
  @apply flex items-center gap-3;
}

.titlebar-right {
  @apply flex items-center gap-0.5;
}

.titlebar-button {
  @apply h-8 w-8 p-0 text-muted-foreground hover:text-foreground
    transition-colors;
  border-radius: 4px;
}

.titlebar-button:hover {
  @apply bg-muted/50;
}

.titlebar-button:last-child:hover {
  @apply bg-destructive/10 text-destructive;
}

/* Ensure the drag region works properly */
[data-tauri-drag-region] {
  -webkit-app-region: drag;
  app-region: drag;
}

[data-tauri-drag-region] button,
[data-tauri-drag-region] .titlebar-button {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.titlebar-button {
  -webkit-app-region: no-drag;
  app-region: no-drag;
}
</style>
