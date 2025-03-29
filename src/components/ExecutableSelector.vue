<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { storeToRefs } from 'pinia'
import RadioButton from 'primevue/radiobutton'
import { computed, ref } from 'vue'
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
const zipPreviewRef = ref<InstanceType<typeof ZipPreview> | null>(null)

// Define the FileNode interface explicitly
interface FileNode {
  path?: string
  name: string
  type: string
}

// File filter function based on selected filter mode
const fileFilter = computed(() => {
  return (node: FileNode) => {
    // Always show directories
    if (node.type === 'directory') return true
    
    // Filter files based on selected mode
    switch (filterMode.value) {
      case 'exe':
        return node.name.toLowerCase().endsWith('.exe')
      case 'executable':
        return (
          node.name.toLowerCase().endsWith('.exe') || 
          node.name.toLowerCase().endsWith('.bat') || 
          node.name.toLowerCase().endsWith('.cmd') || 
          node.name.toLowerCase().endsWith('.ps1') || 
          node.name.toLowerCase().endsWith('.sh') || 
          node.name.toLowerCase().endsWith('.jar')
        )
      case 'all':
      default:
        return true
    }
  }
})

// Handle node selection with proper typing
function handleNodeSelect(node: FileNode) {
  if (node?.path) {
    executable_path.value = node.path
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
          ref="zipPreviewRef"
          :zip-path="zipPath"
          :filter-function="fileFilter"
          :details-loading="detailsLoading"
          @node-select="handleNodeSelect"
        />
      </div>
    </div>
  </div>
</template>