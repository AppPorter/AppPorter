<script setup lang="ts">
import { goTo } from '@/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import Button from 'primevue/button'
import Panel from 'primevue/panel'
import ProgressBar from 'primevue/progressbar'
import Tooltip from 'primevue/tooltip'
import { useToast } from 'primevue/usetoast'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

// Installation state
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const extractProgress = ref(0)
const isFinished = ref(false)
const currentStatus = ref(null)
const canClose = ref(false)
const finalExecutablePath = ref('')

const installationConfig = useInstallationConfigStore()
installationConfig.page = 'Progress'
const toast = useToast()
const { t } = useI18n()

// Compute full installation path including app folder
const fullInstallPath = computed(() => {
  const basePath = installationConfig.install_path
  const appName = installationConfig.name
  if (basePath && appName) {
    return `${basePath.replace(/\\$/, '')}\\${appName}\\`
  }
  return basePath
})

// Helper functions to format and display installation information
const getInstallMode = (isCurrentUser: boolean) =>
  isCurrentUser
    ? t('settings.installation.current_user_only.title')
    : t('settings.installation.all_users.title')

const getShortcutsList = (config: {
  create_desktop_shortcut: boolean
  create_start_menu_shortcut: boolean
  create_registry_key: boolean
}) => {
  const shortcuts = []
  if (config.create_desktop_shortcut) shortcuts.push(t('settings.installation.desktop_shortcut'))
  if (config.create_start_menu_shortcut) shortcuts.push(t('settings.installation.start_menu'))
  if (config.create_registry_key) shortcuts.push(t('settings.installation.registry_key'))
  return shortcuts.length ? shortcuts.join(', ') : t('installation.config.shortcuts')
}

// Copy information to clipboard with feedback
const handleCopy = async (text: string, type: string) => {
  await navigator.clipboard.writeText(text)
  toast.add({
    severity: 'info',
    summary: t('system.edit.copy'),
    detail: `${type} copied to clipboard`,
    life: 2000,
  })
}

onMounted(() => {
  // Initial installation setup
  currentStatus.value = t('installation.progress.preparing')

  // Setup event listeners for installation progress
  listen('installation', (event) => {
    if (event.payload === 0) {
      progressMode.value = 'indeterminate'
      currentStatus.value = t('installation.progress.preparing_extract')
    }
    if (event.payload === 101) {
      currentStatus.value = t('installation.progress.completed')
      isFinished.value = true
      canClose.value = true
    }
  })

  listen('installation_extract', (event) => {
    progressMode.value = 'determinate'
    extractProgress.value = event.payload as number
    currentStatus.value = t('installation.progress.extracting', { progress: extractProgress.value })
  })

  // Start installation process
  invoke('execute_command', {
    command: {
      name: 'Installation',
      config: {
        zip_path: installationConfig.zip_path,
        details: {
          name: installationConfig.name,
          icon: installationConfig.icon,
          publisher: installationConfig.publisher,
          version: installationConfig.version,
          current_user_only: installationConfig.current_user_only,
          create_desktop_shortcut: installationConfig.create_desktop_shortcut,
          create_registry_key: installationConfig.create_registry_key,
          create_start_menu_shortcut: installationConfig.create_start_menu_shortcut,
          install_path: installationConfig.install_path,
          executable_path: installationConfig.executable_path,
        },
      },
    },
  }).then((result) => {
    finalExecutablePath.value = result as string
  })
})

const handleClose = () => {
  goTo('/Installation/Home')
}

// Register tooltip directive
defineOptions({
  directives: {
    tooltip: Tooltip,
  },
})
</script>

<template>
  <div class="flex size-full flex-col">
    <div class="flex-1 overflow-auto p-1.5 pb-6">
      <Panel class="mx-auto w-full max-w-5xl">
        <template #header>
          <div class="flex w-full min-w-0 items-center justify-between py-1">
            <!-- Progress Title -->
            <div class="flex min-w-0 shrink items-center gap-2">
              <div class="shrink-0 rounded-md p-1.5">
                <span
                  class="mir-text-xl"
                  :class="[
                    isFinished ? 'mir-task_alt' : 'mir-install_desktop',
                    isFinished
                      ? 'text-green-600 dark:text-green-400'
                      : 'text-primary-600 dark:text-primary-400',
                  ]"
                ></span>
              </div>
              <div class="min-w-0 shrink">
                <h2 class="text-lg font-medium">
                  {{ t('installation.progress.title') }}
                </h2>
                <p class="text-xs">
                  {{ t('installation.progress.description') }}
                </p>
              </div>
            </div>

            <!-- App Details -->
            <div class="ml-4 flex shrink-0 select-text items-center gap-3">
              <div class="text-right">
                <h3 class="text-base font-medium leading-none">
                  {{ installationConfig.name }}
                </h3>
                <p class="mt-1 text-xs">
                  {{ installationConfig.version || 'Version N/A' }}
                </p>
                <p class="mt-0.5 text-xs">
                  {{ installationConfig.publisher || 'Publisher N/A' }}
                </p>
              </div>
              <div
                class="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800"
              >
                <img
                  v-if="installationConfig.icon"
                  :src="installationConfig.icon"
                  class="size-8 object-contain"
                  alt="App Icon"
                />
                <span v-else class="mir-apps text-2xl"></span>
              </div>
            </div>
          </div>
        </template>

        <div class="space-y-4">
          <!-- Progress Section -->
          <div class="space-y-2">
            <p class="text-sm" :class="[isFinished ? 'text-green-600 dark:text-green-400' : '']">
              {{ currentStatus }}
            </p>
            <ProgressBar :mode="progressMode" :value="extractProgress" class="h-1.5" />
          </div>

          <div
            v-if="isFinished"
            class="rounded-lg border border-slate-200 p-4 dark:border-zinc-600"
          >
            <div class="flex w-full items-center justify-between py-1">
              <div class="flex items-center gap-2">
                <span class="mir-terminal"></span>
                <span class="text-sm font-medium">{{ t('installation.progress.full_path') }}</span>
              </div>
              <Button
                severity="secondary"
                outlined
                v-tooltip.top="t('installation.progress.copy_path')"
                class="h-7 w-8"
                icon="mir-content_copy"
                @click="handleCopy(finalExecutablePath, t('installation.progress.executable_path'))"
              />
            </div>
            <p class="select-text break-all text-sm font-medium">
              {{ finalExecutablePath }}
            </p>
          </div>

          <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
            <div class="rounded-lg border border-slate-200 p-4 dark:border-zinc-600">
              <div class="flex w-full items-center justify-between py-1">
                <div class="flex items-center gap-2">
                  <span class="mir-settings"></span>
                  <span class="text-sm font-medium">{{
                    t('installation.progress.install_settings')
                  }}</span>
                </div>
                <Button
                  severity="secondary"
                  outlined
                  v-tooltip.top="t('installation.progress.copy_settings')"
                  class="h-7 w-8"
                  icon="mir-content_copy"
                  @click="
                    handleCopy(
                      `Install Mode: ${getInstallMode(installationConfig.current_user_only)}\nShortcuts: ${getShortcutsList(installationConfig)}\nInstall Path: ${fullInstallPath}`,
                      'Settings'
                    )
                  "
                />
              </div>
              <div class="select-text space-y-3">
                <div class="space-y-1">
                  <span class="text-sm">{{ t('installation.config.install_mode') }}</span>
                  <p class="text-sm font-medium">
                    {{ getInstallMode(installationConfig.current_user_only) }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm">{{ t('installation.config.shortcuts') }}</span>
                  <p class="text-sm font-medium">
                    {{ getShortcutsList(installationConfig) }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm">{{ t('installation.config.install_path') }}</span>
                  <p class="break-all text-sm font-medium">
                    {{ fullInstallPath }}
                  </p>
                </div>
              </div>
            </div>

            <div class="rounded-lg border border-slate-200 p-4 dark:border-zinc-600">
              <div class="flex w-full items-center justify-between py-1">
                <div class="flex items-center gap-2">
                  <span class="mir-folder_zip"></span>
                  <span class="text-sm font-medium">{{
                    t('installation.progress.package_info')
                  }}</span>
                </div>
                <Button
                  severity="secondary"
                  outlined
                  v-tooltip.top="t('installation.progress.copy_package_info')"
                  class="h-7 w-8"
                  icon="mir-content_copy"
                  @click="
                    handleCopy(
                      `Source Archive: ${installationConfig.zip_path}\nSelected Executable: ${installationConfig.executable_path}`,
                      'Package info'
                    )
                  "
                />
              </div>
              <div class="select-text space-y-3">
                <div class="space-y-1">
                  <span class="text-sm">{{ t('installation.progress.source_archive') }}</span>
                  <p class="break-all text-sm font-medium">
                    {{ installationConfig.zip_path }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm">{{ t('installation.progress.selected_executable') }}</span>
                  <p class="break-all text-sm font-medium">
                    {{ installationConfig.executable_path }}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div class="flex justify-end">
            <Button
              v-if="canClose"
              @click="handleClose"
              :severity="isFinished ? 'success' : 'danger'"
              class="h-8 w-24"
              :icon="isFinished ? 'mir-home' : 'mir-close'"
              :label="
                isFinished ? t('installation.progress.finish') : t('installation.progress.close')
              "
            />
          </div>
        </div>
      </Panel>
    </div>
  </div>
</template>
