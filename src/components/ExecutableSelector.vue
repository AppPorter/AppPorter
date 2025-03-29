<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { storeToRefs } from 'pinia'
import RadioButton from 'primevue/radiobutton'
import Button from 'primevue/button'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ZipPreview from './ZipPreview.vue'

const store = useInstallationConfigStore()
const { executable_path } = storeToRefs(store)

const { t } = useI18n()

// Constants
const FILTER_MODES = {
  exe: { 
    value: 'exe', 
    label: t('installation.preview.filter.exe'), 
    icon: 'mir-terminal',
    description: t('installation.preview.filter.exe_desc') 
  },
  executable: {
    value: 'executable',
    label: t('installation.preview.filter.executable'),
    icon: 'mir-code',
    description: t('installation.preview.filter.executable_desc')
  },
  all: { 
    value: 'all', 
    label: t('installation.preview.filter.all'), 
    icon: 'mir-description',
    description: t('installation.preview.filter.all_desc')
  },
}

// Props
defineProps<{
  zipPath: string
  detailsLoading?: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

// State
const filterMode = ref<'exe' | 'executable' | 'all'>('exe')
const zipPreviewRef = ref<InstanceType<typeof ZipPreview> | null>(null)
const selectedPath = ref('')

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
    selectedPath.value = node.path
  }
}

function handleSelect() {
  executable_path.value = selectedPath.value
  emit('close')
}
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Enhanced Filter UI -->
    <div class="mb-4 rounded-lg border border-slate-200 bg-slate-50 p-3 shadow-sm dark:border-zinc-700 dark:bg-zinc-800/50">
      <div class="flex flex-col gap-3">        
        <!-- Filter options with improved layout -->
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-3">
          <div 
            v-for="option in FILTER_MODES" 
            :key="option.value"
            :class="[
              'flex cursor-pointer items-start gap-2 rounded-md border border-slate-200 p-2 transition-all duration-150',
              'hover:bg-white dark:border-zinc-700 dark:hover:bg-zinc-800',
              filterMode === option.value ? 'bg-white ring-2 ring-blue-500 dark:bg-zinc-800 dark:ring-blue-400' : ''
            ]"
            @click="filterMode = option.value as any"
          >
            <RadioButton 
              v-model="filterMode" 
              :value="option.value" 
              :inputId="option.value"
              class="mt-0.5" 
            />
            <div class="flex-1">
              <label :for="option.value" class="flex cursor-pointer items-center gap-1.5 text-sm font-medium">
                <span :class="[option.icon, filterMode === option.value ? 'text-blue-500' : '']" />
                {{ option.label }}
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main content area with fixed height and proper overflow handling -->
    <div class="min-h-0 flex-1 overflow-hidden rounded-lg bg-white shadow-sm dark:bg-zinc-900">
      <ZipPreview
        ref="zipPreviewRef"
        :zip-path="zipPath"
        :filter-function="fileFilter"
        :is-selectable-function="isSelectableFile"
        :details-loading="detailsLoading"
        @node-select="handleNodeSelect"
      />
    </div>
    
    <!-- Selected file and button container -->
    <div class="mt-3 flex items-center justify-between gap-4">
      <div :class="[
        'flex flex-1 items-center gap-2 rounded-md p-2 text-sm transition-colors',
        selectedPath 
          ? 'bg-green-50 dark:bg-green-900/20' 
          : 'bg-slate-50 dark:bg-zinc-800/50'
      ]">
        <span :class="[
          selectedPath ? 'mir-check_circle text-green-500 dark:text-green-400' : 'mir-info text-slate-500 dark:text-slate-400'
        ]"></span>
        <span :class="[
          'font-medium',
          selectedPath 
            ? 'text-green-800 dark:text-green-300'
            : 'text-slate-700 dark:text-slate-300'
        ]">{{ selectedPath ? t('installation.preview.selected') : t('installation.preview.select_prompt') }}:</span>
        <span :class="[
          'truncate',
          selectedPath 
            ? 'text-green-700 dark:text-green-400'
            : 'text-slate-600 dark:text-slate-400'
        ]">{{ selectedPath || t('installation.preview.no_selection') }}</span>
      </div>
      <Button
        severity="primary"
        :disabled="!selectedPath"
        @click="handleSelect"
      >
        {{ t('common.select') }}
      </Button>
    </div>
  </div>
</template>