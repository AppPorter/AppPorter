<script setup lang="ts">
import { goTo } from '@/router';
import { InstallConfigStore } from '@/stores/install_config';
import { invoke } from '@tauri-apps/api/core';
import Button from 'primevue/button';
import { useConfirm } from 'primevue/useconfirm';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import AppDetails from './components/AppDetails.vue';
import Options from './components/AppOptions.vue';

const { t } = useI18n()
const installConfig = InstallConfigStore()
const confirm = useConfirm()

// Set the install config page
installConfig.page = 'Install_App_Config'

// Validation states
const pathError = ref(false)
const nameError = ref(false)

// UI state management
const detailsLoading = ref(false)
const detailsLoadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')

// Handle back button click
function handleBackClick() {
  goTo('/Install/Preview')
}

// Handle install process
async function handleInstallClick() {
  // Reset validation errors
  nameError.value = false
  pathError.value = false

  // Validate required fields
  if (!installConfig.app_details.info.name) {
    nameError.value = true
  }

  if (!installConfig.app_details.paths.parent_install_path) {
    pathError.value = true
  }

  // If any validation failed, return early
  if (nameError.value || pathError.value) {
    return
  }

  try {
    const validatedPath = (await invoke('execute_command', {
      command: {
        name: 'ValidatePath',
        path: installConfig.app_details.paths.parent_install_path,
      },
    })) as string

    installConfig.app_details.paths.parent_install_path = validatedPath
    const fullPath = `${validatedPath}\\${installConfig.app_details.info.name}`

    try {
      await invoke('execute_command', {
        command: {
          name: 'CheckPathEmpty',
          path: fullPath,
        },
      })

      await new Promise((resolve, reject) => {
        confirm.require({
          message: t('ui.install.confirm_install'),
          group: 'dialog',
          icon: 'mir-install_desktop',
          header: t('ui.install.start_install'),
          rejectProps: {
            label: t('g.cancel'),
            severity: 'secondary',
            outlined: true,
            icon: 'mir-close',
          },
          acceptProps: {
            label: t('cls.install.self'),
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
            message: t('ui.valid.directory_not_empty'),
            group: 'dialog',
            icon: 'mir-warning',
            header: t('g.warning'),
            reject: () => reject(),
          })
        })
      } else {
        pathError.value = true
        return
      }
    }

    goTo('/Install/App/Progress')
  } catch {
    pathError.value = true
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
            :details-loading-progress="detailsLoadingProgress" :progress-mode="progressMode" />
          <Options :path-error="pathError" @update:path-error="pathError = $event" />
        </div>
      </div>
    </div>

    <!-- Bottom bar with buttons -->
    <div class="flex items-center justify-between border-t bg-surface-0 px-4 py-3">
      <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleBackClick"
        icon="mir-arrow_back" :label="t('g.back')" outlined />

      <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleInstallClick"
        icon="mir-install_desktop" :label="t('cls.install.self')" />
    </div>
  </div>
</template>
