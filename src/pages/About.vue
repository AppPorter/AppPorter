<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <div class="flex size-full flex-col">
    <Panel class="flex size-full flex-col overflow-auto">
      <!-- Header Section -->
      <template #header>
        <div class="flex items-center gap-2">
          <span class="mir-info text-xl"></span>
          <div>
            <h2 class="text-lg font-medium">{{ t('about.title') }}</h2>
          </div>
        </div>
      </template>

      <div class="mx-auto w-full max-w-5xl space-y-6 overflow-y-auto px-4 pb-16">
        <!-- App Info Section -->
        <div class="flex flex-col items-center justify-center">
          <img src="../assets/appporter.svg" class="mb-4 size-32" alt="AppPorter" />
          <h1 class="mb-2 text-3xl font-bold">AppPorter</h1>
          <p class="text-lg">v0.2.0</p>
        </div>

        <!-- Overview Section -->
        <Panel class="w-full">
          <template #header>
            <div class="flex items-center gap-2">
              <span class="mir-description"></span>
              <span>{{ t('about.overview.title') }}</span>
            </div>
          </template>
          <div class="space-y-4">
            <p>{{ t('about.overview.description') }}</p>
            <p>{{ t('about.overview.extension_info') }}</p>
          </div>
        </Panel>
      </div>
    </Panel>
  </div>
</template>
