<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/styles/icons/material_icons'
import ErrorHandler from '@/components/ErrorHandler.vue'
import { window as tauriWindow } from '@/main'
import { goTo } from '@/router'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { exit } from '@tauri-apps/plugin-process'
import Button from 'primevue/button'
import ConfirmDialog from 'primevue/confirmdialog'
import ConfirmPopup from 'primevue/confirmpopup'
import ContextMenu from 'primevue/contextmenu'
import type { MenuItem } from 'primevue/menuitem'
import Message from 'primevue/message'
import SplitButton from 'primevue/splitbutton'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onBeforeMount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const settingsStore = useSettingsStore()
const confirm = useConfirm()
const { t } = useI18n()
const errorHandler = ref()

const dismissWarning = ref(false)

// First run check and disclaimer handling
onMounted(async () => {
  if (settingsStore.first_run) {
    confirm.require({
      group: 'disclaimer',
      header: t('system.disclaimer.title'),
      message: t('system.disclaimer.message'),
      icon: 'mir-info',
      acceptLabel: t('system.disclaimer.accept'),
      acceptProps: {
        label: t('system.disclaimer.accept'),
        icon: 'mir-check',
        severity: 'primary',
      },
      rejectProps: {
        label: t('system.disclaimer.exit'),
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
    message: t('system.admin.prompt'),
    group: 'admin_popup',
    icon: 'mir-warning',
    rejectProps: {
      label: t('system.admin.dismiss'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      label: t('system.disclaimer.accept'),
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
    reject: () => {},
  })
}

const warningActions = [
  {
    label: t('system.admin.dismiss'),
    command: () => {
      dismissWarning.value = true
    },
  },
]

// Navigation menu configuration
const menuItems = [
  {
    label: t('system.menu.installation'),
    icon: 'mir-install_desktop',
    command: () => goTo('/'),
    paths: ['/Installation/Home', '/Installation/Config', '/Installation/Progress'],
  },
  {
    label: t('system.menu.applist'),
    icon: 'mir-apps',
    command: () => goTo('/AppList'),
    paths: ['/AppList'],
  },
  { separator: true },
  {
    label: t('system.menu.settings'),
    icon: 'mir-settings',
    command: () => goTo('/Settings'),
    paths: ['/Settings'],
    class: 'right',
  },
]

// Context menu setup
const contextMenu = ref()
const editMenuItems = ref<MenuItem[]>([
  {
    label: t('system.edit.cut'),
    icon: 'mir-content_cut',
    command: () => document.execCommand('cut'),
  },
  {
    label: t('system.edit.copy'),
    icon: 'mir-content_copy',
    command: () => document.execCommand('copy'),
  },
  {
    label: t('system.edit.paste'),
    icon: 'mir-content_paste',
    command: async () =>
      document.execCommand('insertText', false, await navigator.clipboard.readText()),
  },
  { separator: true },
  {
    label: t('system.edit.select_all'),
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
      <div class="flex w-full items-center pr-24" style="-webkit-app-region: drag">
        <span class="flex items-center whitespace-nowrap py-3 pl-6 text-lg font-semibold">
          <img src="./assets/appporter.svg" class="mr-1" />AppPorter
        </span>

        <!-- Admin Privileges Warning -->
        <Message
          v-if="!settingsStore.elevated && !dismissWarning"
          size="small"
          severity="warn"
          class="mx-4 w-full py-0"
          icon="mir-warning"
        >
          {{ t('system.admin.warning') }}
          <ConfirmPopup group="admin_popup" />
          <SplitButton
            :label="t('system.admin.solve')"
            outlined
            severity="warn"
            class="left-2 z-40"
            size="small"
            :model="warningActions"
            @click="handleAdminPrompt($event)"
          />
        </Message>
      </div>

      <!-- Navigation Menu -->
      <div class="flex px-4">
        <div
          class="flex w-full items-center justify-between gap-1 border-b border-gray-200 dark:border-gray-700"
        >
          <div class="flex items-center gap-1">
            <Button
              v-for="item in menuItems"
              :key="item.label"
              v-show="!item.separator && !item.class?.includes('right')"
              :class="[
                'relative flex min-w-[120px] items-center justify-center gap-2 px-4 py-3 text-[0.95rem] font-medium transition-all duration-200',
                'text-gray-600 dark:text-gray-300',
                item.paths?.includes(currentPath)
                  ? 'after:absolute after:bottom-0 after:left-1/2 after:h-0.5 after:w-4/5 after:-translate-x-1/2 after:rounded-t-full after:bg-primary-500 after:transition-all dark:after:bg-primary-400'
                  : '',
                'hover:bg-gray-100 dark:hover:bg-gray-800/50',
              ]"
              text
              plain
              @click="item.command"
            >
              <span :class="[item.icon, 'text-[1.1rem]']" />
              <span>{{ item.label }}</span>
            </Button>
          </div>
          <div class="flex items-center gap-1">
            <Button
              v-for="item in menuItems"
              :key="item.label"
              v-show="!item.separator && item.class?.includes('right')"
              :class="[
                'relative flex min-w-[120px] items-center justify-center gap-2 px-4 py-3 text-[0.95rem] font-medium transition-all duration-200',
                'text-gray-600 dark:text-gray-300',
                item.paths?.includes(currentPath)
                  ? 'after:absolute after:bottom-0 after:left-1/2 after:h-0.5 after:w-4/5 after:-translate-x-1/2 after:rounded-t-full after:bg-primary-500 after:transition-all dark:after:bg-primary-400'
                  : '',
                'hover:bg-gray-100 dark:hover:bg-gray-800/50',
              ]"
              text
              plain
              @click="item.command"
            >
              <span :class="[item.icon, 'text-[1.1rem]']" />
              <span>{{ item.label }}</span>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="z-30 flex h-full gap-2 overflow-hidden px-4 pb-2 pt-[6.8rem]">
      <router-view v-slot="{ Component, route }" class="flex w-full">
        <keep-alive :include="cachedComponents ? undefined : []" class="w-full overflow-auto">
          <component :is="Component" :key="route.path" class="flex-1" />
        </keep-alive>
      </router-view>
    </div>

    <!-- Context Menu -->
    <ContextMenu ref="contextMenu" :model="editMenuItems" />
  </div>
</template>
