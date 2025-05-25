<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/styles/icons/material_icons'
import AdminWarning from '@/components/Core/AdminWarning.vue'
import ContextMenuManager from '@/components/Core/ContextMenuManager.vue'
import ErrorHandler from '@/components/Core/ErrorHandler.vue'
import NavigationBar from '@/components/Core/NavigationBar.vue'
import WindowControls from '@/components/Core/WindowControls.vue'
import { goTo } from '@/router'
import { useSettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { exit } from '@tauri-apps/plugin-process'
import ConfirmDialog from 'primevue/confirmdialog'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onBeforeMount, onMounted, ref } from 'vue'
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
const contextMenuManager = ref()

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
          name: app.details.info.name,
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

// Route caching control
const route = useRoute()
const cachedComponents = computed(() => {
  return route.path !== '/Home'
})

onBeforeMount(() => {
  generateMaterialIconsClasses()
})
</script>

<template>
  <div class="h-screen w-screen select-none" @contextmenu="contextMenuManager.handleContextMenu">
    <!-- System Dialogs and Notifications -->
    <ErrorHandler ref="errorHandler" />
    <ConfirmDialog group="dialog" />
    <ConfirmDialog group="disclaimer" class="w-[32rem] max-w-[90vw]" :closable="false" />

    <!-- Window Controls -->
    <WindowControls />

    <!-- Title Bar & Navigation -->
    <NavigationBar>
      <template #admin-warning>
        <AdminWarning :dismissWarning="dismissWarning" @dismiss="dismissWarning = true" />
      </template>
    </NavigationBar>

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
    <ContextMenuManager ref="contextMenuManager" />
  </div>
</template>
