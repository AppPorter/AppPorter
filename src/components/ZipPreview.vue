<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import Tree from 'primevue/tree'
import type { TreeNode } from 'primevue/treenode'
import { computed, onMounted, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'

// Types and interfaces
type FileStatus = 'ready' | 'loading' | 'error'

interface CustomNodeData {
  path: string
}

interface CustomTreeNode extends TreeNode {
  key: string
  label: string
  icon?: string
  children?: CustomTreeNode[]
  data?: CustomNodeData
}

// Props & emits
const props = defineProps<{
  zipPath: string
  detailsLoading?: boolean
}>()

const emit = defineEmits<{
  (event: 'loading', value: boolean): void
  (event: 'progress', value: number): void
  (event: 'node-select', node: TreeNode): void
}>()

const { t } = useI18n()

// State
const status = ref<FileStatus>('ready')
const expandedKeys = ref<Record<string, boolean>>({})
const fileTree = ref<CustomTreeNode[]>([])
const fileCache = ref<string[] | null>(null)
const isExpanding = ref(false)
const isCollapsing = ref(false)

// Computed properties
const hasScanned = computed(() => fileCache.value !== null)
const isEmpty = computed(() => hasScanned.value && fileTree.value.length === 0)

// Utility functions
function getFileIcon(): string {
  return 'mir-draft'
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

      // Find or create node
      let node = current.find((n) => n.label === part)

      if (!node) {
        node = {
          key: currentPath,
          label: part,
          icon: isFile ? getFileIcon() : 'folder',
          children: isFile ? undefined : [],
          data: {
            path: currentPath,
          },
        }

        current.push(node)
      }

      if (node.children) {
        current = node.children
      }
    })
  })

  // Sort nodes (directories first, then alphabetically)
  const sortNodes = (nodes: CustomTreeNode[]): void => {
    nodes.sort((a, b) => {
      const aIsDir = !!a.children
      const bIsDir = !!b.children

      // Directories first
      if (aIsDir !== bIsDir) return aIsDir ? -1 : 1

      // Alphabetical by default
      return a.label.localeCompare(b.label)
    })

    // Sort children recursively
    nodes.forEach((node) => {
      if (node.children?.length) sortNodes(node.children)
    })
  }

  sortNodes(root)

  // Auto-expand if single top folder
  if (root.length === 1 && root[0].children) {
    expandedKeys.value[root[0].key] = true
  }

  return root
}

// Load and process archive contents
async function loadZipContent() {
  if (!props.zipPath) return

  status.value = 'loading'
  emit('loading', true)

  // Get archive content from backend
  const result = await invoke('execute_command', {
    command: {
      name: 'GetArchiveContent',
      path: props.zipPath,
    },
  })

  const paths = JSON.parse(result as string)
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
  fileTree.value = buildFileTree(fileCache.value)
  status.value = 'ready'
  emit('loading', false)
}

// Expand all nodes in the tree
const expandAll = () => {
  if (isExpanding.value) return
  isExpanding.value = true

  const expandNode = (node: CustomTreeNode) => {
    if (node.children?.length) {
      expandedKeys.value[node.key] = true
      node.children.forEach(expandNode)
    }
  }

  fileTree.value.forEach(expandNode)
  expandedKeys.value = { ...expandedKeys.value }
  isExpanding.value = false
}

// Collapse all nodes in the tree
const collapseAll = () => {
  if (isCollapsing.value) return
  isCollapsing.value = true
  expandedKeys.value = {}
  isCollapsing.value = false
}

// Effects
watchEffect(() => {
  if (!props.zipPath) return
  fileCache.value = null
  loadZipContent()
})

// Setup event listeners and load initial content
onMounted(() => {
  if (props.zipPath) loadZipContent()
})
</script>

<template>
  <div class="relative flex h-full flex-col overflow-auto">
    <div class="mb-2 flex gap-1 px-1">
      <Button
        type="button"
        class="h-6 p-1"
        severity="secondary"
        :disabled="isExpanding"
        v-tooltip.bottom="t('installation.preview.expand_all')"
        @click="expandAll"
        :icon="isExpanding ? 'mir-progress_activity' : 'mir-unfold_more'"
      />
      <Button
        type="button"
        class="h-6 p-1"
        severity="secondary"
        :disabled="isCollapsing"
        v-tooltip.bottom="t('installation.preview.collapse_all')"
        @click="collapseAll"
        :icon="isCollapsing ? 'mir-progress_activity' : 'mir-unfold_less'"
      />
    </div>

    <!-- File Tree -->
    <div
      class="relative min-h-0 flex-1 overflow-auto rounded-lg border border-slate-200 shadow-sm dark:border-zinc-600"
    >
      <Tree
        v-if="hasScanned && !isEmpty"
        :value="fileTree"
        v-model:expandedKeys="expandedKeys"
        class="size-full"
        toggleOnClick
        @node-select="node => emit('node-select', node)"
      />

      <!-- Empty State -->
      <div
        v-if="hasScanned && isEmpty"
        class="absolute inset-0 flex flex-col items-center justify-center gap-2 backdrop-blur-[0.125rem]"
      >
        <span class="mir-folder_off text-4xl"></span>
        <p class="text-sm">
          {{ t('installation.preview.no_files') }}
        </p>
      </div>
    </div>
  </div>
</template>
