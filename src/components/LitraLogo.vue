<script setup lang="ts">
interface Props {
  size?: number
  className?: string
}

withDefaults(defineProps<Props>(), {
  size: 24,
  className: '',
})
</script>

<template>
  <svg
    :width="size"
    :height="size"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    :class="className"
  >
    <defs>
      <linearGradient id="logoGradient" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" style="stop-color: #3b82f6; stop-opacity: 1" />
        <stop offset="100%" style="stop-color: #1d4ed8; stop-opacity: 0.8" />
      </linearGradient>
    </defs>

    <!-- Outer container with rounded corners -->
    <rect
      x="2"
      y="2"
      width="20"
      height="20"
      rx="6"
      ry="6"
      fill="url(#logoGradient)"
    />

    <!-- Inner light/bulb shape -->
    <rect
      x="6"
      y="6"
      width="12"
      height="12"
      rx="2"
      ry="2"
      fill="white"
      opacity="0.9"
    />

    <!-- Light rays effect -->
    <path
      d="M12 4 L12 2 M12 22 L12 20 M20 12 L22 12 M2 12 L4 12 M18.36 5.64 L19.78 4.22 M4.22 19.78 L5.64 18.36 M18.36 18.36 L19.78 19.78 M4.22 4.22 L5.64 5.64"
      stroke="url(#logoGradient)"
      stroke-width="1.5"
      stroke-linecap="round"
      opacity="0.6"
    />
  </svg>
</template>
