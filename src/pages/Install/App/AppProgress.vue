<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

// Install state
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const extractProgress = ref(0)
const isFinished = ref(false)
const currentStatus = ref('')
const canClose = ref(false)
const finalExecutablePath = ref('')
const executablePathCopied = ref(false)
const installPathCopied = ref(false)

const installConfig = InstallConfigStore()
installConfig.page = 'Install_App_Progress'
const { t } = useI18n()

// Handle opening executable
const handleOpenExecutable = async () => {
  if (finalExecutablePath.value) {
    await invoke('execute_command', {
      command: {
        name: 'OpenApp',
        path: finalExecutablePath.value
      }
    })
  }
}

// Handle opening install folder
const handleOpenInstallFolder = async () => {
  console.log(fullInstallPath.value)
  await invoke('execute_command', {
    command: {
      name: 'OpenFolder',
      path: fullInstallPath.value
    }
  })
}

// Compute full install path including app folder
const fullInstallPath = computed(() => {
  const basePath = installConfig.app_details.paths.parent_install_path
  const appName = installConfig.app_details.info.name
  if (!basePath || !appName) return basePath || ''
  return `${basePath.replace(/\\$/, '')}\\${appName}\\`
})

// Copy information to clipboard with feedback
const handleCopy = async (text: string, type: 'executable' | 'install') => {
  await navigator.clipboard.writeText(text)
  if (type === 'executable') {
    executablePathCopied.value = true
    setTimeout(() => {
      executablePathCopied.value = false
    }, 2000)
  } else {
    installPathCopied.value = true
    setTimeout(() => {
      installPathCopied.value = false
    }, 2000)
  }
}

onMounted(async () => {
  // Initial install setup
  currentStatus.value = t('ui.install.progress.preparing')

  // Setup event listener for install progress
  const installUnlisten = await listen('app_install_progress', (event) => {
    const payload = event.payload as number
    if (payload === 0) {
      progressMode.value = 'indeterminate'
      currentStatus.value = t('ui.install.progress.preparing_extract')
    } else if (payload === 101) {
      extractProgress.value = 100
      currentStatus.value = ''
      isFinished.value = true
      canClose.value = true
    } else if (payload > 0 && payload <= 100) {
      progressMode.value = 'determinate'
      extractProgress.value = payload
      currentStatus.value = t('ui.install.progress.extracting', { progress: extractProgress.value })
    }
  })

  // Start install process
  try {
    const result = await invoke('execute_command', {
      command: {
        name: 'InstallApp',
        config: {
          zip_path: installConfig.zip_path,
          password: installConfig.archive_password,
          details: installConfig.app_details,
          timestamp: installConfig.timestamp,
        },
      },
    })
    finalExecutablePath.value = result as string
  } catch (error) {
    console.error('Install failed:', error)
    currentStatus.value = t('ui.install.progress.failed')
    canClose.value = true
  }

  // Cleanup listeners on unmount
  return () => {
    installUnlisten()
  }
})

const handleClose = () => {
  goTo('/Home')
}
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <!-- Main scrollable container -->
    <div class="flex-1 overflow-auto">
      <!-- Content wrapper -->
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
          <!-- Main progress panel -->
          <Panel class="shadow-sm">
            <template #header>
              <div class="flex items-center gap-1.5">
                <span class="mir-text-lg" :class="[
                  isFinished ? 'mir-task_alt' : 'mir-install_desktop',
                  'text-primary-600 dark:text-primary-400',
                ]"></span>
                <h2 class="text-base font-medium">
                  {{ t('ui.install.progress.title') }}
                </h2>
              </div>
            </template>

            <div class="space-y-4 p-2">
              <!-- App info section -->
              <div class="flex items-center gap-3">
                <div
                  class="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                  <img v-if="installConfig.app_details.info.icon" :src="installConfig.app_details.info.icon"
                    class="size-8 object-contain" alt="App Icon" />
                  <span v-else class="mir-apps text-2xl"></span>
                </div>
                <div class="min-w-0 flex-1">
                  <h3 class="text-base font-medium leading-none">
                    {{ installConfig.app_details.info.name }}
                  </h3>
                  <p class="mt-1 text-xs">
                    {{ installConfig.app_details.info.version || 'Version N/A' }}
                  </p>
                  <p class="mt-0.5 text-xs">
                    {{ installConfig.app_details.info.publisher || 'Publisher N/A' }}
                  </p>
                </div>
              </div>

              <!-- Progress section -->
              <div class="space-y-2">
                <p class="text-sm">
                  {{ currentStatus }}
                </p>
                <ProgressBar :mode="progressMode" :value="extractProgress" class="h-1.5" />
              </div>

              <!-- Install path section -->
              <div class="space-y-2">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <span class="mir-folder text-sm"></span>
                    <span class="text-sm font-medium">{{ t('ui.install.path') }}</span>
                  </div>
                  <Button outlined v-tooltip.top="t('ui.install.progress.copy_path')" class="h-7 w-8"
                    :icon="installPathCopied ? 'mir-check' : 'mir-content_copy'"
                    :severity="installPathCopied ? 'success' : 'secondary'"
                    @click="handleCopy(fullInstallPath, 'install')" />
                </div>
                <p class="select-text break-all rounded bg-surface-50 p-2 text-sm font-medium dark:bg-surface-800">
                  {{ fullInstallPath }}
                </p>
              </div>
            </div>
          </Panel>
        </div>
      </div>
    </div>

    <!-- Bottom bar with buttons -->
    <div class="flex items-center justify-between px-4 py-3">
      <!-- Action buttons (shown only when finished) -->
      <div v-if="isFinished" class="flex items-center gap-2">
        <Button @click="handleOpenExecutable" severity="secondary" outlined class="h-8" icon="mir-terminal"
          :label="t('g.open')" :disabled="!finalExecutablePath" />
        <Button @click="handleOpenInstallFolder" severity="secondary" outlined class="h-8" icon="mir-folder"
          :label="t('ui.library.open_install_folder')" />
      </div>

      <!-- Close/Finish button -->
      <div class="flex items-center">
        <Button v-if="canClose" @click="handleClose" :severity="isFinished ? 'primary' : 'danger'" class="h-8 w-24"
          :icon="isFinished ? 'mir-home' : 'mir-close'" :label="isFinished ? t('g.finish') : t('g.close')" />
      </div>
    </div>
  </div>
</template>
