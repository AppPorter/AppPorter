<script setup lang="ts">
import { exec } from '@/exec'
import { envStore } from '@/main';
import ConfirmPopup from 'primevue/confirmpopup'
import { useConfirm } from 'primevue/useconfirm'
import { useI18n } from 'vue-i18n'

defineProps<{
  dismissWarning: boolean
}>()

const emit = defineEmits(['dismiss'])

const confirm = useConfirm()
const { t } = useI18n()

const handleAdminPrompt = () => {
  confirm.require({
    message: t('ui.admin_prompt.msg'),
    group: 'admin_popup',
    icon: 'mir-warning',
    rejectProps: {
      label: t('g.dismiss'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      severity: 'warn',
      label: t('g.accept'),
    },
    accept: () => {
      if (!envStore.debug) {
        exec('Elevate', {
          revert: false,
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
  <div v-if="!envStore.elevated && !dismissWarning"
    class="flex min-h-7 flex-wrap items-center gap-x-2 gap-y-1 rounded bg-amber-50 px-3 py-1 dark:bg-amber-900/30">
    <div class="flex items-center gap-2">
      <span class="mir-warning text-amber-600 dark:text-amber-400" />
      <span class="text-sm text-amber-700 dark:text-amber-300">{{ t('g.warning') }}</span>
      <ConfirmPopup group="admin_popup" />
      <button
        class="rounded-md bg-amber-100 px-2 py-1 text-xs font-medium text-amber-700 transition-colors hover:bg-amber-200 hover:text-amber-800 dark:bg-amber-800/30 dark:text-amber-400 dark:hover:bg-amber-700/50 dark:hover:text-amber-300"
        @click="handleAdminPrompt()">
        {{ t('g.solve') }}
      </button>
    </div>
  </div>
</template>
