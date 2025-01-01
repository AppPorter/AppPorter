<script setup lang="ts">
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import { goTo } from "@/router";
</script>

<template>
  <div class="fixed top-0 right-0 h-auto z-50">
    <button class="px-2 py-1 hover:bg-[#d7d7d7]">
      <span v-svg="'minimize'"></span>
    </button>
    <button class="px-2 py-1 hover:bg-[#e81123]">
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
