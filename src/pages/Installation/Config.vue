<script setup lang="ts">
import { goTo } from '@/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from 'primevue'
import Button from 'primevue/button'
import { useConfirm } from 'primevue/useconfirm'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import AppDetails from './components/AppDetails.vue'
import Options from './components/Options.vue'

const installationConfig = useInstallationConfigStore()
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

function handleBackClick() {
  goTo('/Installation/Home')
}

// Handle installation process initiation
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

  if (hasErrors) {
    return
  }

  try {
    const validatedPath = (await invoke('execute_command', {
      command: {
        name: 'ValidatePath',
        path: installationConfig.install_path,
      },
    })) as string

    installationConfig.install_path = validatedPath
    const fullPath = `${validatedPath}\\${installationConfig.name}`

    try {
      await invoke('execute_command', {
        command: {
          name: 'CheckPathEmpty',
          path: fullPath,
        },
      })

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
    } catch (error) {
      if (error === 'Installation directory is not empty') {
        await new Promise((resolve, reject) => {
          confirm.require({
            message: t('installation.config.directory_not_empty'),
            group: 'dialog',
            icon: 'mir-warning',
            header: t('installation.config.warning'),
            rejectProps: {
              label: t('installation.config.cancel'),
              severity: 'secondary',
              outlined: true,
              icon: 'mir-close',
            },
            acceptProps: {
              label: t('installation.config.force_install'),
              severity: 'danger',
              icon: 'mir-warning',
            },
            accept: () => resolve(true),
            reject: () => reject(),
          })
        })
      } else {
        pathError.value = error as string
        return
      }
    }

    goTo('/Installation/Progress')
  } catch (error) {
    pathError.value = error as string
  }
}
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <!-- Main scrollable container -->
    <div class="flex-1 overflow-auto">
      <!-- Content wrapper with additional space at bottom -->
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
          <Options :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
          <AppDetails
            :name-error="nameError"
            :details-loading="detailsLoading"
            :details-loading-progress="detailsLoadingProgress"
            :progress-mode="progressMode"
          />
        </div>
      </div>

      <!-- This is the key element that creates additional scrollable space -->
      <div class="mt-4 h-20 w-full"></div>
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
