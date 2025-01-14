<script setup lang="ts">
import { window } from "@/main";
import { useSettingsStore } from "@/stores/settings";
import { exit } from "@tauri-apps/plugin-process";
import Menubar from "primevue/menubar";
import { goTo } from "./plugin/router";

const settingsStore = useSettingsStore();

function close_button() {
  if (settingsStore.minimize_to_tray_on_close) {
    window.hide();
  } else {
    exit(0);
  }
}

function minimize_button() {
  window.minimize();
}

const menu_items = [
  {
    label: "Installation",
    icon: "material-symbols-rounded text-xl",
    iconClass: "install_desktop",
    command: () => goTo("/installation"),
  },
  {
    label: "Settings",
    icon: "material-symbols-rounded text-xl",
    iconClass: "settings",
    command: () => goTo("/settings"),
  },
];
</script>

<template>
  <div class="select-none">
    <!-- Window Controls -->
    <div class="fixed top-0 right-0 h-auto z-50 flex">
      <button
        class="px-3 py-1.5 hover:bg-surface-200 dark:hover:bg-surface-700 transition-colors"
        @click="minimize_button"
      >
        <span class="material-symbols-rounded">remove</span>
      </button>
      <button
        class="px-3 py-1.5 hover:bg-red-600 hover:text-white transition-colors"
        @click="close_button"
      >
        <span class="material-symbols-rounded">close</span>
      </button>
    </div>

    <!-- Title Bar -->
    <div class="fixed w-full z-40">
      <div style="-webkit-app-region: drag" class="w-full">
        <h1 class="text-lg font-semibold pt-3 pl-6 pb-2">AppPorter</h1>
      </div>

      <div class="flex justify-center">
        <Menubar
          :model="menu_items"
          class="border-none shadow-none w-[calc(100vw-40px)]"
        >
          <template #item="{ item }">
            <div class="flex items-center">
              <span :class="[item.icon, 'flex items-center']">{{
                item.iconClass
              }}</span>
              <span class="ml-2">{{ item.label }}</span>
            </div>
          </template>
        </Menubar>
      </div>
    </div>

    <!-- Main Content -->
    <div class="pt-28 px-4 pb-6">
      <RouterView class="z-40" />
    </div>

    <!-- Status Bar -->
    <div
      class="fixed bottom-0 left-0 right-0 h-6 px-4 flex items-center text-sm"
    >
      <p>{{ $route.fullPath }}</p>
    </div>
  </div>
</template>
