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
    icon: "install_desktop",
    command: () => {
      goTo("/installation");
    },
  },
  {
    label: "Settings",
    icon: "settings",
    command: () => {
      goTo("/settings");
    },
  },
];
</script>

<template>
  <!-- Window Controls -->
  <div class="fixed top-0 right-0 h-auto z-50 flex">
    <button class="px-3 py-1.5 hover:bg-[#e9e9e9]" @click="minimize_button()">
      <span
        class="material-symbols-rounded w-5 h-5 flex items-center justify-center"
      >
        remove
      </span>
    </button>
    <button class="px-3 py-1.5 hover:bg-[#c42b1c]" @click="close_button()">
      <span
        class="material-symbols-rounded w-5 h-5 flex items-center justify-center"
      >
        close
      </span>
    </button>
  </div>

  <!-- Title Bar -->
  <div class="fixed w-full bg-background/95 backdrop-blur-lg z-40">
    <div style="-webkit-app-region: drag" class="w-full border-b">
      <h1 class="text-lg font-semibold relative pt-3 pl-6 pb-2">AppPorter</h1>
    </div>

    <Menubar :model="menu_items" class="flex">
      <template #item="{ item }">
        <a href="#">
          <span class="material-symbols-rounded">{{ item.icon }}</span>
          <span>{{ item.label }}</span>
        </a>
      </template>
    </Menubar>
  </div>

  <!-- Main Content -->
  <div class="pt-24 px-6 pb-6">
    <Card class="h-[calc(100vh-130px)] overflow-y-auto">
      <RouterView></RouterView>
    </Card>
  </div>

  <!-- Status Bar -->
  <div
    class="fixed bottom-0 left-0 right-0 h-6 border-t px-4 flex items-center text-xs z-40"
  >
    <p class="text-sm">{{ $route.fullPath }}</p>
  </div>
</template>
