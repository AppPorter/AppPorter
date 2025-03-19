<script setup lang="ts">
import Toast from 'primevue/toast'
import { useToast } from 'primevue/usetoast'
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const toast = useToast()
const { t } = useI18n()

// Global error handling toast service
const errorToast = {
  showError: (error: unknown) => {
    let errorMessage = ''
    if (error instanceof Error) {
      errorMessage = error.message
    } else if (typeof error === 'string') {
      errorMessage = error
    } else if (error !== null && typeof error === 'object') {
        errorMessage = JSON.stringify(error)
    } else {
      errorMessage = String(error)
    }

    // Trim long error messages for better display
    if (errorMessage.length > 200) {
      errorMessage = errorMessage.substring(0, 200) + '...'
    }

    toast.add({
      severity: 'error',
      summary: t('system.error.title'),
      detail: errorMessage,
      life: 0, // Persist until user dismisses
    })
  },
}

// Expose error handling for parent components
defineExpose({ errorToast })

// Global error handling setup
onMounted(() => {
  // Override console.error to show errors in toast
  const originalConsoleError = console.error
  console.error = function (...args) {
    // Call original first to preserve logging
    originalConsoleError.apply(console, args)

    // Format error message for toast display
    const errorMessage = args
      .map((arg) =>
        typeof arg === 'object' && arg !== null
          ? arg instanceof Error
            ? arg.message
            : JSON.stringify(arg)
          : String(arg)
      )
      .join(' ')

    errorToast.showError(errorMessage)
  }

  // Capture uncaught window errors
  globalThis.window.addEventListener('error', (event) => {
    errorToast.showError(event.error || event.message)
    return false
  })

  // Capture unhandled promise rejections
  globalThis.window.addEventListener('unhandledrejection', (event) => {
    errorToast.showError(event.reason)
  })
})
</script>

<template>
  <Toast position="bottom-left" class="z-40">
    <template #message="slotProps">
      <div class="flex w-full max-w-[600px] items-center">
        <i
          :class="[
            'mir-mr-2 text-lg',
            {
              info: slotProps.message.severity === 'info',
              warning: slotProps.message.severity === 'warn',
              error: slotProps.message.severity === 'error',
              check_circle: slotProps.message.severity === 'success',
            },
          ]"
        />
        <div class="flex min-w-0 grow flex-col">
          <div class="font-bold">
            {{ slotProps.message.summary }}
          </div>
          <div class="mt-1 cursor-text select-text break-all">
            {{ slotProps.message.detail }}
          </div>
        </div>
      </div>
    </template>
  </Toast>
</template>