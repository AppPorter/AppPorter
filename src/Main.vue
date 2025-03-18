<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/styles/icons/material_icons'
import { window as tauriWindow } from '@/main'
import { goTo } from '@/router'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { exit } from '@tauri-apps/plugin-process'
import ConfirmDialog from 'primevue/confirmdialog'
import ConfirmPopup from 'primevue/confirmpopup'
import ContextMenu from 'primevue/contextmenu'
import type { MenuItem } from 'primevue/menuitem'
import Message from 'primevue/message'
import SplitButton from 'primevue/splitbutton'
import Toast from 'primevue/toast'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { computed, onBeforeMount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const settingsStore = useSettingsStore()
const confirm = useConfirm()
const toast = useToast()
const { t } = useI18n()

// Global error handling toast service
const errorToast = {
  showError: (error: unknown) => {
    let errorMessage = ''
    if (error instanceof Error) {
      errorMessage = error.message
    } else if (typeof error === 'string') {
      errorMessage = error
    } else if (error !== null && typeof error === 'object') {
      try {
        errorMessage = JSON.stringify(error)
      } catch {
        errorMessage = t('system.error.unknown')
      }
    } else {
      errorMessage = String(error)
    }

    // Trim long error messages for better display
    if (errorMessage.length > 200) {
      errorMessage = errorMessage.substring(0, 200) + '...'
    }

    toast.add({
      severity: 'error',
      summary: t('system.error.title'),
      detail: errorMessage,
      life: 0, // Persist until user dismisses
    })
  },
}

// Expose error handling for child components
defineExpose({ errorToast })

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
        toast.add({
          severity: 'info',
          summary: t('system.welcome.title'),
          detail: t('system.welcome.message'),
          life: 3000,
        })
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
        toast.add({
          severity: 'success',
          summary: t('system.admin.success'),
          detail: t('system.admin.restart_message'),
          life: 3000,
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

// Define type for menu item
interface NavMenuItem {
  label?: string
  icon?: string
  command?: () => void
  paths?: string[]
  separator?: boolean
  class?: string
}

// Get active menu item style
const getActiveClass = (item: NavMenuItem) => {
  return item.paths?.includes(currentPath.value)
    ? 'after:absolute after:bottom-[-1px] after:left-0 after:h-0.5 after:w-full after:bg-primary-500 dark:after:bg-primary-400 text-primary-700 dark:text-primary-300'
    : 'hover:bg-gray-100 dark:hover:bg-gray-800'
}

// Update current path when route changes
watch(route, (newRoute) => {
  currentPath.value = newRoute.path
})

onBeforeMount(() => {
  generateMaterialIconsClasses()
})

// Global error handling setup
onMounted(() => {
  // Override console.error to show errors in toast
  const originalConsoleError = console.error
  console.error = function (...args) {
    // Call original first to preserve logging
    originalConsoleError.apply(console, args)

    // Format error message for toast display
    const errorMessage = args
      .map((arg) =>
        typeof arg === 'object' && arg !== null
          ? arg instanceof Error
            ? arg.message
            : JSON.stringify(arg)
          : String(arg)
      )
      .join(' ')

    errorToast.showError(errorMessage)
  }

  // Capture uncaught window errors
  globalThis.window.addEventListener('error', (event) => {
    errorToast.showError(event.error || event.message)
    return false
  })

  // Capture unhandled promise rejections
  globalThis.window.addEventListener('unhandledrejection', (event) => {
    errorToast.showError(event.reason)
  })
})
</script>

<template>
  <div class="h-screen w-screen select-none" @contextmenu="handleContextMenu">
    <!-- System Dialogs and Notifications -->
    <Toast position="bottom-left" class="z-40">
      <template #message="slotProps">
        <div class="flex w-full max-w-[600px] items-center">
          <i
            :class="[
              'mir-mr-2 text-lg',
              {
                info: slotProps.message.severity === 'info',
                warning: slotProps.message.severity === 'warn',
                error: slotProps.message.severity === 'error',
                check_circle: slotProps.message.severity === 'success',
              },
            ]"
          />
          <div class="flex min-w-0 grow flex-col">
            <div class="font-bold">
              {{ slotProps.message.summary }}
            </div>
            <div class="mt-1 cursor-text select-text break-all">
              {{ slotProps.message.detail }}
            </div>
          </div>
        </div>
      </template>
    </Toast>
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
          class="flex w-full items-center justify-between gap-1 border-b border-gray-200 bg-white/80 dark:border-gray-700 dark:bg-gray-900/80"
        >
          <div class="flex items-center gap-1">
            <button
              v-for="item in menuItems"
              :key="item.label"
              v-show="!item.separator && !item.class?.includes('right')"
              :class="[
                'relative flex items-center gap-2 px-4 py-2.5 text-gray-700 transition-colors dark:text-gray-200',
                getActiveClass(item),
              ]"
              @click="item.command"
            >
              <i :class="item.icon" />
              <span>{{ item.label }}</span>
            </button>
          </div>
          <div class="flex items-center gap-1">
            <button
              v-for="item in menuItems"
              :key="item.label"
              v-show="!item.separator && item.class?.includes('right')"
              :class="[
                'relative flex items-center gap-2 px-4 py-2.5 text-gray-700 transition-colors dark:text-gray-200',
                getActiveClass(item),
              ]"
              @click="item.command"
            >
              <i :class="item.icon" />
              <span>{{ item.label }}</span>
            </button>
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
