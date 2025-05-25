<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import ProgressSpinner from 'primevue/progressspinner'
import RadioButton from 'primevue/radiobutton'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ZipPreview from './ZipPreview.vue'

const store = InstallConfigStore()

const { t } = useI18n()

// Constants
const FILTER_MODES = {
  exe: {
    value: 'exe',
    label: t('executable_selector.filter.exe'),
    icon: 'mir-terminal',
  },
  executable: {
    value: 'executable',
    label: t('executable_selector.filter.executable'),
    icon: 'mir-code',
  },
  all: {
    value: 'all',
    label: t('executable_selector.filter.all'),
    icon: 'mir-description',
  },
}

// Props
const props = defineProps<{
  zipPath: string
  detailsLoading?: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'loading', value: boolean): void
}>()

// State
const filterMode = ref<'exe' | 'executable' | 'all'>('exe')
const zipPreviewRef = ref<InstanceType<typeof ZipPreview> | null>(null)
const selectedPath = ref('')
const isSelecting = ref(false)

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
        return EXECUTABLE_EXTENSIONS.some((ext) => node.name.toLowerCase().endsWith(ext))
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

  return EXECUTABLE_EXTENSIONS.some((ext) => node.name.toLowerCase().endsWith(ext))
}

// Handle node selection with proper typing
function handleNodeSelect(node: FileNode) {
  // Only allow selecting executable files, not directories
  if (node?.path && isSelectableFile(node)) {
    selectedPath.value = node.path
  }
}

async function handleSelect() {
  if (isSelecting.value) return
  try {
    isSelecting.value = true
    emit('loading', true)
    const result = await invoke('execute_command', {
      command: {
        name: 'GetDetails',
        path: {
          zip_path: props.zipPath,
          executable_path: selectedPath.value,
          password: store.archive_password,
        },
      },
    })

    const details = JSON.parse(result as string)
    store.details.info.name = details.product_name
    store.details.info.version = details.version
    store.details.info.publisher = details.copyright
    store.details.info.icon = details.icon_data_url
    store.details.config.archive_exe_path = selectedPath.value

    emit('close')
  } finally {
    isSelecting.value = false
    emit('loading', false)
  }
}
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Enhanced Filter UI -->
    <div
      class="mb-4 rounded-lg border border-slate-200 bg-slate-50 p-3 shadow-sm dark:border-zinc-700 dark:bg-zinc-800/50">
      <div class="flex flex-col gap-3">
        <!-- Filter options with horizontal layout -->
        <div class="flex gap-2">
          <div v-for="option in FILTER_MODES" :key="option.value" :class="[
            'flex flex-1 cursor-pointer items-start gap-2 rounded-md border border-slate-200 p-2 transition-all duration-150',
            'hover:bg-white dark:border-zinc-700 dark:hover:bg-zinc-800',
            filterMode === option.value
              ? 'bg-white ring-2 ring-blue-500 dark:bg-zinc-800 dark:ring-blue-400'
              : '',
          ]" @click="filterMode = option.value as any">
            <RadioButton v-model="filterMode" :value="option.value" :inputId="option.value" class="mt-0.5" />
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
      <ZipPreview ref="zipPreviewRef" :zip-path="zipPath" :filter-function="fileFilter"
        :is-selectable-function="isSelectableFile" :details-loading="detailsLoading" @node-select="handleNodeSelect" />
    </div>

    <!-- Selected file and button container -->
    <div class="mt-3 flex items-center justify-between gap-4">
      <div :class="[
        'flex flex-1 items-center gap-2 rounded-md p-2 text-sm transition-colors',
        selectedPath ? 'bg-green-50 dark:bg-green-900/20' : 'bg-slate-50 dark:bg-zinc-800/50',
      ]">
        <span :class="[
          selectedPath
            ? 'mir-check_circle text-green-500 dark:text-green-400'
            : 'mir-info text-slate-500 dark:text-slate-400',
        ]"></span>
        <span :class="[
          'font-medium',
          selectedPath
            ? 'text-green-800 dark:text-green-300'
            : 'text-slate-700 dark:text-slate-300',
        ]">{{
          selectedPath
            ? t('executable_selector.selected')
            : t('executable_selector.select_prompt')
        }}:</span>
        <span :class="[
          'truncate',
          selectedPath
            ? 'text-green-700 dark:text-green-400'
            : 'text-slate-600 dark:text-slate-400',
        ]">{{ selectedPath || t('executable_selector.no_selection') }}</span>
      </div>
      <ProgressSpinner v-if="isSelecting" style="width: 2rem; height: 2rem" strokeWidth="4" />
      <Button v-else severity="primary" :disabled="!selectedPath" @click="handleSelect">
        {{ t('select') }}
      </Button>
    </div>
  </div>
</template>
