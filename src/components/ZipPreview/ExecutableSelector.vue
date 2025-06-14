<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import ProgressSpinner from 'primevue/progressspinner'
import RadioButton from 'primevue/radiobutton'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ZipPreview, { FileTreeNode } from './ZipPreview.vue'

const store = InstallConfigStore()
const { t } = useI18n()

// Props
const props = defineProps<{
  zipPath: string
  password?: string
  fileTree: FileTreeNode[]
  detailsLoading?: boolean
}>()

const emit = defineEmits<{
  (e: 'loading', value: boolean): void
  (e: 'executable-selected'): void
  (e: 'no-executable'): void
  (e: 'update-file-tree', fileTree: FileTreeNode[]): void
}>()

// State
const filterMode = ref<'exe' | 'executable' | 'all'>('exe')
const selectedPath = ref('')
const isSelecting = ref(false)

// File node interface
interface FileNode {
  path?: string
  name: string
  type: string
}

// Executable file extensions
const EXECUTABLE_EXTENSIONS = ['.exe', '.bat', '.cmd', '.ps1', '.sh', '.jar']

// Filter modes
const FILTER_MODES = [
  {
    value: 'exe',
    label: t('ui.executable_selector.filter.exe'),
    icon: 'mir-terminal',
  },
  {
    value: 'executable',
    label: t('ui.executable_selector.filter.executable'),
    icon: 'mir-code',
  },
  {
    value: 'all',
    label: t('ui.executable_selector.filter.all'),
    icon: 'mir-description',
  },
]

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

// Check if file tree contains any executable files
const hasExecutableFiles = computed(() => {
  const checkNode = (node: FileTreeNode): boolean => {
    // Check if current node is an executable file
    if (node.node_type === 'file') {
      return EXECUTABLE_EXTENSIONS.some((ext) => node.name.toLowerCase().endsWith(ext))
    }

    // Recursively check children if it's a directory
    if (node.node_type === 'directory' && node.children) {
      return node.children.some((child) => checkNode(child))
    }

    return false
  }

  return props.fileTree.some((node) => checkNode(node))
})

// Get description for filter options
function getFilterDescription(mode: 'exe' | 'executable' | 'all'): string {
  switch (mode) {
    case 'exe':
      return t('ui.executable_selector.filter_description.exe')
    case 'executable':
      return t('ui.executable_selector.filter_description.executable')
    case 'all':
      return t('ui.executable_selector.filter_description.all')
    default:
      return ''
  }
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
          password: props.password,
        },
      },
    })

    const details = JSON.parse(result as string)
    store.app_details.info.name = details.product_name
    store.app_details.info.version = details.version
    store.app_details.info.publisher = details.copyright
    store.app_details.info.icon = details.icon_data_url
    store.app_details.config.archive_exe_path = selectedPath.value

    emit('executable-selected')
  } finally {
    isSelecting.value = false
    emit('loading', false)
  }
}

function handleNoExecutable() {
  emit('no-executable')
}
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Enhanced Filter UI -->
    <div
      class="mb-4 rounded-lg border border-slate-200 bg-slate-50 p-3 shadow-sm dark:border-zinc-700 dark:bg-zinc-800/50">
      <div class="flex flex-col gap-3">
        <!-- Filter options with horizontal layout and better descriptions -->
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
              <p class="mt-1 text-xs text-slate-500 dark:text-slate-400">
                {{ getFilterDescription(option.value as any) }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main content area with fixed height and proper overflow handling -->
    <div class="min-h-0 flex-1 overflow-hidden rounded-lg bg-white shadow-sm dark:bg-zinc-900">
      <ZipPreview :zip-path="zipPath" :password="password" :file-tree="fileTree" :filter-function="fileFilter"
        :selected-path="selectedPath" :is-selectable-function="isSelectableFile" @node-click="handleNodeSelect"
        @update-file-tree="emit('update-file-tree', $event)" />
    </div>

    <!-- Selected file and button container -->
    <div class="mt-3 flex items-center justify-between gap-4">
      <div :class="[
        'flex flex-1 items-center gap-2 rounded-md p-2 text-sm transition-colors',
        selectedPath
          ? 'bg-green-50 dark:bg-green-900/20'
          : hasExecutableFiles
            ? 'bg-slate-50 dark:bg-zinc-800/50'
            : 'bg-orange-50 dark:bg-orange-900/20',
      ]">
        <span :class="[
          selectedPath
            ? 'mir-check_circle text-green-500 dark:text-green-400'
            : hasExecutableFiles
              ? 'mir-info text-slate-500 dark:text-slate-400'
              : 'mir-warning text-orange-500 dark:text-orange-400',
        ]"></span>
        <span :class="[
          'font-medium',
          selectedPath
            ? 'text-green-800 dark:text-green-300'
            : hasExecutableFiles
              ? 'text-slate-700 dark:text-slate-300'
              : 'text-orange-800 dark:text-orange-300',
        ]">{{
          selectedPath
            ? t('ui.executable_selector.selected')
            : hasExecutableFiles
              ? t('ui.executable_selector.select_prompt')
              : t('ui.executable_selector.no_executables_found')
        }}{{ hasExecutableFiles ? ':' : '' }}</span>
        <span v-if="hasExecutableFiles" :class="[
          'truncate',
          selectedPath
            ? 'text-green-700 dark:text-green-400'
            : 'text-slate-600 dark:text-slate-400',
        ]">{{ selectedPath || t('ui.executable_selector.no_selection') }}</span>
      </div>
      <div class="flex gap-2">
        <ProgressSpinner v-if="isSelecting" style="width: 2rem; height: 2rem" strokeWidth="4" />
        <template v-else>
          <Button severity="secondary" @click="handleNoExecutable">
            {{ t('ui.executable_selector.no_executable') }}
          </Button>
          <Button v-if="hasExecutableFiles" severity="primary" :disabled="!selectedPath" @click="handleSelect">
            {{ t('g.select') }}
          </Button>
        </template>
      </div>
    </div>
  </div>
</template>
