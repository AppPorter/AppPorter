<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import SelectButton from 'primevue/selectbutton'
import SplitButton from 'primevue/splitbutton'
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

// Handle installer mode selection
async function handleInstallerMode() {
  if (isSelecting.value) return
  isSelecting.value = true
  emit('loading', true)

  await invoke('execute_command', {
    command: {
      name: 'RunInstaller',
      path: {
        zip_path: props.zipPath,
        executable_path: selectedPath.value,
        password: props.password,
      },
    },
  })

  isSelecting.value = false
  emit('loading', false)
  store.show_preview_drawer = false
}

// Split button menu items
const menuItems = ref([
  {
    label: t('cls.install.installer_mode'),
    icon: 'mir-settings',
    command: handleInstallerMode
  }
])
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Filter Options -->
    <div class="mb-2">
      <SelectButton v-model="filterMode" :options="FILTER_MODES" optionLabel="label" optionValue="value"
        :allowEmpty="false" size="small" class="w-full" />
    </div>

    <!-- Main content area -->
    <div class="min-h-0 flex-1 overflow-hidden">
      <ZipPreview :zip-path="zipPath" :password="password" :file-tree="fileTree" :filter-function="fileFilter"
        :selected-path="selectedPath" :is-selectable-function="isSelectableFile" @node-click="handleNodeSelect"
        @update-file-tree="emit('update-file-tree', $event)" />
    </div>

    <!-- Bottom status and buttons -->
    <div class="mt-2 flex items-center justify-between gap-4">
      <div class="flex flex-1 items-center gap-2 text-sm">
        <span :class="[
          selectedPath
            ? 'mir-check_circle text-green-500'
            : hasExecutableFiles
              ? 'mir-info text-slate-500'
              : 'mir-warning text-orange-500',
        ]"></span>
        <span class="font-medium">
          {{
            selectedPath
              ? t('ui.executable_selector.selected')
              : hasExecutableFiles
                ? t('ui.executable_selector.select_prompt')
                : t('ui.executable_selector.no_executables_found')
          }}
        </span>
        <span v-if="selectedPath" class="truncate text-slate-600 dark:text-slate-400">
          {{ selectedPath }}
        </span>
      </div>
      <div class="flex gap-2">
        <div v-if="isSelecting" class="flex items-center">
          <div class="size-5 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
        </div>
        <template v-else>
          <Button severity="secondary" @click="handleNoExecutable" :label="t('ui.executable_selector.no_executable')"
            class="h-8 text-sm" icon="mir-rule" outlined />
          <SplitButton v-if="hasExecutableFiles" severity="primary" :disabled="!selectedPath" @click="handleSelect"
            icon="mir-check" class="h-8 text-sm" :label="t('g.select')" :model="menuItems" />
        </template>
      </div>
    </div>
  </div>
</template>
