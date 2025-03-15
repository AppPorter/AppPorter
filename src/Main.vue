<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/styles/icons/material_icons'
import { window as tauriWindow } from '@/main'
import { goTo } from '@/router'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { exit } from '@tauri-apps/plugin-process'
import { computed, onBeforeMount, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'

import ConfirmDialog from 'primevue/confirmdialog'
import ConfirmPopup from 'primevue/confirmpopup'
import ContextMenu from 'primevue/contextmenu'
import Menubar from 'primevue/menubar'
import type { MenuItem } from 'primevue/menuitem'
import Message from 'primevue/message'
import SplitButton from 'primevue/splitbutton'
import Toast from 'primevue/toast'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { useI18n } from 'vue-i18n'

const settingsStore = useSettingsStore()
const confirm = useConfirm()
const toast = useToast()
const { t } = useI18n()

// Global toast service for error handling
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

    // Limit message length for better display
    if (errorMessage.length > 200) {
      errorMessage = errorMessage.substring(0, 200) + '...'
    }

    toast.add({
      severity: 'error',
      summary: t('system.error.title'),
      detail: errorMessage,
      life: 0, // Set to 0 so it will never auto-dismiss
    })
  },
}

// Expose errorToast for other components to use
defineExpose({ errorToast })

const dismissWarning = ref(false)

// Check for first run and show disclaimer
onMounted(async () => {
  if (settingsStore.first_run) {
    confirm.require({
      group: 'disclaimer',
      header: t('system.disclaimer.title'),
      message: t('system.disclaimer.message'),
      icon: 'mir info',
      acceptLabel: t('system.disclaimer.accept'),
      acceptProps: {
        label: t('system.disclaimer.accept'),
        icon: 'mir check',
        severity: 'primary',
      },
      rejectProps: {
        label: t('system.disclaimer.exit'),
        icon: 'mir close',
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
    icon: 'mir warning',
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
    icon: 'mir install_desktop',
    command: () => goTo('/'),
  },
  {
    label: t('system.menu.applist'),
    icon: 'mir apps',
    command: () => goTo('/AppList'),
  },
  { separator: true },
  {
    label: t('system.menu.settings'),
    icon: 'mir settings',
    command: () => goTo('/Settings'),
    class: 'absolute right-7',
  },
]

// Context menu setup
const contextMenu = ref()
const editMenuItems = ref<MenuItem[]>([
  {
    label: t('system.edit.cut'),
    icon: 'mir content_cut',
    command: () => document.execCommand('cut'),
  },
  {
    label: t('system.edit.copy'),
    icon: 'mir content_copy',
    command: () => document.execCommand('copy'),
  },
  {
    label: t('system.edit.paste'),
    icon: 'mir content_paste',
    command: async () =>
      document.execCommand('insertText', false, await navigator.clipboard.readText()),
  },
  { separator: true },
  {
    label: t('system.edit.select_all'),
    icon: 'mir select_all',
    command: () => document.execCommand('selectAll'),
  },
])

function handleContextMenu(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
    contextMenu.value?.show(event)
  }
  event.preventDefault()
}

const route = useRoute()
const cachedComponents = computed(() => {
  return route.path !== '/Installation/Home'
})

onBeforeMount(() => {
  generateMaterialIconsClasses()
})

// Set up global error handling
onMounted(() => {
  // Override console.error to show errors in toast
  const originalConsoleError = console.error
  console.error = function (...args) {
    // Call the original console.error first
    originalConsoleError.apply(console, args)

    // Extract error message and show in toast
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

  // Capture uncaught errors - using globalThis.window instead of imported Tauri window
  globalThis.window.addEventListener('error', (event) => {
    errorToast.showError(event.error || event.message)
    return false
  })

  // Capture unhandled promise rejections - using globalThis.window instead of imported Tauri window
  globalThis.window.addEventListener('unhandledrejection', (event) => {
    errorToast.showError(event.reason)
  })
})
</script>

<template>
  <div class="select-none w-screen h-screen" @contextmenu="handleContextMenu">
    <!-- System Dialogs -->
    <Toast position="bottom-left" class="z-40 custom-toast">
      <template #message="slotProps">
        <div class="flex items-center w-full max-w-[600px]">
          <i
            :class="[
              'mir text-lg mr-2',
              {
                info: slotProps.message.severity === 'info',
                warning: slotProps.message.severity === 'warn',
                error: slotProps.message.severity === 'error',
                check_circle: slotProps.message.severity === 'success',
              },
            ]"
          ></i>
          <div class="flex flex-col grow min-w-0">
            <div class="font-bold">{{ slotProps.message.summary }}</div>
            <div class="select-text cursor-text break-all mt-1">{{ slotProps.message.detail }}</div>
          </div>
        </div>
      </template>
    </Toast>
    <ConfirmDialog group="dialog"> </ConfirmDialog>
    <ConfirmDialog group="disclaimer" class="w-[32rem] max-w-[90vw]" :closable="false">
    </ConfirmDialog>

    <!-- Window Controls -->
    <div class="fixed top-0 right-0 h-auto z-50 flex">
      <button
        class="w-12 h-8 hover:bg-[#e9e9e9] dark:hover:bg-[#2d2d2d] flex items-center justify-center"
        @click="handleMinimize"
      >
        <span class="mir remove"></span>
      </button>
      <button
        class="w-12 h-8 hover:bg-[#c42b1c] flex items-center justify-center"
        @click="handleClose"
      >
        <span class="mir close"></span>
      </button>
    </div>

    <!-- Title Bar & Navigation -->
    <div class="fixed w-full z-30 backdrop-blur-md">
      <div class="flex items-center pr-24 w-full" style="-webkit-app-region: drag">
        <span class="text-lg font-semibold py-3 pl-6 flex items-center whitespace-nowrap">
          <img src="@/assets/appporter.svg" class="mr-1" />AppPorter
        </span>

        <!-- Admin Privileges Warning -->
        <Message
          size="small"
          severity="warn"
          class="mx-4 py-0 w-full"
          v-if="!settingsStore.elevated && !dismissWarning"
          icon="mir warning"
        >
          {{ t('system.admin.warning') }}
          <ConfirmPopup group="admin_popup"></ConfirmPopup>
          <SplitButton
            @click="handleAdminPrompt($event)"
            :label="t('system.admin.solve')"
            outlined
            severity="warn"
            class="z-40 left-2"
            size="small"
            :model="warningActions"
          ></SplitButton>
        </Message>
      </div>

      <!-- Navigation Menu -->
      <div class="flex px-4">
        <Menubar :model="menuItems" class="border-none shadow-none w-full"> </Menubar>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="pt-[6.8rem] px-4 pb-2 h-full z-30 gap-2 flex overflow-hidden">
      <router-view v-slot="{ Component, route }" class="w-full flex">
        <keep-alive :include="cachedComponents ? undefined : []" class="w-full overflow-auto">
          <component :is="Component" :key="route.path" class="flex-1" />
        </keep-alive>
      </router-view>
    </div>

    <!-- Context Menu -->
    <ContextMenu ref="contextMenu" :model="editMenuItems" />
  </div>
</template>
