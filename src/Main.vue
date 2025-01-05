<script setup lang="ts">
import { Card } from "@/components/ui/card";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import { window } from "@/main";
import { goTo } from "@/plugin/router";
import { useSettingsStore } from "@/stores/settings";
import { exit } from "@tauri-apps/plugin-process";

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
</script>

<template>
  <!-- Window Controls -->
  <div class="fixed top-0 right-0 h-auto z-50 flex">
    <button
      class="px-3 py-1.5 hover:bg-[#d7d7d7] transition-colors"
      @click="minimize_button()"
    >
      <span
        v-svg="'minimize'"
        class="w-3 h-3 overflow-hidden flex items-center justify-center"
      ></span>
    </button>
    <button
      class="px-3 py-1.5 hover:bg-[#e81123] transition-colors"
      @click="close_button()"
    >
      <span
        v-svg="'close'"
        class="w-3 h-3 overflow-hidden flex items-center justify-center"
      ></span>
    </button>
  </div>

  <!-- Title Bar -->
  <div class="fixed w-full bg-background/95 backdrop-blur-lg z-40">
    <div style="-webkit-app-region: drag" class="w-full border-b">
      <h1 class="text-lg font-semibold relative pt-3 pl-6 pb-2">AppPorter</h1>
    </div>

    <!-- Navigation -->
    <div class="px-6 py-1">
      <NavigationMenu>
        <NavigationMenuList>
          <NavigationMenuItem>
            <NavigationMenuLink
              :class="[navigationMenuTriggerStyle(), 'px-4']"
              @click="goTo('/Installation')"
            >
              Installation
            </NavigationMenuLink>
          </NavigationMenuItem>
          <NavigationMenuItem>
            <NavigationMenuLink
              :class="[navigationMenuTriggerStyle(), 'px-4']"
              @click="goTo('/Settings')"
            >
              Settings
            </NavigationMenuLink>
          </NavigationMenuItem>
        </NavigationMenuList>
      </NavigationMenu>
    </div>
  </div>

  <!-- Main Content -->
  <div class="pt-20 px-6 pb-8">
    <Card class="h-[calc(100vh-120px)] overflow-y-auto">
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
