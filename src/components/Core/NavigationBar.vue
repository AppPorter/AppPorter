<script setup lang="ts">
import { goTo } from '@/router'
import Button from 'primevue/button'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const { t } = useI18n()
const route = useRoute()

const leftMenuItems = [
  {
    label: t('cls.install.self'),
    icon: 'mir-install_desktop',
    command: () => goTo('/Install'),
    paths: ['/Home', '/Install/App/Config', '/Install/Tool/Config', '/Install/App/Progress', '/Install/Tool/Progress'],
  },
  {
    label: t('cls.library.self'),
    icon: 'mir-apps',
    command: () => goTo('/Library'),
    paths: ['/Library'],
  },
]

const rightMenuItems = [
  {
    label: t('cls.settings.self'),
    icon: 'mir-settings',
    command: () => goTo('/Settings'),
    paths: ['/Settings'],
  },
]

const currentPath = ref(route.path)

watch(route, (newRoute) => {
  currentPath.value = newRoute.path
})

const navButtonClass = computed(() => [
  'relative flex min-w-[120px] items-center justify-center gap-2 px-4 py-3 text-[0.95rem] font-medium',
  'text-gray-600 dark:text-gray-300',
  'transition-colors duration-200 ease-in-out',
  'hover:bg-gray-100 hover:text-gray-900 dark:hover:bg-gray-800/50 dark:hover:text-gray-100',
])

const activeIndicatorClass = computed(
  () => (active: boolean) =>
    active
      ? 'after:absolute after:bottom-0 after:left-1/2 after:h-0.5 after:w-4/5 after:-translate-x-1/2 after:rounded-t-full after:bg-primary-500 after:transition-all after:duration-300 after:ease-out dark:after:bg-primary-400'
      : 'after:absolute after:bottom-0 after:left-1/2 after:h-0.5 after:w-0 after:-translate-x-1/2 after:rounded-t-full after:bg-primary-500 after:opacity-0 after:transition-all after:duration-300 after:ease-out dark:after:bg-primary-400'
)
</script>

<template>
  <div class="fixed z-30 w-full backdrop-blur-md">
    <div class="flex w-full items-center justify-between pr-24" style="-webkit-app-region: drag">
      <span class="flex items-center whitespace-nowrap px-6 py-3 text-lg font-semibold">
        <img src="@/assets/appporter.svg" class="mr-1" />AppPorter
      </span>

      <div class="flex flex-1 justify-center">
        <slot name="admin-warning"></slot>
      </div>
    </div>

    <div class="flex px-4">
      <div class="flex w-full items-center justify-between gap-1 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-1">
          <Button v-for="item in leftMenuItems" :key="item.label"
            :class="[...navButtonClass, activeIndicatorClass(item.paths?.includes(currentPath))]" text plain
            @click="item.command">
            <span :class="[
              item.icon,
              'text-[1.1rem] transition-transform duration-200 ease-in-out group-hover:scale-105',
            ]" />
            <span>{{ item.label }}</span>
          </Button>
        </div>
        <div class="flex items-center gap-1">
          <Button v-for="item in rightMenuItems" :key="item.label"
            :class="[...navButtonClass, activeIndicatorClass(item.paths?.includes(currentPath))]" text plain
            @click="item.command">
            <span :class="[
              item.icon,
              'text-[1.1rem] transition-transform duration-200 ease-in-out group-hover:scale-105',
            ]" />
            <span>{{ item.label }}</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
