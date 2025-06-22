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
    toast.add({
      severity: 'error',
      summary: t('g.error'),
      detail: String(error),
      life: 0, // Persist until user dismisses
    })
  },
}

// Make error handler globally available
globalThis.$errorHandler = errorToast

// Expose error handling for parent components
defineExpose({ errorToast })

// Global error handling setup
onMounted(() => {
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
        <i :class="[
          'mir-mr-2 text-lg',
          {
            info: slotProps.message.severity === 'info',
            warning: slotProps.message.severity === 'warn',
            error: slotProps.message.severity === 'error',
            check_circle: slotProps.message.severity === 'success',
          },
        ]" />
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
