<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import ConfirmPopup from 'primevue/confirmpopup'
import { useConfirm } from 'primevue/useconfirm'
import { useI18n } from 'vue-i18n'

defineProps<{
  dismissWarning: boolean
}>()

const emit = defineEmits(['dismiss'])

const settingsStore = useSettingsStore()
const confirm = useConfirm()
const { t } = useI18n()

// Admin privileges warning handler
const handleAdminPrompt = (event) => {
  confirm.require({
    target: event.currentTarget,
    message: t('admin.prompt'),
    group: 'admin_popup',
    icon: 'mir-warning',
    rejectProps: {
      label: t('dismiss'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      severity: 'warn',
      label: t('accept'),
    },
    accept: () => {
      if (!settingsStore.debug) {
        invoke('execute_command', {
          command: {
            name: 'Elevate',
            revert: false,
          },
        })
      }
    },
    reject: () => {
      emit('dismiss')
    },
  })
}
</script>

<template>
  <div
    v-if="!settingsStore.elevated && !dismissWarning"
    class="flex min-h-7 flex-wrap items-center gap-x-2 gap-y-1 rounded bg-amber-50 px-3 py-1 dark:bg-amber-900/30"
  >
    <div class="flex items-center gap-2">
      <span class="mir-warning text-amber-600 dark:text-amber-400" />
      <span class="text-sm text-amber-700 dark:text-amber-300">{{ t('admin.warning') }}</span>
      <ConfirmPopup group="admin_popup" />
      <button
        class="rounded-md bg-amber-100 px-2 py-1 text-xs font-medium text-amber-700 transition-colors hover:bg-amber-200 hover:text-amber-800 dark:bg-amber-800/30 dark:text-amber-400 dark:hover:bg-amber-700/50 dark:hover:text-amber-300"
        @click="handleAdminPrompt($event)"
      >
        {{ t('solve') }}
      </button>
    </div>
  </div>
</template>