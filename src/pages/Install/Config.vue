<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from 'primevue'
import Button from 'primevue/button'
import { useConfirm } from 'primevue/useconfirm'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import AppDetails from './components/AppDetails.vue'
import Options from './components/Options.vue'

const installConfig = InstallConfigStore()
installConfig.page = 'Install_App_Config'
const toast = useToast()
const confirm = useConfirm()
const { t } = useI18n()

// UI state management
const detailsLoading = ref(false)
const detailsLoadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')

// Validation states
const pathError = ref('')
const nameError = ref(false)
const executablePathError = ref(false)

function handleBackClick() {
  goTo('/Home')
}

// Handle install process initiation
async function handleInstallClick() {
  // Reset validation errors
  nameError.value = false
  pathError.value = ''
  executablePathError.value = false

  let hasErrors = false

  // Validate required fields
  if (!installConfig.executable_path) {
    executablePathError.value = true
    toast.add({
      severity: 'error',
      summary: t('validation.executable_missing'),
      detail: t('validation.select_executable'),
      life: 3000,
    })
    hasErrors = true
  }

  if (!installConfig.name) {
    nameError.value = true
    toast.add({
      severity: 'error',
      summary: t('validation.name_required'),
      detail: t('validation.enter_name'),
      life: 3000,
    })
    hasErrors = true
  }

  if (!installConfig.install_path) {
    pathError.value = t('validation.select_path')
    toast.add({
      severity: 'error',
      summary: t('validation.path_required'),
      detail: t('validation.select_path'),
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
        path: installConfig.install_path,
      },
    })) as string

    installConfig.details.paths.parent_install_path = validatedPath
    const fullPath = `${validatedPath}\\${installConfig.name}`

    try {
      await invoke('execute_command', {
        command: {
          name: 'CheckPathEmpty',
          path: fullPath,
        },
      })

      await new Promise((resolve, reject) => {
        confirm.require({
          message: t('install.config.confirm_install'),
          group: 'dialog',
          icon: 'mir-install_desktop',
          header: t('install.config.start_install'),
          rejectProps: {
            label: t('cancel'),
            severity: 'secondary',
            outlined: true,
            icon: 'mir-close',
          },
          acceptProps: {
            label: t('install'),
            icon: 'mir-navigate_next',
          },
          accept: () => resolve(true),
          reject: () => reject(),
        })
      })
    } catch (error) {
      if (error === 'Install directory is not empty') {
        await new Promise((resolve, reject) => {
          confirm.require({
            message: t('install.config.directory_not_empty'),
            group: 'dialog',
            icon: 'mir-warning',
            header: t('warning'),
            rejectProps: {
              label: t('cancel'),
              severity: 'secondary',
              outlined: true,
              icon: 'mir-close',
            },
            acceptProps: {
              label: t('install.config.force_install'),
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

    goTo('/Install/Progress')
  } catch (error) {
    pathError.value = error as string
  }
}
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <!-- Main scrollable container -->
    <div class="flex-1 overflow-auto">
      <!-- Content wrapper -->
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
          <AppDetails :name-error="nameError" :details-loading="detailsLoading"
            :details-loading-progress="detailsLoadingProgress" :progress-mode="progressMode"
            :executable-path-error="executablePathError" />
          <Options :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
        </div>
      </div>

      <!-- Button container -->
      <div class="mt-4 flex justify-between px-1 pb-2">
        <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleBackClick"
          icon="mir-arrow_back" :label="t('back')" outlined />
        <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleInstallClick"
          icon="mir-install_desktop" :label="t('install')" />
      </div>
    </div>
  </div>
</template>
