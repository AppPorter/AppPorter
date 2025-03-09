<script setup lang="ts">
import { goTo } from '@/plugins/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import Button from 'primevue/button'
import Card from 'primevue/card'
import Panel from 'primevue/panel'
import ProgressBar from 'primevue/progressbar'
import Tooltip from 'primevue/tooltip'
import { useToast } from 'primevue/usetoast'
import { computed, onMounted, ref } from 'vue'

// Installation state
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const extractProgress = ref(0)
const isFinished = ref(false)
const currentStatus = ref('Preparing installation...')
const canClose = ref(false)
const finalExecutablePath = ref('')

const installationConfig = useInstallationConfigStore()
installationConfig.page = 'Progress'
const toast = useToast()

// Compute full installation path including app folder
const fullInstallPath = computed(() => {
  const basePath = installationConfig.install_path
  const appName = installationConfig.name
  if (basePath && appName) {
    return `${basePath.replace(/\\$/, '')}\\${appName}\\`
  }
  return basePath
})

// Helper functions
const getInstallMode = (isCurrentUser: boolean) =>
  isCurrentUser ? 'Current User Only' : 'All Users'

const getShortcutsList = (config: {
  create_desktop_shortcut: boolean
  create_start_menu_shortcut: boolean
  create_registry_key: boolean
}) => {
  const shortcuts = []
  if (config.create_desktop_shortcut) shortcuts.push('Desktop')
  if (config.create_start_menu_shortcut) shortcuts.push('Start Menu')
  if (config.create_registry_key) shortcuts.push('Registry Entry')
  return shortcuts.length ? shortcuts.join(', ') : 'None'
}

// Copy to clipboard handler
const handleCopy = async (text: string, type: string) => {
  try {
    await navigator.clipboard.writeText(text)
    toast.add({
      severity: 'info',
      summary: 'Copied',
      detail: `${type} copied to clipboard`,
      life: 2000,
    })
  } catch (error) {
    console.error('Copy failed:', error)
  }
}

onMounted(() => {
  // Setup event listeners for installation progress
  listen('installation', (event) => {
    if (event.payload === 0) {
      progressMode.value = 'indeterminate'
      currentStatus.value = 'Preparing to extract files...'
    }
    if (event.payload === 101) {
      currentStatus.value = 'Installation completed successfully!'
      isFinished.value = true
      canClose.value = true
    }
  })

  listen('installation_extract', (event) => {
    progressMode.value = 'determinate'
    extractProgress.value = event.payload as number
    currentStatus.value = `Extracting files (${extractProgress.value}%)...`
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
  })
    .then((result) => {
      if (typeof result === 'string') {
        finalExecutablePath.value = result
      }
    })
    .catch((error) => {
      console.error('Installation failed:', error)
      currentStatus.value = 'Installation failed. Please try again.'
      canClose.value = true
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
  <div class="p-1.5 pb-12 flex items-center w-full">
    <Panel
      class="border border-surface-200 dark:border-surface-700 shadow-sm max-w-5xl w-full mx-auto"
    >
      <template #header>
        <div class="flex items-center justify-between py-1 w-full min-w-0">
          <!-- Progress Title -->
          <div class="flex items-center gap-2 min-w-0 flex-shrink">
            <div
              class="p-1.5 rounded-md shrink-0"
              :class="[
                isFinished
                  ? 'bg-green-50 dark:bg-green-900/20'
                  : 'bg-primary-50 dark:bg-primary-900/20',
              ]"
            >
              <span
                class="text-xl mir"
                :class="[
                  isFinished ? 'task_alt' : 'install_desktop',
                  isFinished
                    ? 'text-green-600 dark:text-green-400'
                    : 'text-primary-600 dark:text-primary-400',
                ]"
              ></span>
            </div>
            <div class="min-w-0 flex-shrink">
              <h2 class="text-lg font-medium text-surface-900 dark:text-surface-0">
                Installation Progress
              </h2>
              <p class="text-xs text-surface-600 dark:text-surface-400">
                Installing application to your system
              </p>
            </div>
          </div>

          <!-- App Details -->
          <div class="flex items-center gap-3 shrink-0 ml-4 select-text">
            <div class="text-right">
              <h3 class="text-base font-medium text-surface-900 dark:text-surface-0 leading-none">
                {{ installationConfig.name }}
              </h3>
              <p class="text-xs text-surface-600 dark:text-surface-400 mt-1">
                {{ installationConfig.version || 'Version N/A' }}
              </p>
              <p class="text-xs text-surface-500 dark:text-surface-400 mt-0.5">
                {{ installationConfig.publisher || 'Publisher N/A' }}
              </p>
            </div>
            <div
              class="w-10 h-10 rounded-lg overflow-hidden shrink-0 flex items-center justify-center bg-surface-50 dark:bg-surface-800 border border-surface-200 dark:border-surface-700"
            >
              <img
                v-if="installationConfig.icon"
                :src="installationConfig.icon"
                class="w-8 h-8 object-contain"
                alt="App Icon"
              />
              <span v-else class="mir apps text-2xl text-surface-400 dark:text-surface-600"></span>
            </div>
          </div>
        </div>
      </template>

      <div class="space-y-4">
        <!-- Progress Section -->
        <div class="space-y-2">
          <p
            class="text-sm"
            :class="[
              isFinished
                ? 'text-green-600 dark:text-green-400'
                : 'text-surface-600 dark:text-surface-400',
            ]"
          >
            {{ currentStatus }}
          </p>
          <ProgressBar :mode="progressMode" :value="extractProgress" class="h-1.5" />
        </div>

        <!-- Final Executable Path Card -->
        <Card
          v-if="isFinished"
          class="shadow-none border border-surface-200 dark:border-surface-700"
        >
          <template #title>
            <div class="flex items-center justify-between w-full py-1">
              <div class="flex items-center gap-2">
                <span class="mir terminal text-surface-600 dark:text-surface-400"></span>
                <span class="text-sm font-medium">Installed Location</span>
              </div>
              <Button
                severity="secondary"
                outlined
                v-tooltip.top="'Copy path'"
                class="w-8 h-7"
                icon="mir content_copy"
                @click="handleCopy(finalExecutablePath, 'Executable path')"
              />
            </div>
          </template>
          <template #content>
            <p class="text-sm font-medium break-all select-text">
              {{ finalExecutablePath }}
            </p>
          </template>
        </Card>

        <!-- Installation Details Grid -->
        <div class="grid grid-cols-2 gap-3">
          <!-- Installation Settings Card -->
          <Card class="shadow-none border border-surface-200 dark:border-surface-700">
            <template #title>
              <div class="flex items-center justify-between w-full py-1">
                <div class="flex items-center gap-2">
                  <span class="mir settings text-surface-600 dark:text-surface-400"></span>
                  <span class="text-sm font-medium">Installation Settings</span>
                </div>
                <Button
                  severity="secondary"
                  outlined
                  v-tooltip.top="'Copy settings'"
                  class="w-8 h-7"
                  icon="mir content_copy"
                  @click="
                    handleCopy(
                      `Install Mode: ${getInstallMode(installationConfig.current_user_only)}\nShortcuts: ${getShortcutsList(installationConfig)}\nInstall Path: ${fullInstallPath}`,
                      'Settings'
                    )
                  "
                />
              </div>
            </template>
            <template #content>
              <div class="space-y-3 select-text">
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400">Install Mode</span>
                  <p class="text-sm font-medium">
                    {{ getInstallMode(installationConfig.current_user_only) }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400">Shortcuts</span>
                  <p class="text-sm font-medium">
                    {{ getShortcutsList(installationConfig) }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400">Install Path</span>
                  <p class="text-sm font-medium break-all">
                    {{ fullInstallPath }}
                  </p>
                </div>
              </div>
            </template>
          </Card>

          <!-- Package Information Card -->
          <Card class="shadow-none border border-surface-200 dark:border-surface-700">
            <template #title>
              <div class="flex items-center justify-between w-full py-1">
                <div class="flex items-center gap-2">
                  <span class="mir folder_zip text-surface-600 dark:text-surface-400"></span>
                  <span class="text-sm font-medium">Package Information</span>
                </div>
                <Button
                  severity="secondary"
                  outlined
                  v-tooltip.top="'Copy package info'"
                  class="w-8 h-7"
                  icon="mir content_copy"
                  @click="
                    handleCopy(
                      `Source Archive: ${installationConfig.zip_path}\nSelected Executable: ${installationConfig.executable_path}`,
                      'Package info'
                    )
                  "
                />
              </div>
            </template>
            <template #content>
              <div class="space-y-3 select-text">
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400">Source Archive</span>
                  <p class="text-sm font-medium break-all">
                    {{ installationConfig.zip_path }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400"
                    >Selected Executable</span
                  >
                  <p class="text-sm font-medium break-all">
                    {{ installationConfig.executable_path }}
                  </p>
                </div>
              </div>
            </template>
          </Card>
        </div>

        <!-- Action Buttons -->
        <div class="flex justify-end">
          <Button
            v-if="canClose"
            @click="handleClose"
            :severity="isFinished ? 'success' : 'danger'"
            class="w-24 h-8"
            :icon="isFinished ? 'mir home' : 'mir close'"
            :label="isFinished ? 'Finish' : 'Close'"
          />
        </div>
      </div>
    </Panel>
  </div>
</template>
