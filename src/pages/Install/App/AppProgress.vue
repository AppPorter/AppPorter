<script setup lang="ts">
import { exec } from '@/exec'
import { generalStore, installConfig } from '@/main'
import { goTo } from '@/router'
import { listen } from '@tauri-apps/api/event'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const extractProgress = ref(0)
const isFinished = ref(false)
const currentStatus = ref('')
const canClose = ref(false)
const executablePathCopied = ref(false)
const installPathCopied = ref(false)

const installPath = ref('')
const fullPath = ref('')

generalStore.page = 'Install_App_Progress'
const { t } = useI18n()

const handleOpenExecutable = async () => {
  if (fullPath.value) {
    await exec('OpenApp', {
      path: fullPath.value
    })
  }
}

const handleOpenInstallFolder = async () => {
  await exec('OpenFolder', {
    path: installPath.value
  })
}

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
  currentStatus.value = t('ui.install.progress.preparing')

  const installUnlisten = await listen('app_install_progress', (event) => {
    const payload = event.payload as number
    if (payload === 0) {
      progressMode.value = 'indeterminate'
      currentStatus.value = t('ui.install.progress.preparing_extract')
    } else if (payload === 101) {
      progressMode.value = 'determinate'
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

  try {
    let result = await exec('InstallApp', {
      config: {
        app: {
          id: installConfig.id,
          installed: false,
          url: installConfig.url,
          archive_password: installConfig.archive_password,
          details: installConfig.app_details,
          validation_status: {
            file_exists: false,
            registry_valid: false,
            path_exists: false
          }
        },
        archive_exe_path: installConfig.archive_exe_path,
        archive_path_dir: installConfig.app_details.config.add_to_path[1],
        zip_path: installConfig.zip_path,
      }
    })
    installPath.value = result[0]
    fullPath.value = result[1]
  } catch (error) {
    globalThis.$errorHandler.showError(error)
    currentStatus.value = t('ui.install.progress.failed')
    canClose.value = true
  }

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
    <div class="flex-1 overflow-auto">
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
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

              <div class="space-y-2">
                <p class="text-sm">
                  {{ currentStatus }}
                </p>
                <ProgressBar :mode="progressMode" :value="extractProgress" class="h-1.5" />
              </div>

              <div class="space-y-2">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <span class="mir-folder text-sm"></span>
                    <span class="text-sm font-medium">{{ t('ui.install.path') }}</span>
                  </div>
                  <Button outlined v-tooltip.top="t('ui.install.progress.copy_path')" class="h-7 w-8"
                    :icon="installPathCopied ? 'mir-check' : 'mir-content_copy'"
                    :severity="installPathCopied ? 'success' : 'secondary'"
                    @click="handleCopy(installPath, 'install')" />
                </div>
                <p class="select-text break-all rounded bg-surface-50 p-2 text-sm font-medium dark:bg-surface-800">
                  {{ installPath }}
                </p>
              </div>
            </div>
          </Panel>
        </div>
      </div>
    </div>

    <div class="flex items-center justify-between px-4 py-3">
      <div v-if="isFinished" class="flex items-center gap-2">
        <Button @click="handleOpenExecutable" severity="secondary" outlined class="h-8" icon="mir-terminal"
          :label="t('g.open')" :disabled="!fullPath" />
        <Button @click="handleOpenInstallFolder" severity="secondary" outlined class="h-8" icon="mir-folder"
          :label="t('ui.library.open_install_folder')" />
      </div>

      <div class="flex items-center">
        <Button v-if="canClose" @click="handleClose" :severity="isFinished ? 'primary' : 'danger'" class="h-8 w-24"
          :icon="isFinished ? 'mir-home' : 'mir-close'" :label="isFinished ? t('g.finish') : t('g.close')" />
      </div>
    </div>
  </div>
</template>
