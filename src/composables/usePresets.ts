import { computed, ref, watch } from 'vue'
import { useDevice } from './useDevice'

export interface Preset {
  id: string
  name: string
  description: string
  temperature: number
  brightness: number
}

export const presets: Preset[] = [
  {
    id: 'cozy-daylight',
    name: 'Cozy Daylight',
    description: 'A soft, balanced daylight effect. Good for a natural look on video calls.',
    temperature: 5200,
    brightness: 60,
  },
  {
    id: 'natural-balance',
    name: 'Natural Balance',
    description: 'Slightly more neutral/white, for accurate color representation.',
    temperature: 5600,
    brightness: 70,
  },
  {
    id: 'cool-blue',
    name: 'Cool Blue',
    description: 'A higher color temperature for a crisp, energetic effect.',
    temperature: 6500,
    brightness: 75,
  },
  {
    id: 'warm-candlelight',
    name: 'Warm Candlelight',
    description: 'A lower color temperature for a warmer, softer, ambient feel.',
    temperature: 3000,
    brightness: 40,
  },
  {
    id: 'bright-companionship',
    name: 'Bright Companionship',
    description: 'A brighter, focused setting for maximum illumination.',
    temperature: 5000,
    brightness: 100,
  },
]

export function usePresets() {
  const device = useDevice()
  const showPresetDropdown = ref(false)
  const manuallySelectedPreset = ref<string | null>(null)

  // Calculate brightness percentage from device values
  const getBrightnessPercentage = (device: any) => {
    if (!device)
      return 0
    const { brightness_lumens, min_brightness_lumens, max_brightness_lumens } = device
    return Math.round(((brightness_lumens - min_brightness_lumens) / (max_brightness_lumens - min_brightness_lumens)) * 100)
  }

  // Detect current preset based on device values
  const detectCurrentPreset = computed(() => {
    if (!device.selectedDevice.value)
      return 'manual'

    const currentTemp = device.selectedDevice.value.temperature_kelvin
    const currentBrightness = getBrightnessPercentage(device.selectedDevice.value)

    // Check if current values match any preset (with some tolerance)
    for (const preset of presets) {
      const tempTolerance = 100 // Allow 100K tolerance
      const brightnessTolerance = 5 // Allow 5% tolerance

      const tempMatch = Math.abs(currentTemp - preset.temperature) <= tempTolerance
      const brightnessMatch = Math.abs(currentBrightness - preset.brightness) <= brightnessTolerance

      if (tempMatch && brightnessMatch) {
        return preset.id
      }
    }

    return 'manual'
  })

  // Current preset (either detected or manually selected)
  const currentPreset = computed(() => {
    // If user manually selected a preset, show that
    if (manuallySelectedPreset.value) {
      return manuallySelectedPreset.value
    }

    // Otherwise, detect from device values
    return detectCurrentPreset.value
  })

  // Get preset by ID
  const getPresetById = (id: string) => {
    return presets.find(p => p.id === id)
  }

  // Apply preset
  const applyPreset = async (preset: Preset) => {
    if (!device.selectedDevice.value)
      return

    manuallySelectedPreset.value = preset.id
    showPresetDropdown.value = false

    // Calculate brightness in lumens
    const brightnessRange = device.selectedDevice.value.max_brightness_lumens - device.selectedDevice.value.min_brightness_lumens
    const targetLumens = Math.round(device.selectedDevice.value.min_brightness_lumens + (brightnessRange * preset.brightness / 100))

    try {
      // Apply temperature
      await device.setTemperatureInKelvin(preset.temperature)

      // Apply brightness
      await device.setBrightnessInLumen(targetLumens)
    }
    catch (error) {
      console.error('Failed to apply preset:', error)
    }
  }

  // Set manual mode
  const setManualMode = () => {
    manuallySelectedPreset.value = null
    showPresetDropdown.value = false
  }

  // Watch for device changes and reset manual selection if values change
  watch(
    [() => device.selectedDevice.value?.temperature_kelvin, () => device.selectedDevice.value?.brightness_lumens],
    () => {
      // If we have a manually selected preset, but the device values changed
      // and no longer match any preset, reset to manual
      if (manuallySelectedPreset.value && detectCurrentPreset.value === 'manual') {
        manuallySelectedPreset.value = null
      }
    },
    { flush: 'post' },
  )

  return {
    presets,
    currentPreset,
    showPresetDropdown,
    detectCurrentPreset,
    getPresetById,
    applyPreset,
    setManualMode,
  }
}
