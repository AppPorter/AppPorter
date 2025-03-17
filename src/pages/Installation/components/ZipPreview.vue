<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import Panel from 'primevue/panel'
import ProgressBar from 'primevue/progressbar'
import RadioButton from 'primevue/radiobutton'
import Tree from 'primevue/tree'
import type { TreeNode } from 'primevue/treenode'
import { computed, nextTick, onMounted, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'

const store = useInstallationConfigStore()
const { executable_path } = storeToRefs(store)

// Types and interfaces
type FilterMode = 'exe' | 'executable' | 'all'
type FileStatus = 'ready' | 'loading' | 'error'

interface CustomNodeData {
  path: string
  isExecutable?: boolean
}

interface CustomTreeNode extends TreeNode {
  key: string
  label: string
  icon?: string
  children?: CustomTreeNode[]
  selectable?: boolean
  data?: CustomNodeData
  containsExecutable?: boolean
}

const { t } = useI18n()

// Constants
const FILTER_MODES = {
  exe: { value: 'exe', label: t('installation.preview.filter.exe'), icon: 'mir terminal' },
  executable: {
    value: 'executable',
    label: t('installation.preview.filter.executable'),
    icon: 'mir code',
  },
  all: { value: 'all', label: t('installation.preview.filter.all'), icon: 'mir description' },
}

// Props & emits
const props = defineProps<{
  zipPath: string
  detailsLoading?: boolean
}>()

const emit = defineEmits<{
  (event: 'loading', value: boolean): void
  (event: 'progress', value: number): void
}>()

// State
const status = ref<FileStatus>('ready')
const filterMode = ref<FilterMode>('exe')
const expandedKeys = ref<Record<string, boolean>>({})
const selectedNode = ref<Record<string, boolean>>({})
const fileTree = ref<CustomTreeNode[]>([])
const fileCache = ref<string[] | null>(null)
const loadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const isExpanding = ref(false)
const isCollapsing = ref(false)

// Computed properties
const filteredPaths = computed(() => {
  if (!fileCache.value) return []
  if (filterMode.value === 'all') return fileCache.value

  return fileCache.value.filter((path) => {
    if (path.endsWith('\\')) return true // Always include directories

    const isExe = path.toLowerCase().endsWith('.exe')
    if (filterMode.value === 'exe') return isExe
    return isExe || /\.(bat|ps1)$/i.test(path) // Executable filter includes .exe, .bat and .ps1
  })
})

const hasScanned = computed(() => fileCache.value !== null)
const isEmpty = computed(() => hasScanned.value && fileTree.value.length === 0)

// Utility functions
function getFileIcon(fileName: string): string {
  const lower = fileName.toLowerCase()
  if (lower.endsWith('.exe')) return 'mir terminal'
  if (/\.(ps1|bat)$/i.test(lower)) return 'mir code'
  return 'mir draft'
}

// Format and build tree structure from flat file paths
function buildFileTree(paths: string[]): CustomTreeNode[] {
  // Build directory map for quick lookup
  const dirMap = new Map<string, boolean>()
  paths.forEach((path) => {
    const parts = path.split('\\')
    let currentPath = ''
    for (let i = 0; i < parts.length - 1; i++) {
      if (!parts[i]) continue
      currentPath = currentPath ? `${currentPath}\\${parts[i]}` : parts[i]
      dirMap.set(currentPath, true)
    }
  })

  // Identify paths that are executable files
  const executablePaths = new Set<string>()
  paths.forEach((path) => {
    if (!path.endsWith('\\') && /\.(exe|bat|ps1)$/i.test(path.toLowerCase())) {
      executablePaths.add(path)
    }
  })

  // Build tree
  const root: CustomTreeNode[] = []

  paths.forEach((path) => {
    if (!path.trim()) return

    const parts = path.split('\\').filter((p) => p !== '')
    if (parts.length === 0) return

    let current = root
    let currentPath = ''

    parts.forEach((part, index) => {
      currentPath = currentPath ? `${currentPath}\\${part}` : part
      const isLast = index === parts.length - 1
      const isFile = isLast && !dirMap.has(currentPath)
      const isExecutable = isFile && /\.(exe|bat|ps1)$/i.test(part.toLowerCase())

      // Find or create node
      let node = current.find((n) => n.label === part)

      if (!node) {
        node = {
          key: currentPath,
          label: part,
          selectable: true,
          icon: isFile ? getFileIcon(part) : 'folder',
          children: isFile ? undefined : [],
          data: {
            path: currentPath,
            isExecutable: isFile && isExecutable,
          },
          containsExecutable: isExecutable,
        }

        current.push(node)
      }

      if (node.children) {
        current = node.children
      }
    })
  })

  // Check if directories contain executables and mark them
  function markExecutableContainers(nodes: CustomTreeNode[]): boolean {
    let containsExecutable = false

    for (const node of nodes) {
      if (node.children?.length) {
        // Process children first
        node.containsExecutable = markExecutableContainers(node.children)
        containsExecutable = containsExecutable || node.containsExecutable
      } else if (node.data?.isExecutable) {
        // This is an executable file
        containsExecutable = true
      }
    }

    return containsExecutable
  }

  // Filter out directories without executables based on filter mode
  function filterEmptyDirectories(nodes: CustomTreeNode[]): CustomTreeNode[] {
    if (filterMode.value === 'all') {
      return nodes
    }

    return nodes.filter((node) => {
      if (node.children?.length) {
        // For directories, first filter their children
        node.children = filterEmptyDirectories(node.children)
        // Then check if there are any children left after filtering
        return node.children.length > 0
      }

      // For files, keep executables based on filter mode
      if (filterMode.value === 'exe') {
        return node.data?.path.toLowerCase().endsWith('.exe')
      } else {
        // 'executable'
        return node.data?.isExecutable
      }
    })
  }

  // Mark directories that contain executables
  markExecutableContainers(root)

  // Filter out directories without executables based on filter mode
  const filteredRoot = filterEmptyDirectories(root)

  // Sort nodes (directories first, then executables, then alphabetical)
  const sortNodes = (nodes: CustomTreeNode[]): void => {
    nodes.sort((a, b) => {
      const aIsDir = !!a.children
      const bIsDir = !!b.children

      // Directories first
      if (aIsDir !== bIsDir) return aIsDir ? -1 : 1

      // For files: executable priority
      if (!aIsDir && !bIsDir) {
        const aIsExe = a.label.toLowerCase().endsWith('.exe')
        const bIsExe = b.label.toLowerCase().endsWith('.exe')
        if (aIsExe !== bIsExe) return aIsExe ? -1 : 1

        const aIsExecutable = /\.(bat|ps1)$/i.test(a.label.toLowerCase())
        const bIsExecutable = /\.(bat|ps1)$/i.test(b.label.toLowerCase())
        if (aIsExecutable !== bIsExecutable) return aIsExecutable ? -1 : 1
      }

      // Alphabetical by default
      return a.label.localeCompare(b.label)
    })

    // Sort children recursively
    nodes.forEach((node) => {
      if (node.children?.length) sortNodes(node.children)
    })
  }

  sortNodes(filteredRoot)

  // Auto-expand if single top folder
  if (filteredRoot.length === 1 && filteredRoot[0].children) {
    expandedKeys.value[filteredRoot[0].key] = true
  }

  return filteredRoot
}

// Handle tree node selection (folders and files)
function handleNodeSelect(node: TreeNode) {
  const customNode = node as CustomTreeNode

  // Toggle folder expansion
  if (customNode.children) {
    expandedKeys.value[customNode.key] = !expandedKeys.value[customNode.key]
    expandedKeys.value = { ...expandedKeys.value }
    nextTick(() => (selectedNode.value = {}))
    return
  }

  // Select executable file or clear selection
  if (customNode.data?.isExecutable) {
    executable_path.value = customNode.data.path
    selectedNode.value = { [customNode.key]: true }
  } else {
    selectedNode.value = {}
  }
}

// Try to automatically select an executable if there's only one at root level
function tryAutoSelectExecutable() {
  if (!fileCache.value) return

  // Auto-select top-level executable if only one exists
  const topLevelExes = fileCache.value.filter(
    (path) => path.toLowerCase().endsWith('.exe') && path.split('\\').length <= 2
  )

  if (topLevelExes.length === 1) {
    executable_path.value = topLevelExes[0]
    selectedNode.value = { [topLevelExes[0]]: true }
  }
}

// Load and process archive contents
async function loadZipContent() {
  if (!props.zipPath) return

  status.value = 'loading'
  emit('loading', true)
  progressMode.value = 'indeterminate'

  try {
    // Get archive content from backend
    const result = await invoke('execute_command', {
      command: {
        name: 'GetArchiveContent',
        path: props.zipPath,
      },
    })

    const parsedResult = JSON.parse(result as string)
    if ('error' in parsedResult) throw new Error(parsedResult.error)

    // Process file paths
    const paths = Array.isArray(parsedResult) ? parsedResult : []
    const allPaths = [...paths]
    const directories = new Set<string>()

    // Generate directory paths
    paths.forEach((path) => {
      const parts = path.split('\\')
      let currentDir = ''
      for (let i = 0; i < parts.length - 1; i++) {
        if (!parts[i]) continue
        currentDir = currentDir ? `${currentDir}\\${parts[i]}` : parts[i]
        directories.add(`${currentDir}\\`)
      }
    })

    directories.forEach((dir) => {
      if (!allPaths.includes(dir)) allPaths.push(dir)
    })

    // Update state
    fileCache.value = allPaths
    loadingProgress.value = 100
    fileTree.value = buildFileTree(filteredPaths.value)
    tryAutoSelectExecutable()
  } catch (error) {
    console.error('Failed to read zip:', error)
    fileTree.value = []
    status.value = 'error'
  } finally {
    status.value = 'ready'
    emit('loading', false)
    loadingProgress.value = 0
  }
}

// Expand all nodes in the tree
const expandAll = () => {
  if (isExpanding.value) return
  isExpanding.value = true

  try {
    const expandNode = (node: CustomTreeNode) => {
      if (node.children?.length) {
        expandedKeys.value[node.key] = true
        node.children.forEach(expandNode)
      }
    }

    fileTree.value.forEach(expandNode)
    expandedKeys.value = { ...expandedKeys.value }
  } finally {
    isExpanding.value = false
  }
}

// Collapse all nodes in the tree
const collapseAll = () => {
  if (isCollapsing.value) return
  isCollapsing.value = true
  try {
    expandedKeys.value = {}
  } finally {
    isCollapsing.value = false
  }
}

// Effects
watchEffect(() => {
  if (!props.zipPath) return
  fileCache.value = null
  selectedNode.value = {}
  executable_path.value = ''
  loadZipContent()
})

watchEffect(() => {
  if (fileCache.value) {
    fileTree.value = buildFileTree(filteredPaths.value)
  }
})

// Setup event listeners and load initial content
onMounted(() => {
  if (props.zipPath) loadZipContent()
})
</script>

<template>
  <Panel class="relative flex h-full flex-col overflow-auto rounded-md border shadow-sm">
    <!-- Header -->
    <template #header>
      <div class="flex w-full items-center justify-between">
        <div class="flex items-center gap-1">
          <span class="mir folder_zip text-lg"></span>
          <span class="text-base font-medium">{{ t('installation.preview.title') }}</span>
        </div>
        <div class="ml-2 flex gap-1">
          <Button
            type="button"
            class="h-6 p-1"
            severity="secondary"
            :disabled="isExpanding"
            v-tooltip.bottom="t('installation.preview.expand_all')"
            @click="expandAll"
            :icon="isExpanding ? 'mir progress_activity' : 'mir unfold_more'"
          />
          <Button
            type="button"
            class="h-6 p-1"
            severity="secondary"
            :disabled="isCollapsing"
            v-tooltip.bottom="t('installation.preview.collapse_all')"
            @click="collapseAll"
            :icon="isCollapsing ? 'mir progress_activity' : 'mir unfold_less'"
          />
        </div>
      </div>
    </template>

    <div class="flex h-full flex-col p-1">
      <!-- Main content area - flex-1 to take all available space -->
      <div class="flex flex-1 flex-col overflow-hidden">
        <!-- Filter Controls -->
        <div class="mb-2 shrink-0">
          <div class="space-y-1.5 rounded-md p-2">
            <div
              v-for="mode in Object.values(FILTER_MODES)"
              :key="mode.value"
              class="flex items-center gap-2"
            >
              <RadioButton
                v-model="filterMode"
                :value="mode.value"
                :inputId="'filter-' + mode.value"
              />
              <label :for="'filter-' + mode.value" class="flex cursor-pointer items-center gap-2.5">
                <span class="mir" :class="mode.icon"></span>
                <span class="text-sm">{{ mode.label }}</span>
              </label>
            </div>
          </div>
        </div>

        <!-- File Tree - flex-1 and overflow-auto to allow scrolling -->
        <div
          class="relative min-h-0 flex-1 overflow-auto rounded-lg border border-slate-200 dark:border-zinc-600"
        >
          <Tree
            v-if="hasScanned && !isEmpty"
            :value="fileTree"
            v-model:selectionKeys="selectedNode"
            v-model:expandedKeys="expandedKeys"
            class="size-full"
            selectionMode="single"
            toggleOnClick
            @node-select="handleNodeSelect"
          />

          <!-- Empty State -->
          <div
            v-if="hasScanned && isEmpty"
            class="absolute inset-0 flex flex-col items-center justify-center gap-2 backdrop-blur-[0.125rem]"
          >
            <span class="mir folder_off text-4xl"></span>
            <p class="text-sm">
              {{ t('installation.preview.no_files') }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading Overlay -->
    <div
      v-if="props.detailsLoading"
      class="absolute inset-0 flex flex-col items-center justify-center gap-2 backdrop-blur-[0.125rem]"
    >
      <h3 class="text-base font-semibold">
        {{ t('installation.preview.reading_archive') }}
      </h3>
      <ProgressBar :mode="progressMode" :value="loadingProgress" class="w-40" />
      <p class="text-sm">
        {{
          loadingProgress === 100
            ? t('installation.preview.completed')
            : t('installation.preview.loading')
        }}
      </p>
    </div>
  </Panel>
</template>
