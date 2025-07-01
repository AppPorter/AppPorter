<script setup lang="ts">
import { window as tauriWindow } from '@/main'
import { settingsStore } from '@/main'
import { exec } from '@/exec'

function handleClose() {
  if (settingsStore.minimize_to_tray_on_close) {
    tauriWindow.hide()
  } else {
    exec('Exit', {
      code: 0,
    })
  }
}

function handleMinimize() {
  tauriWindow.minimize()
}
</script>

<template>
  <div class="fixed right-0 top-0 z-50 flex h-auto">
    <button class="flex h-8 w-12 items-center justify-center hover:bg-[#e9e9e9] dark:hover:bg-[#2d2d2d]"
      @click="handleMinimize">
      <span class="mir-remove" />
    </button>
    <button class="group flex h-8 w-12 items-center justify-center hover:bg-[#c42b1c]" @click="handleClose">
      <span class="mir-close group-hover:text-white" />
    </button>
  </div>
</template>
