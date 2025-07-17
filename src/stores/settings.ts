/// Settings store for managing application preferences with Pinia.
/// 
/// This store handles user preferences, app settings, and persistent configuration.

import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AppSettings } from '../types'

const SETTINGS_STORAGE_KEY = 'litra-control-settings'

const defaultSettings: AppSettings = {
  theme: 'system',
  autoRefreshInterval: 5000,
  showNotifications: true,
  defaultBrightness: 50,
  defaultTemperature: 4000
}

export const useSettingsStore = defineStore('settings', () => {
  // State
  const settings = ref<AppSettings>({ ...defaultSettings })
  const isInitialized = ref(false)

  // Load settings from localStorage
  const loadSettings = (): void => {
    try {
      const stored = localStorage.getItem(SETTINGS_STORAGE_KEY)
      if (stored) {
        const parsedSettings = JSON.parse(stored)
        settings.value = { ...defaultSettings, ...parsedSettings }
      }
    } catch (error) {
      console.warn('Failed to load settings from localStorage:', error)
      settings.value = { ...defaultSettings }
    }
    isInitialized.value = true
  }

  // Save settings to localStorage
  const saveSettings = (): void => {
    try {
      localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(settings.value))
    } catch (error) {
      console.warn('Failed to save settings to localStorage:', error)
    }
  }

  // Watch for changes and auto-save
  watch(
    settings,
    () => {
      if (isInitialized.value) {
        saveSettings()
      }
    },
    { deep: true }
  )

  // Actions
  const updateSettings = (newSettings: Partial<AppSettings>): void => {
    settings.value = { ...settings.value, ...newSettings }
  }

  const resetSettings = (): void => {
    settings.value = { ...defaultSettings }
  }

  const setTheme = (theme: AppSettings['theme']): void => {
    settings.value.theme = theme
    applyTheme(theme)
  }

  const setAutoRefreshInterval = (interval: number): void => {
    settings.value.autoRefreshInterval = Math.max(1000, interval) // Minimum 1 second
  }

  const toggleNotifications = (): void => {
    settings.value.showNotifications = !settings.value.showNotifications
  }

  const setDefaultBrightness = (brightness: number): void => {
    settings.value.defaultBrightness = Math.max(0, Math.min(100, brightness))
  }

  const setDefaultTemperature = (temperature: number): void => {
    // Round to nearest 100K and ensure valid range
    const rounded = Math.round(temperature / 100) * 100
    settings.value.defaultTemperature = Math.max(2700, Math.min(6500, rounded))
  }

  // Theme application
  const applyTheme = (theme: AppSettings['theme']): void => {
    const root = document.documentElement
    
    if (theme === 'system') {
      // Use system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.classList.toggle('dark', prefersDark)
    } else {
      root.classList.toggle('dark', theme === 'dark')
    }
  }

  // Initialize theme system preference listener
  const initializeTheme = (): void => {
    applyTheme(settings.value.theme)
    
    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
      if (settings.value.theme === 'system') {
        applyTheme('system')
      }
    })
  }

  // Export/Import settings
  const exportSettings = (): string => {
    return JSON.stringify(settings.value, null, 2)
  }

  const importSettings = (settingsJson: string): boolean => {
    try {
      const importedSettings = JSON.parse(settingsJson)
      
      // Validate imported settings
      const validatedSettings: Partial<AppSettings> = {}
      
      if (['light', 'dark', 'system'].includes(importedSettings.theme)) {
        validatedSettings.theme = importedSettings.theme
      }
      
      if (typeof importedSettings.autoRefreshInterval === 'number' && importedSettings.autoRefreshInterval >= 1000) {
        validatedSettings.autoRefreshInterval = importedSettings.autoRefreshInterval
      }
      
      if (typeof importedSettings.showNotifications === 'boolean') {
        validatedSettings.showNotifications = importedSettings.showNotifications
      }
      
      if (typeof importedSettings.defaultBrightness === 'number' && 
          importedSettings.defaultBrightness >= 0 && 
          importedSettings.defaultBrightness <= 100) {
        validatedSettings.defaultBrightness = importedSettings.defaultBrightness
      }
      
      if (typeof importedSettings.defaultTemperature === 'number' && 
          importedSettings.defaultTemperature >= 2700 && 
          importedSettings.defaultTemperature <= 6500 &&
          importedSettings.defaultTemperature % 100 === 0) {
        validatedSettings.defaultTemperature = importedSettings.defaultTemperature
      }
      
      updateSettings(validatedSettings)
      return true
    } catch (error) {
      console.warn('Failed to import settings:', error)
      return false
    }
  }

  // Initialize store
  const initialize = (): void => {
    loadSettings()
    initializeTheme()
  }

  return {
    // State
    settings,
    isInitialized,

    // Actions
    updateSettings,
    resetSettings,
    setTheme,
    setAutoRefreshInterval,
    toggleNotifications,
    setDefaultBrightness,
    setDefaultTemperature,
    exportSettings,
    importSettings,
    initialize,

    // Theme utilities
    applyTheme,
    initializeTheme
  }
})