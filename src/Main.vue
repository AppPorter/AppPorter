<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Button } from '@/components/ui/button'
import { goTo } from '@/router'
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu'
</script>

<template>
  <div class="absolute top-8 left-8 w-max">
    <h1 class="text-3xl" style="-webkit-app-region: drag">AppPorter</h1>
    <div class="absolute top-1 right-1">a</div>
    <div class="relative top-2">
      <NavigationMenu>
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
    </div>
    <RouterView class="relative top-4"></RouterView>
  </div>

  <p class="absolute bottom-2 left-2">{{ $route.fullPath }}</p>
</template>
