<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config';
import { ref } from 'vue';
import AppDetails from './components/AppDetails.vue';
import Options from './components/AppOptions.vue';

// Props
defineProps<{
  nameError: boolean
  executablePathError: boolean
  pathError: string
}>()

// Emits
defineEmits<{
  'update:pathError': [value: string]
}>()

const installConfig = InstallConfigStore()

// UI state management
const detailsLoading = ref(false)
const detailsLoadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <!-- Main scrollable container -->
    <div class="flex-1 overflow-auto">
      <!-- Content wrapper -->
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
          <AppDetails :name-error="nameError" :details-loading="detailsLoading"
            :details-loading-progress="detailsLoadingProgress" :progress-mode="progressMode"
            :executable-path-error="executablePathError" />
          <Options :path-error="pathError" @update:path-error="$emit('update:pathError', $event)" />
        </div>
      </div>
    </div>
  </div>
</template>
