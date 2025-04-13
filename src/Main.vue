<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/styles/icons/material_icons'
import ErrorHandler from '@/components/ErrorHandler.vue'
import { window as tauriWindow } from '@/main'
import { goTo } from '@/router'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { exit } from '@tauri-apps/plugin-process'
import Button from 'primevue/button'
import ConfirmDialog from 'primevue/confirmdialog'
import ConfirmPopup from 'primevue/confirmpopup'
import ContextMenu from 'primevue/contextmenu'
import type { MenuItem } from 'primevue/menuitem'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onBeforeMount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import { useAppListStore } from './stores/app_list'
import { useInstallationConfigStore } from './stores/installation_config'

const settingsStore = useSettingsStore()
const confirm = useConfirm()
const { t } = useI18n()
const errorHandler = ref()
const appListStore = useAppListStore()
const dismissWarning = ref(false)
const InstallationConfig = useInstallationConfigStore()

// Setup event listeners after component is mounted
onMounted(async () => {
  // First run check
  if (settingsStore.first_run) {
    confirm.require({
      group: 'disclaimer',
      header: t('disclaimer.title'),
      message: t('disclaimer.message'),
      icon: 'mir-info',
      acceptProps: {
        label: t('accept'),
        icon: 'mir-check',
        severity: 'primary',
      },
      rejectProps: {
        label: t('exit'),
        icon: 'mir-close',
        severity: 'secondary',
        outlined: true,
      },
      accept: async () => {
        await settingsStore.acknowledgeFirstRun()
      },
      reject: () => {
        exit(0)
      },
    })
  }

  // Setup install listener
  await listen('install', (event) => {
    InstallationConfig.zip_path = event.payload as string
    goTo('/Installation/Config')
  })

  // Setup uninstall listener
  await listen('uninstall', async (event) => {
    goTo('/AppList')
    await appListStore.loadAppList()
    const app = appListStore.getAppByTimestamp(event.payload as number)
    if (!app) return
    await new Promise((resolve, reject) => {
      confirm.require({
        message: t('confirm_uninstall_message', {
          name: app.details.name,
        }),
        group: 'dialog',
        header: t('confirm_uninstall_header'),
        icon: 'mir-warning',
        rejectProps: {
          label: t('cancel'),
          severity: 'secondary',
          outlined: true,
          icon: 'mir-close',
        },
        acceptProps: {
          label: t('uninstall'),
          severity: 'danger',
          icon: 'mir-warning',
        },
        accept: async () => {
          await appListStore.executeUninstall(event.payload as number)
          resolve(true)
        },
        reject: () => reject(),
      })
    })
  })

  await listen('installWithTimestamp', (event) => {
    const payload = event.payload as { zip_path: string; timestamp: number }
    InstallationConfig.zip_path = payload[0]
    InstallationConfig.timestamp = payload[1]
    goTo('/Installation/Config')
  })

  // Execute initial command
  invoke('execute_command', {
    command: {
      name: 'Cli',
    },
  })
})

// Window control handlers
function handleClose() {
  if (settingsStore.minimize_to_tray_on_close) {
    tauriWindow.hide()
  } else {
    exit(0)
  }
}

function handleMinimize() {
  tauriWindow.minimize()
}

// Admin privileges warning handler
const handleAdminPrompt = (event) => {
  confirm.require({
    target: event.currentTarget,
    message: t('admin.prompt'),
    group: 'admin_popup',
    icon: 'mir-warning',
    rejectProps: {
      label: t('dismiss'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      severity: 'warn',
      label: t('accept'),
    },
    accept: () => {
      if (!settingsStore.debug) {
        invoke('execute_command', {
          command: {
            name: 'Elevate',
            revert: false,
          },
        })
      }
    },
    reject: () => {
      dismissWarning.value = true
    },
  })
}

// Navigation menu configuration
const leftMenuItems = [
  {
    label: t('installation.self'),
    icon: 'mir-install_desktop',
    command: () => goTo('/'),
    paths: ['/Installation/Home', '/Installation/Config', '/Installation/Progress'],
  },
  {
    label: t('app_list.self'),
    icon: 'mir-apps',
    command: () => goTo('/AppList'),
    paths: ['/AppList'],
  },
]

const rightMenuItems = [
  {
    label: t('settings.self'),
    icon: 'mir-settings',
    command: () => goTo('/Settings'),
    paths: ['/Settings'],
  },
]

// Context menu setup
const contextMenu = ref()
const editMenuItems = ref<MenuItem[]>([
  {
    label: t('edit.cut'),
    icon: 'mir-content_cut',
    command: () => document.execCommand('cut'),
  },
  {
    label: t('edit.copy'),
    icon: 'mir-content_copy',
    command: () => document.execCommand('copy'),
  },
  {
    label: t('edit.paste'),
    icon: 'mir-content_paste',
    command: async () =>
      document.execCommand('insertText', false, await navigator.clipboard.readText()),
  },
  { separator: true },
  {
    label: t('edit.select_all'),
    icon: 'mir-select_all',
    command: () => document.execCommand('selectAll'),
  },
])

// Show context menu only for text input elements
function handleContextMenu(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
    contextMenu.value?.show(event)
  }
  event.preventDefault()
}

// Route caching control
const route = useRoute()
const cachedComponents = computed(() => {
  return route.path !== '/Installation/Home'
})

const currentPath = ref(route.path)

// Update current path when route changes
watch(route, (newRoute) => {
  currentPath.value = newRoute.path
})

onBeforeMount(() => {
  generateMaterialIconsClasses()
})

// Navigation styles
const navButtonClass = computed(() => [
  'relative flex min-w-[120px] items-center justify-center gap-2 px-4 py-3 text-[0.95rem] font-medium',
  'text-gray-600 dark:text-gray-300',
  'transition-colors duration-200 ease-in-out',
  'hover:bg-gray-100 hover:text-gray-900 dark:hover:bg-gray-800/50 dark:hover:text-gray-100',
])

const activeIndicatorClass = computed(
  () => (active: boolean) =>
    active
      ? 'after:absolute after:bottom-0 after:left-1/2 after:h-0.5 after:w-4/5 after:-translate-x-1/2 after:rounded-t-full after:bg-primary-500 after:transition-all after:duration-300 after:ease-out dark:after:bg-primary-400'
      : 'after:absolute after:bottom-0 after:left-1/2 after:h-0.5 after:w-0 after:-translate-x-1/2 after:rounded-t-full after:bg-primary-500 after:opacity-0 after:transition-all after:duration-300 after:ease-out dark:after:bg-primary-400'
)
</script>

<template>
  <div class="h-screen w-screen select-none" @contextmenu="handleContextMenu">
    <!-- System Dialogs and Notifications -->
    <ErrorHandler ref="errorHandler" />
    <ConfirmDialog group="dialog" />
    <ConfirmDialog group="disclaimer" class="w-[32rem] max-w-[90vw]" :closable="false" />

    <!-- Window Controls -->
    <div class="fixed right-0 top-0 z-50 flex h-auto">
      <button
        class="flex h-8 w-12 items-center justify-center hover:bg-[#e9e9e9] dark:hover:bg-[#2d2d2d]"
        @click="handleMinimize"
      >
        <span class="mir-remove" />
      </button>
      <button
        class="group flex h-8 w-12 items-center justify-center hover:bg-[#c42b1c]"
        @click="handleClose"
      >
        <span class="mir-close group-hover:text-white" />
      </button>
    </div>

    <!-- Title Bar & Navigation -->
    <div class="fixed z-30 w-full backdrop-blur-md">
      <div class="flex w-full items-center justify-between pr-24" style="-webkit-app-region: drag">
        <!-- Left Side with Logo -->
        <span class="flex items-center whitespace-nowrap px-6 py-3 text-lg font-semibold">
          <img src="./assets/appporter.svg" class="mr-1" />AppPorter
        </span>

        <!-- Admin Warning -->
        <div class="flex flex-1 justify-center">
          <div
            v-if="!settingsStore.elevated && !dismissWarning"
            class="flex min-h-7 flex-wrap items-center gap-x-2 gap-y-1 rounded bg-amber-50 px-3 py-1 dark:bg-amber-900/30"
          >
            <div class="flex items-center gap-2">
              <span class="mir-warning text-amber-600 dark:text-amber-400" />
              <span class="text-sm text-amber-700 dark:text-amber-300">{{
                t('admin.warning')
              }}</span>
              <ConfirmPopup group="admin_popup" />
              <button
                class="rounded-md bg-amber-100 px-2 py-1 text-xs font-medium text-amber-700 transition-colors hover:bg-amber-200 hover:text-amber-800 dark:bg-amber-800/30 dark:text-amber-400 dark:hover:bg-amber-700/50 dark:hover:text-amber-300"
                @click="handleAdminPrompt($event)"
              >
                {{ t('solve') }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Navigation Menu -->
      <div class="flex px-4">
        <div
          class="flex w-full items-center justify-between gap-1 border-b border-gray-200 dark:border-gray-700"
        >
          <div class="flex items-center gap-1">
            <Button
              v-for="item in leftMenuItems"
              :key="item.label"
              :class="[...navButtonClass, activeIndicatorClass(item.paths?.includes(currentPath))]"
              text
              plain
              @click="item.command"
            >
              <span
                :class="[
                  item.icon,
                  'text-[1.1rem] transition-transform duration-200 ease-in-out group-hover:scale-105',
                ]"
              />
              <span>{{ item.label }}</span>
            </Button>
          </div>
          <div class="flex items-center gap-1">
            <Button
              v-for="item in rightMenuItems"
              :key="item.label"
              :class="[...navButtonClass, activeIndicatorClass(item.paths?.includes(currentPath))]"
              text
              plain
              @click="item.command"
            >
              <span
                :class="[
                  item.icon,
                  'text-[1.1rem] transition-transform duration-200 ease-in-out group-hover:scale-105',
                ]"
              />
              <span>{{ item.label }}</span>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="z-30 flex h-full gap-2 overflow-hidden px-4 pb-2 pt-[6.5rem]">
      <Suspense>
        <template #default>
          <router-view v-slot="{ Component, route }" class="flex w-full">
            <keep-alive :include="cachedComponents ? undefined : []" class="w-full overflow-auto">
              <component :is="Component" :key="route.path" class="flex-1" />
            </keep-alive>
          </router-view>
        </template>
        <template #fallback>
          <div class="flex size-full items-center justify-center">
            <span class="mir-autorenew animate-spin text-3xl text-primary-500" />
          </div>
        </template>
      </Suspense>
    </div>

    <!-- Context Menu -->
    <ContextMenu ref="contextMenu" :model="editMenuItems" />
  </div>
</template>
