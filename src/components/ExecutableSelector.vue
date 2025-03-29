<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { storeToRefs } from 'pinia'
import RadioButton from 'primevue/radiobutton'
import type { TreeNode } from 'primevue/treenode'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ZipPreview from './ZipPreview.vue'

const store = useInstallationConfigStore()
const { executable_path } = storeToRefs(store)

const { t } = useI18n()

// Constants
const FILTER_MODES = {
  exe: { value: 'exe', label: t('installation.preview.filter.exe'), icon: 'mir-terminal' },
  executable: {
    value: 'executable',
    label: t('installation.preview.filter.executable'),
    icon: 'mir-code',
  },
  all: { value: 'all', label: t('installation.preview.filter.all'), icon: 'mir-description' },
}

// Props
defineProps<{
  zipPath: string
  detailsLoading?: boolean
}>()

// State
const filterMode = ref<'exe' | 'executable' | 'all'>('exe')

// Handle node selection
function handleNodeSelect(node: TreeNode) {
  if (node.data?.path) {
    executable_path.value = node.data.path
  }
}
</script>

<template>
  <div class="relative flex h-full flex-col overflow-auto">
    <div class="flex h-full flex-col">
      <!-- Filter options -->
      <div class="mb-2 flex flex-wrap items-center gap-4 px-2">
        <div v-for="option in FILTER_MODES" :key="option.value" class="flex items-center gap-1">
          <RadioButton v-model="filterMode" :value="option.value" :inputId="option.value" />
          <label :for="option.value" class="flex cursor-pointer items-center gap-1 text-sm">
            <span :class="option.icon" />
            {{ option.label }}
          </label>
        </div>
      </div>

      <!-- Main content area -->
      <div class="flex flex-1 flex-col overflow-hidden">
        <ZipPreview
          :zip-path="zipPath"
          :details-loading="detailsLoading"
          @node-select="handleNodeSelect"
        />
      </div>
    </div>
  </div>
</template>