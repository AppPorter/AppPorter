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

// Executable file extensions
const EXECUTABLE_EXTENSIONS = ['.exe', '.bat', '.cmd', '.ps1', '.sh', '.jar']

// File filter function based on selected filter mode
const fileFilter = computed(() => {
  return (node: FileNode) => {
    // Always show directories for navigation
    if (node.type === 'directory') return true
    
    // Filter files based on selected mode
    switch (filterMode.value) {
      case 'exe':
        // Show only .exe files
        return node.name.toLowerCase().endsWith('.exe')
      case 'executable':
        // Show all executable file types
        return EXECUTABLE_EXTENSIONS.some(ext => 
          node.name.toLowerCase().endsWith(ext)
        )
      case 'all':
      default:
        // Show all files
        return true
    }
  }
})

// Determine if a file is selectable - only executable files are selectable
const isSelectableFile = (node: FileNode): boolean => {
  if (node.type !== 'file') return false
  
  return EXECUTABLE_EXTENSIONS.some(ext => 
    node.name.toLowerCase().endsWith(ext)
  )
}

// Handle node selection with proper typing
function handleNodeSelect(node: FileNode) {
  // Only allow selecting executable files, not directories
  if (node?.path && isSelectableFile(node)) {
    executable_path.value = node.path
  }
}
</script>

<template>
  <div class="flex h-full flex-col">
    <div class="flex h-full flex-col">
      <!-- Filter options -->
      <div class="mb-4 flex flex-wrap items-center gap-4 px-2">
        <div v-for="option in FILTER_MODES" :key="option.value" class="flex items-center gap-1">
          <RadioButton v-model="filterMode" :value="option.value" :inputId="option.value" />
          <label :for="option.value" class="flex cursor-pointer items-center gap-1 text-sm">
            <span :class="option.icon" />
            {{ option.label }}
          </label>
        </div>
      </div>

      <!-- Main content area with fixed height and proper overflow handling -->
      <div class="min-h-0 flex-1 overflow-hidden">
        <ZipPreview
          ref="zipPreviewRef"
          :zip-path="zipPath"
          :filter-function="fileFilter"
          :is-selectable-function="isSelectableFile"
          :details-loading="detailsLoading"
          @node-select="handleNodeSelect"
        />
      </div>
    </div>
  </div>
</template>