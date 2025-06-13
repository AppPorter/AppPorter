<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

// Types and interfaces
interface FileTreeNode {
  key: string
  name: string
  path: string
  node_type: string
  children?: FileTreeNode[]
  expanded: boolean
  level: number
}

interface FileNode {
  key: string
  name: string
  path: string
  type: 'file' | 'directory'
  children?: FileNode[]
  expanded?: boolean
  level: number
}

// Props & emits
const props = defineProps<{
  zipPath: string
  password?: string
  filterFunction?: (node: FileNode) => boolean
  selectedPath?: string
}>()

const emits = defineEmits<{
  (event: 'node-click', node: FileNode): void
}>()

const { t } = useI18n()

// State
const status = ref<'ready' | 'loading' | 'error'>('ready')
const fileTreeData = ref<FileTreeNode[]>([])
const error = ref<string>('')

// Load zip content from backend
async function loadZipContent() {
  if (!props.zipPath) return

  status.value = 'loading'
  error.value = ''

  try {
    const command = {
      name: 'GetArchiveTree',
      path: props.zipPath,
      password: props.password || null
    }

    const result = await invoke<string>('execute_command', { command })
    const treeData = JSON.parse(result) as FileTreeNode[]
    fileTreeData.value = treeData
    status.value = 'ready'
  } catch (err) {
    console.error('Failed to load zip content:', err)
    error.value = err instanceof Error ? err.message : String(err)
    status.value = 'error'
    fileTreeData.value = []
  }
}

// Convert backend tree structure to frontend format
function convertToFileNode(node: FileTreeNode): FileNode {
  return {
    key: node.key,
    name: node.name,
    path: node.path,
    type: node.node_type === 'directory' ? 'directory' : 'file',
    level: node.level,
    expanded: node.expanded,
    children: node.children?.map(convertToFileNode)
  }
}

// Computed properties
const hasData = computed(() => fileTreeData.value.length > 0)
const isEmpty = computed(() => status.value === 'ready' && fileTreeData.value.length === 0)
const fileTree = computed(() => {
  return fileTreeData.value.map(convertToFileNode)
})

// Get appropriate icon based on file extension
function getFileIcon(fileName: string): string {
  const lowerName = fileName.toLowerCase()

  // Executable files
  if (lowerName.endsWith('.exe')) return 'mir-terminal text-indigo-500'
  if (lowerName.endsWith('.bat') || lowerName.endsWith('.cmd')) return 'mir-terminal text-green-600'
  if (lowerName.endsWith('.ps1')) return 'mir-terminal text-blue-600'
  if (lowerName.endsWith('.sh')) return 'mir-terminal text-amber-600'

  // Archives
  if (['.zip', '.rar', '.7z', '.tar', '.gz'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-archive text-amber-600'

  // Documents
  if (['.pdf', '.doc', '.docx', '.txt', '.rtf'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-description text-blue-500'

  // Images
  if (['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-image text-green-500'

  // Code and config files
  if (['.xml', '.json', '.ini', '.config', '.yml', '.yaml'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-code text-slate-500'

  // Default
  return 'mir-draft text-slate-500 dark:text-slate-400'
}

// Toggle node expansion
function handleToggleNode(node: FileNode) {
  node.expanded = !node.expanded
}

// Handle node selection
function handleSelectNode(node: FileNode) {
  // For directories, toggle expansion
  if (node.type === 'directory') {
    handleToggleNode(node)
    return
  }

  // For files, just emit click event
  emits('node-click', node)
}

// Recursive function to render all nodes as flat list with proper indentation
function flattenTree(nodes: FileNode[], level = 0): FileNode[] {
  const result: FileNode[] = []

  for (const node of nodes) {
    result.push({ ...node, level })

    if (node.type === 'directory' && node.expanded && node.children) {
      result.push(...flattenTree(node.children, level + 1))
    }
  }

  return result
}

// Get flattened tree for rendering
const flattenedTree = computed(() => {
  return flattenTree(fileTree.value)
})

// Watch for zipPath changes
watch(() => props.zipPath, () => {
  if (props.zipPath) {
    loadZipContent()
  }
}, { immediate: true })

// Watch for password changes
watch(() => props.password, () => {
  if (props.zipPath) {
    loadZipContent()
  }
})

// Load content on mount
onMounted(() => {
  if (props.zipPath) {
    loadZipContent()
  }
})
</script>

<template>
  <div class="flex h-full min-h-0 flex-col shadow-sm">
    <!-- File Tree Container with improved scrolling -->
    <div class="relative min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 dark:border-zinc-600">
      <!-- Scrollable Tree Content -->
      <div v-if="hasData && !isEmpty" class="h-full overflow-auto p-2 text-sm">
        <!-- Render flattened tree with proper indentation -->
        <template v-for="node in flattenedTree" :key="node.key">
          <div v-if="!props.filterFunction || props.filterFunction(node)" class="w-full">
            <div :class="[
              'my-0.5 flex cursor-pointer items-center rounded-md border px-1 py-1.5 transition-colors duration-150 hover:bg-slate-100 dark:hover:bg-zinc-700',
              props.selectedPath === node.path ? 'border-blue-200 bg-blue-50 dark:border-blue-700 dark:bg-blue-900/20' : 'border-transparent',
            ]" :style="{ paddingLeft: `${node.level * 16 + 4}px` }" @click="handleSelectNode(node)">
              <!-- Expand/collapse icon for directories -->
              <span v-if="node.type === 'directory' && node.children && node.children.length > 0" :class="[
                'mr-1 w-4 shrink-0 text-xs transition-transform duration-200',
                node.expanded ? 'mir-expand_more' : 'mir-chevron_right'
              ]"></span>
              <span v-else class="mr-1 w-4 shrink-0"></span>

              <!-- File/folder icon -->
              <span :class="[
                'mr-2 shrink-0',
                node.type === 'directory'
                  ? node.expanded
                    ? 'mir-folder_open text-amber-500'
                    : 'mir-folder text-amber-500'
                  : getFileIcon(node.name),
              ]"></span>

              <!-- Node name -->
              <span class="flex-1 truncate">{{ node.name }}</span>
            </div>
          </div>
        </template>
      </div>

      <!-- Enhanced loading indicator -->
      <div v-if="status === 'loading'"
        class="absolute inset-0 flex items-center justify-center bg-white/90 backdrop-blur-sm transition-all duration-300 dark:bg-zinc-900/90">
        <div
          class="flex flex-col items-center gap-3 rounded-lg bg-white/60 p-6 shadow-sm backdrop-blur-md dark:bg-zinc-800/60">
          <span class="mir-progress_activity animate-spin text-3xl text-blue-500"></span>
          <p class="text-sm font-medium text-slate-700 dark:text-slate-200">
            {{ t('g.loading') }}
          </p>
        </div>
      </div>

      <!-- Enhanced empty state -->
      <div v-if="hasData && isEmpty"
        class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-white/60 backdrop-blur-sm dark:bg-zinc-900/60">
        <div class="flex flex-col items-center gap-3 rounded-lg bg-white p-6 shadow-sm dark:bg-zinc-800">
          <span class="mir-folder_off text-4xl text-slate-400 dark:text-slate-500"></span>
          <p class="text-sm font-medium text-slate-700 dark:text-slate-300">
            {{ t('ui.zip_preview.no_files') }}
          </p>
        </div>
      </div>

      <!-- Error state -->
      <div v-if="status === 'error'"
        class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-white/60 backdrop-blur-sm dark:bg-zinc-900/60">
        <div class="flex flex-col items-center gap-3 rounded-lg bg-white p-6 shadow-sm dark:bg-zinc-800">
          <span class="mir-error text-4xl text-red-500"></span>
          <p class="text-sm font-medium text-slate-700 dark:text-slate-300">
            {{ error || t('g.error') }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>