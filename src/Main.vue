<script setup lang="ts">
import AdminWarning from '@/components/Core/AdminWarning.vue'
import ContextMenu from '@/components/Core/ContextMenu.vue'
import Disclaimer from '@/components/Core/Disclaimer.vue'
import ErrorHandler from '@/components/Core/ErrorHandler.vue'
import Listener from '@/components/Core/Listener.vue'
import NavigationBar from '@/components/Core/NavigationBar.vue'
import WindowControls from '@/components/Core/WindowControls.vue'
import DirectorySelector from '@/components/Drawer/DirectorySelector.vue'
import Preview from '@/components/Drawer/Preview.vue'
import Reinstall from '@/components/Drawer/Reinstall.vue'
import Uninstall from '@/components/Drawer/Uninstall.vue'
import { generateMaterialIconsClasses } from '@/styles/material_icons.ts'
import ConfirmDialog from 'primevue/confirmdialog'
import { computed, onBeforeMount, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { trayIcon } from './main.ts'

const errorHandler = ref()
const dismissWarning = ref(false)
const contextMenu = ref()

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
  <div class="h-screen w-screen select-none" @contextmenu="contextMenu.handleContextMenu">
    <Listener />

    <ErrorHandler ref="errorHandler" />
    <ConfirmDialog group="dialog" class="w-[32rem] max-w-[90vw]" />
    <Disclaimer />

    <WindowControls />

    <NavigationBar>
      <template #admin-warning>
        <AdminWarning :dismissWarning="dismissWarning" @dismiss="dismissWarning = true" />
      </template>
    </NavigationBar>

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

    <ContextMenu ref="contextMenu" />

    <Preview />
    <Uninstall />
    <Reinstall />
    <DirectorySelector />
  </div>
</template>
