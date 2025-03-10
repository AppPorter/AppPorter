<script setup lang="ts">
import { goTo } from '@/plugins/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

// Components
import AppDetails from './components/AppDetails.vue'
import Options from './components/Options.vue'
import ZipPreview from './components/ZipPreview.vue'

const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
installationConfig.page = 'Config'
const toast = useToast()
const confirm = useConfirm()
const { t } = useI18n()

// UI state management
const detailsLoading = ref(false)
const detailsLoadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')

// Validation states
const pathError = ref('')
const nameError = ref('')

function handleDetailsProgress(value: number) {
  progressMode.value = 'indeterminate'
  detailsLoadingProgress.value = value
}

// Return to Home page
function handleBackClick() {
  goTo('/Installation/Home')
}

async function handleInstallClick() {
  // Reset validation errors
  nameError.value = ''
  pathError.value = ''

  let hasErrors = false

  // Validate required fields
  if (!installationConfig.executable_path) {
    toast.add({
      severity: 'error',
      summary: t('installation.validation.executable_missing'),
      detail: t('installation.validation.select_executable'),
      life: 3000,
    })
    hasErrors = true
  }

  if (!installationConfig.name) {
    nameError.value = t('installation.validation.name_required')
    toast.add({
      severity: 'error',
      summary: t('installation.validation.name_required'),
      detail: t('installation.validation.enter_name'),
      life: 3000,
    })
    hasErrors = true
  }

  if (!installationConfig.install_path) {
    pathError.value = t('installation.validation.path_required')
    toast.add({
      severity: 'error',
      summary: t('installation.validation.path_required'),
      detail: t('installation.validation.select_path'),
      life: 3000,
    })
    hasErrors = true
  }

  // Stop if validation errors exist
  if (hasErrors) {
    return
  }

  try {
    // Validate installation path on backend
    await invoke('execute_command', {
      command: {
        name: 'ValidatePath',
        path: installationConfig.install_path,
      },
    })

    // Confirm installation intent
    await new Promise((resolve, reject) => {
      confirm.require({
        message: t('installation.config.confirm_install'),
        group: 'dialog',
        icon: 'mir install_desktop',
        header: t('installation.config.start_installation'),
        rejectProps: {
          label: t('installation.config.cancel'),
          severity: 'secondary',
          outlined: true,
          icon: 'mir close',
        },
        acceptProps: {
          label: t('installation.config.install'),
          icon: 'mir navigate_next',
        },
        accept: () => resolve(true),
        reject: () => reject(),
      })
    })

    goTo('/Installation/Progress')
  } catch (error) {
    pathError.value = error
    toast.add({
      severity: 'error',
      summary: t('installation.validation.invalid_path'),
      detail: t('installation.validation.select_valid_path'),
      life: 3000,
    })
  }
}
</script>

<template>
  <!-- Left Column: Options & App Details -->
  <div class="flex-1 min-w-[25rem] space-y-2">
    <Options :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
    <AppDetails
      :name-error="nameError"
      :details-loading="detailsLoading"
      :details-loading-progress="detailsLoadingProgress"
      :progress-mode="progressMode"
    />
  </div>

  <!-- Right Column: ZIP Preview -->
  <div class="min-w-[22rem] w-[40%] h-[calc(100vh-10.625rem)]">
    <ZipPreview
      :zip-path="zip_path"
      :details-loading="detailsLoading"
      @loading="(value) => (detailsLoading = value)"
      @progress="handleDetailsProgress"
    />
  </div>

  <!-- Back Button -->
  <div class="fixed bottom-4 left-6 z-40">
    <Button
      severity="secondary"
      size="large"
      class="w-28 h-8 text-sm shadow-lg hover:shadow-xl transition-all duration-300"
      @click="handleBackClick"
      icon="mir arrow_back"
      :label="t('installation.config.back')"
      outlined
    />
  </div>

  <!-- Installation Button -->
  <div class="fixed bottom-4 right-6 z-40">
    <Button
      severity="primary"
      size="large"
      class="w-28 h-8 text-sm shadow-lg hover:shadow-xl transition-all duration-300"
      @click="handleInstallClick"
      icon="mir download"
      :label="t('installation.config.install')"
    />
  </div>
</template>
