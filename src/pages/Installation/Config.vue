<script setup lang="ts">
import { goTo } from '@/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import { useConfirm } from 'primevue/useconfirm'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import AppDetails from './components/AppDetails.vue'
import Options from './components/Options.vue'
import ZipPreview from './components/ZipPreview.vue'

const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
installationConfig.page = 'Config'
const confirm = useConfirm()
const { t } = useI18n()

// UI state management
const detailsLoading = ref(false)
const detailsLoadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')

// Validation states
const pathError = ref('')
const nameError = ref('')

// Update progress indicator for details loading
function handleDetailsProgress(value: number) {
  progressMode.value = 'indeterminate'
  detailsLoadingProgress.value = value
}

function handleBackClick() {
  goTo('/Installation/Home')
}

// Handle installation process initiation
async function handleInstallClick() {
  await invoke('execute_command', {
    command: {
      name: 'ValidatePath',
      path: installationConfig.install_path,
    },
  })

  // Confirm installation intent and proceed
  await new Promise((resolve, reject) => {
    confirm.require({
      message: t('installation.config.confirm_install'),
      group: 'dialog',
      icon: 'mir-install_desktop',
      header: t('installation.config.start_installation'),
      rejectProps: {
        label: t('installation.config.cancel'),
        severity: 'secondary',
        outlined: true,
        icon: 'mir-close',
      },
      acceptProps: {
        label: t('installation.config.install'),
        icon: 'mir-navigate_next',
      },
      accept: () => resolve(true),
      reject: () => reject(),
    })
  })

  goTo('/Installation/Progress')
}
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <div class="flex flex-1 flex-wrap gap-4 overflow-auto p-1 md:flex-nowrap">
      <div class="min-w-72 flex-1 space-y-2">
        <Options :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
        <AppDetails
          :name-error="nameError"
          :details-loading="detailsLoading"
          :details-loading-progress="detailsLoadingProgress"
          :progress-mode="progressMode"
        />
      </div>

      <div class="size-full max-h-[calc(100vh-11rem)] min-w-72 md:w-2/5">
        <ZipPreview
          :zip-path="zip_path"
          :details-loading="detailsLoading"
          @loading="(value) => (detailsLoading = value)"
          @progress="handleDetailsProgress"
          class="h-full"
        />
      </div>
    </div>

    <div class="fixed bottom-4 left-6 z-40">
      <Button
        severity="secondary"
        class="h-8 w-28 text-sm backdrop-blur-md transition-all duration-200"
        @click="handleBackClick"
        icon="mir-arrow_back"
        :label="t('installation.config.back')"
        outlined
      />
    </div>

    <div class="fixed bottom-4 right-10 z-40">
      <Button
        severity="primary"
        class="h-8 w-28 text-sm transition-all duration-200"
        @click="handleInstallClick"
        icon="mir-install_desktop"
        :label="t('installation.config.install')"
      />
    </div>
  </div>
</template>
