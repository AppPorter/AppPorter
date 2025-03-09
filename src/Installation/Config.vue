<script setup lang="ts">
import { goTo } from '@/plugins/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { ref } from 'vue'

// Components
import AppDetails from './components/AppDetails.vue'
import Options from './components/Options.vue'
import ZipPreview from './components/ZipPreview.vue'

const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
installationConfig.page = 'Config'
const toast = useToast()
const confirm = useConfirm()

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

async function handleInstallClick() {
  // Reset validation errors
  nameError.value = ''
  pathError.value = ''

  let hasErrors = false

  // Validate required fields
  if (!installationConfig.executable_path) {
    toast.add({
      severity: 'error',
      summary: 'Executable Missing',
      detail: 'Please select an executable file from the archive',
      life: 3000,
    })
    hasErrors = true
  }

  if (!installationConfig.name) {
    nameError.value = 'Application name is required'
    toast.add({
      severity: 'error',
      summary: 'Missing App Name',
      detail: 'Please enter an application name',
      life: 3000,
    })
    hasErrors = true
  }

  if (!installationConfig.install_path) {
    pathError.value = 'Installation path is required'
    toast.add({
      severity: 'error',
      summary: 'Missing Install Path',
      detail: 'Please select an installation path',
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
        message: 'Do you want to start the installation process now?',
        group: 'dialog',
        icon: 'mir install_desktop',
        header: 'Start Installation',
        rejectProps: {
          label: 'Cancel',
          severity: 'secondary',
          outlined: true,
          icon: 'mir close',
        },
        acceptProps: {
          label: 'Install',
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
      summary: 'Invalid Install Path',
      detail: 'Please select a valid installation path',
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

  <!-- Installation Button -->
  <div class="fixed bottom-4 right-6 z-40">
    <Button
      severity="primary"
      size="large"
      class="w-28 h-8 text-sm shadow-lg hover:shadow-xl transition-all duration-300"
      @click="handleInstallClick"
      icon="mir download"
      label="Install"
    />
  </div>
</template>
