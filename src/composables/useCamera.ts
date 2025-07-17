import { onUnmounted, ref } from 'vue'

export interface CameraDevice {
  deviceId: string
  label: string
  kind: string
}

export function useCamera() {
  const videoRef = ref<HTMLVideoElement>()
  const currentStream = ref<MediaStream | null>(null)
  const availableCameras = ref<CameraDevice[]>([])
  const selectedCameraId = ref<string>('')
  const isStreaming = ref(false)
  const error = ref<string>('')
  const permissionDenied = ref(false)
  const isLinux = ref(false)

  // Get available cameras
  const getCameras = async () => {
    try {
      const devices = await navigator.mediaDevices.enumerateDevices()
      availableCameras.value = devices
        .filter(device => device.kind === 'videoinput')
        .map(device => ({
          deviceId: device.deviceId,
          label: device.label || `Camera ${device.deviceId.slice(0, 8)}`,
          kind: device.kind,
        }))

      // Select first camera by default
      if (availableCameras.value.length > 0 && !selectedCameraId.value) {
        selectedCameraId.value = availableCameras.value[0].deviceId
      }
    }
    catch (err) {
      error.value = 'Failed to get camera devices'
      console.error('Error getting cameras:', err)
    }
  }

  // Start camera stream
  const startStream = async (deviceId?: string) => {
    try {
      error.value = ''
      permissionDenied.value = false

      // Stop existing stream first
      if (currentStream.value) {
        stopStream()
      }

      const constraints: MediaStreamConstraints = {
        video: {
          deviceId: deviceId ? { exact: deviceId } : undefined,
          width: { ideal: 1280 },
          height: { ideal: 720 },
          facingMode: 'user',
        },
        audio: false,
      }

      const stream = await navigator.mediaDevices.getUserMedia(constraints)
      currentStream.value = stream

      if (videoRef.value) {
        videoRef.value.srcObject = stream
        videoRef.value.play()
      }

      isStreaming.value = true

      // Get updated camera list with labels
      await getCameras()
    }
    catch (err: any) {
      isStreaming.value = false

      if (err.name === 'NotAllowedError') {
        error.value = 'Camera access denied. Please allow camera permissions.'
        permissionDenied.value = true
      }
      else if (err.name === 'NotFoundError') {
        error.value = 'No camera found'
      }
      else if (err.name === 'NotReadableError') {
        error.value = 'Camera is already in use'
      }
      else {
        error.value = 'Failed to start camera'
      }

      console.error('Error starting camera:', err)
    }
  }

  // Stop camera stream
  const stopStream = () => {
    if (currentStream.value) {
      currentStream.value.getTracks().forEach(track => track.stop())
      currentStream.value = null
    }

    if (videoRef.value) {
      videoRef.value.srcObject = null
    }

    isStreaming.value = false
  }

  // Switch camera
  const switchCamera = async (deviceId: string) => {
    selectedCameraId.value = deviceId
    if (isStreaming.value) {
      await startStream(deviceId)
    }
  }

  // Check if camera is supported
  const isCameraSupported = () => {
    return !!(navigator.mediaDevices && navigator.mediaDevices.getUserMedia)
  }

  // Check current permission status
  const checkPermission = async () => {
    try {
      const result = await navigator.permissions.query({ name: 'camera' as PermissionName })
      return result.state // 'granted', 'denied', 'prompt'
    }
    catch (err) {
      console.warn('Permission API not supported')
      return 'prompt'
    }
  }

  // Request camera permission
  const requestPermission = async () => {
    try {
      error.value = ''
      permissionDenied.value = false

      // Try to get a temporary stream just to trigger permission
      const stream = await navigator.mediaDevices.getUserMedia({
        video: { width: { min: 1 }, height: { min: 1 } },
        audio: false,
      })

      // Stop the temporary stream immediately
      stream.getTracks().forEach(track => track.stop())

      return true
    }
    catch (err: any) {
      if (err.name === 'NotAllowedError') {
        error.value = 'Camera permission denied. Please allow camera access in your system settings.'
        permissionDenied.value = true
        return false
      }
      throw err
    }
  }

  // Check if running on Linux
  const checkPlatform = () => {
    const userAgent = navigator.userAgent.toLowerCase()
    const platform = navigator.platform.toLowerCase()
    isLinux.value = platform.includes('linux') || userAgent.includes('linux')
  }

  // Initialize
  const initialize = async () => {
    checkPlatform()

    if (isLinux.value) {
      error.value = 'Camera features are not available on Linux systems'
      return
    }

    if (!isCameraSupported()) {
      error.value = 'Camera not supported on this system'
      return
    }

    await getCameras()
  }

  // Cleanup on unmount
  onUnmounted(() => {
    stopStream()
  })

  return {
    videoRef,
    currentStream,
    availableCameras,
    selectedCameraId,
    isStreaming,
    error,
    permissionDenied,
    isLinux,
    getCameras,
    startStream,
    stopStream,
    switchCamera,
    isCameraSupported,
    checkPermission,
    requestPermission,
    initialize,
  }
}
