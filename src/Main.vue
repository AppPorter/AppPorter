<script setup lang="ts">
import { generateMaterialIconsClasses } from '@/assets/styles/icons/material_icons'
import AdminWarning from '@/components/Core/AdminWarning.vue'
import ContextMenuManager from '@/components/Core/ContextMenuManager.vue'
import ErrorHandler from '@/components/Core/ErrorHandler.vue'
import Listener from '@/components/Core/Listener.vue'
import NavigationBar from '@/components/Core/NavigationBar.vue'
import Uninstall from '@/components/Core/Uninstall.vue'
import WindowControls from '@/components/Core/WindowControls.vue'
import PreviewDrawer from '@/components/Drawer/PreviewDrawer.vue'
import { invoke } from '@tauri-apps/api/core'
import ConfirmDialog from 'primevue/confirmdialog'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onBeforeMount, onMounted, provide, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import { trayIcon } from './main.ts'
import { AppTypes } from './stores/library.ts'
import { SettingsStore } from './stores/settings.ts'

const confirm = useConfirm()
const { t } = useI18n()
const errorHandler = ref()
const dismissWarning = ref(false)
const settings = SettingsStore()
const contextMenuManager = ref()
const uninstallComponent = ref()

// Provide uninstall function to child components
const triggerUninstall = async (apptype: AppTypes, timestamp: number) => {
  await uninstallComponent.value?.confirmAndUninstall(apptype, timestamp)
}

provide('triggerUninstall', triggerUninstall)

// Setup event listeners after component is mounted
onMounted(async () => {
  // First run check
  if (settings.first_run) {
    confirm.require({
      group: 'disclaimer',
      header: t('ui.disclaimer.header'),
      message: t('ui.disclaimer.msg'),
      icon: 'mir-info',
      acceptProps: {
        label: t('g.accept'),
        icon: 'mir-check',
        severity: 'primary',
      },
      rejectProps: {
        label: t('g.exit'),
        icon: 'mir-close',
        severity: 'secondary',
        outlined: true,
      },
      accept: async () => {
        await settings.acknowledgeFirstRun()
      },
      reject: () => {
        invoke('execute_command', {
          command: { name: 'Exit', code: 0 },
        })
      },
    })
  }
})

// Route caching control
const route = useRoute()
const cachedComponents = computed(() => {
  return route.path !== '/Home'
})

const handleBeforeUnload = () => {
  if (trayIcon) {
    trayIcon.close()
  }
};

onMounted(() => {
  window.addEventListener('beforeunload', handleBeforeUnload);
});

onBeforeMount(() => {
  generateMaterialIconsClasses()
})
</script>

<template>
  <div class="h-screen w-screen select-none" @contextmenu="contextMenuManager.handleContextMenu">
    <!-- Event Listeners -->
    <Listener />

    <!-- System Dialogs and Notifications -->
    <ErrorHandler ref="errorHandler" />
    <ConfirmDialog group="dialog" class="w-[32rem] max-w-[90vw]" />
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

    <!-- Preview Drawer -->
    <PreviewDrawer />

    <!-- Uninstall Component -->
    <Uninstall ref="uninstallComponent" />
  </div>
</template>
