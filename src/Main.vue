<script setup lang="ts">
import { window } from "@/main";
import { useSettingsStore } from "@/stores/settings";
import { exit } from "@tauri-apps/plugin-process";
import Card from "primevue/card";
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
  <div
    class="fixed w-full bg-surface-0/95 dark:bg-surface-900/95 backdrop-blur-lg z-40 border-b border-surface-200 dark:border-surface-700"
  >
    <div style="-webkit-app-region: drag" class="w-full">
      <h1 class="text-lg font-semibold pt-3 pl-6 pb-2">AppPorter</h1>
    </div>

    <Menubar :model="menu_items" class="border-none shadow-none">
      <template #item="{ item }">
        <span :class="item.icon">{{ item.iconClass }}</span>
        <span class="ml-2">{{ item.label }}</span>
      </template>
    </Menubar>
  </div>

  <!-- Main Content -->
  <div class="pt-28 px-4 pb-6">
    <Card class="h-[calc(100vh-150px)] overflow-y-auto">
      <template #content>
        <RouterView class="z-40" />
      </template>
    </Card>
  </div>

  <!-- Status Bar -->
  <div
    class="fixed bottom-0 left-0 right-0 h-6 border-t border-surface-200 dark:border-surface-700 px-4 flex items-center text-xs text-surface-600 dark:text-surface-400 bg-surface-50 dark:bg-surface-900"
  >
    <p>{{ $route.fullPath }}</p>
  </div>
</template>
