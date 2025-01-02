<script setup lang="ts">
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import { window } from "@/main";
import { goTo } from "@/router";
import { exit } from "@tauri-apps/plugin-process";
import { storeToRefs } from "pinia";
import { useSettingsStore } from "./stores/settings";

const settingsStore = useSettingsStore();
const { minimize_to_tray_on_close } = storeToRefs(settingsStore);
function close_button() {
  if (minimize_to_tray_on_close.value) {
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
  <div class="fixed top-0 right-0 h-auto z-50">
    <button class="px-2 py-1 hover:bg-[#d7d7d7]" @click="minimize_button()">
      <span v-svg="'minimize'"></span>
    </button>
    <button class="px-2 py-1 hover:bg-[#e81123]" @click="close_button()">
      <span v-svg="'close'"></span>
    </button>
  </div>
  <div class="fixed w-full">
    <div style="-webkit-app-region: drag" class="w-full border-b-2">
      <h1 class="text-3xl relative pt-8 pl-8 pb-4">AppPorter</h1>
    </div>
    <div class="relative left-8">
      <NavigationMenu class="relative top-2">
        <NavigationMenuList>
          <NavigationMenuItem>
            <NavigationMenuLink
              :class="navigationMenuTriggerStyle()"
              @click="goTo('/Installation')"
            >
              Installation</NavigationMenuLink
            >
          </NavigationMenuItem>
          <NavigationMenuItem>
            <NavigationMenuLink
              :class="navigationMenuTriggerStyle()"
              @click="goTo('/Installation/Option')"
            >
              Installation/Option
            </NavigationMenuLink>
          </NavigationMenuItem>
          <NavigationMenuItem>
            <NavigationMenuLink
              :class="navigationMenuTriggerStyle()"
              @click="goTo('/Settings')"
              >Settings
            </NavigationMenuLink>
          </NavigationMenuItem>
        </NavigationMenuList>
      </NavigationMenu>
      <div class="relative top-6">
        <RouterView class="relative"></RouterView>
      </div>
    </div>
  </div>

  <p class="fixed bottom-2 left-2">{{ $route.fullPath }}</p>
</template>
