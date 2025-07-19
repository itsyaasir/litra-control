<script setup lang="ts">
import { Toaster } from '@/components/ui/sonner'
import 'vue-sonner/style.css'

document.addEventListener('DOMContentLoaded', () => {
  document.addEventListener('contextmenu', (e) => {
    e.preventDefault()
  })
})
</script>

<template>
  <RouterView />
  <Toaster />
</template>
