<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/material_icons'
import { window } from '@/main'
import { goTo } from '@/plugins/router'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { exit } from '@tauri-apps/plugin-process'
import { onBeforeMount, ref } from 'vue'

import ConfirmDialog from 'primevue/confirmdialog'
import ContextMenu from 'primevue/contextmenu'
import Menubar from 'primevue/menubar'
import type { MenuItem } from 'primevue/menuitem'
import Toast from 'primevue/toast'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'

const settingsStore = useSettingsStore()
const confirm = useConfirm()
const toast = useToast()

const dismissWarning = ref(false)

// Window control handlers
function handleClose() {
  if (settingsStore.minimize_to_tray_on_close) {
    window.hide()
  } else {
    exit(0)
  }
}

function handleMinimize() {
  window.minimize()
}

// Admin privileges warning handler
const handleAdminPrompt = (event) => {
  confirm.require({
    target: event.currentTarget,
    message: 'Do you want to restart the application with administrator privileges?',
    group: 'admin_popup',
    icon: 'mir warning',
    rejectProps: {
      label: 'Cancel',
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      label: 'Yes',
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
          summary: 'Restart to apply the change',
          detail:
            'You have to restart the application to run the application with administrator privileges.',
          life: 3000,
        })
      }
    },
    reject: () => {},
  })
}

const warningActions = [
  {
    label: 'Dismiss Once',
    command: () => {
      dismissWarning.value = true
    },
  },
]

// Navigation menu configuration
const menuItems = [
  {
    label: 'Installation',
    icon: 'mir install_desktop',
    command: () => goTo('/'),
  },
  {
    label: 'AppList',
    icon: 'mir apps',
    command: () => goTo('/AppList'),
  },
  { separator: true },
  {
    label: 'Settings',
    icon: 'mir settings',
    command: () => goTo('/Settings'),
    class: 'absolute right-7',
  },
]

// Context menu setup
const contextMenu = ref()
const editMenuItems = ref<MenuItem[]>([
  {
    label: 'Cut',
    icon: 'mir content_cut',
    command: () => document.execCommand('cut'),
  },
  {
    label: 'Copy',
    icon: 'mir content_copy',
    command: () => document.execCommand('copy'),
  },
  {
    label: 'Paste',
    icon: 'mir content_paste',
    command: () => document.execCommand('paste'),
  },
  { separator: true },
  {
    label: 'Select All',
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

onBeforeMount(() => {
  generateMaterialIconsClasses()
})
</script>

<template>
  <div class="select-none w-screen h-screen" @contextmenu="handleContextMenu">
    <!-- System Dialogs -->
    <Toast position="bottom-left" class="z-40" />
    <ConfirmDialog group="dialog"> </ConfirmDialog>

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
    <div class="fixed w-full z-30">
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
          Application is not running with administrator privileges. Some features may be
          unavailable.
          <ConfirmPopup group="admin_popup"></ConfirmPopup>
          <SplitButton
            @click="handleAdminPrompt($event)"
            label="Solve"
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
    <div class="pt-[6.5rem] px-4 pb-2 h-full z-30 gap-2 flex">
      <RouterView />
    </div>

    <!-- Context Menu -->
    <ContextMenu ref="contextMenu" :model="editMenuItems" />
  </div>
</template>
