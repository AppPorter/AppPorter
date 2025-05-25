<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config'
import { storeToRefs } from 'pinia'
import { computed, onMounted, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'

// Types and interfaces
type FileStatus = 'ready' | 'loading' | 'error'
type NodeType = 'file' | 'directory'

interface FileNode {
  key: string
  name: string
  path: string
  type: NodeType
  children?: FileNode[]
  expanded?: boolean
  selected?: boolean
  level: number
}

// Props & emits
const props = defineProps<{
  zipPath: string
  filterFunction?: (node: FileNode) => boolean
  isSelectableFunction?: (node: FileNode) => boolean
}>()

const emits = defineEmits<{
  (event: 'loading', value: boolean): void
  (event: 'progress', value: number): void
  (event: 'node-select', node: FileNode): void
}>()

const { t } = useI18n()

const installConfig = InstallConfigStore()
const { archive_content } = storeToRefs(installConfig)

// State
const status = ref<FileStatus>('ready')
const fileTree = ref<FileNode[]>([])
const fileCache = computed(() => archive_content.value)

// Computed properties
const hasScanned = computed(() => fileCache.value !== null)
const isEmpty = computed(() => hasScanned.value && fileTree.value.length === 0)

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

// Format and build tree structure from flat file paths
function buildFileTree(paths: string[]): FileNode[] {
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
  const root: FileNode[] = []

  paths.forEach((path) => {
    if (!path.trim()) return

    const parts = path.split('\\').filter((p) => p !== '')
    if (parts.length === 0) return

    let current = root
    let currentPath = ''
    let level = 0

    parts.forEach((part, index) => {
      currentPath = currentPath ? `${currentPath}\\${part}` : part
      const isLast = index === parts.length - 1
      const isFile = isLast && !dirMap.has(currentPath)

      // Find or create node
      let node = current.find((n) => n.name === part)

      if (!node) {
        node = {
          key: currentPath,
          name: part,
          path: currentPath,
          type: isFile ? 'file' : 'directory',
          level,
          children: isFile ? undefined : [],
          expanded: false,
          selected: false,
        }

        current.push(node)
      }

      if (node.children) {
        current = node.children
        level++
      }
    })
  })

  // Sort nodes (directories first, then alphabetically)
  const sortNodes = (nodes: FileNode[]): void => {
    nodes.sort((a, b) => {
      // Directories first
      if (a.type !== b.type) return a.type === 'directory' ? -1 : 1

      // Alphabetical by name
      return a.name.localeCompare(b.name)
    })

    // Sort children recursively
    nodes.forEach((node) => {
      if (node.children?.length) sortNodes(node.children)
    })
  }

  sortNodes(root)

  // Auto-expand if single top folder
  if (root.length === 1 && root[0].children) {
    root[0].expanded = true
  }

  return root
}

// Load and process archive contents
async function loadZipContent() {
  if (!props.zipPath || !fileCache.value) return

  status.value = 'loading'
  emits('loading', true)

  const allPaths = [...fileCache.value]
  const directories = new Set<string>()

  // Generate directory paths
  fileCache.value.forEach((path) => {
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
  fileTree.value = buildFileTree(allPaths)
  status.value = 'ready'
  emits('loading', false)
}

// Toggle node expansion
function handleToggleNode(node: FileNode) {
  node.expanded = !node.expanded
}

// Handle node selection
function handleSelectNode(node: FileNode) {
  // For directories, toggle expansion and possibly select it
  if (node.type === 'directory') {
    // Check if the directory is selectable (using the prop function if provided)
    const isSelectable = props.isSelectableFunction ? props.isSelectableFunction(node) : false

    // Toggle node expansion
    handleToggleNode(node)

    // If not selectable, return after toggling expansion
    if (!isSelectable) {
      return
    }
    // Otherwise, continue with selection logic for selectable directories
  }

  // For files and selectable directories, proceed with selection logic
  // Deselect all nodes first
  const deselectAll = (nodes: FileNode[]) => {
    nodes.forEach((node) => {
      node.selected = false
      if (node.children) deselectAll(node.children)
    })
  }

  deselectAll(fileTree.value)

  // Select the clicked node
  node.selected = true

  // Emit selection event with node info
  emits('node-select', node)
}

// Effects
watchEffect(() => {
  if (!props.zipPath || !fileCache.value) return
  loadZipContent()
})

// Setup event listeners and load initial content
onMounted(() => {
  if (props.zipPath && fileCache.value) loadZipContent()
})

// Expose component interface
defineExpose({
  // Exporting types as string literals for parent components to reference
  nodeTypes: ['file', 'directory'] as const,
  // Provide methods that parent can use
  selectNode: handleSelectNode,
  toggleNode: handleToggleNode,
})
</script>

<template>
  <div class="flex h-full min-h-0 flex-col shadow-sm">
    <!-- File Tree Container with improved scrolling -->
    <div class="relative min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 dark:border-zinc-600">
      <!-- Scrollable Tree Content -->
      <div v-if="hasScanned && !isEmpty" class="h-full overflow-auto p-2 text-sm">
        <!-- Recursive Tree Node Template -->
        <template v-for="node in fileTree" :key="node.key">
          <div v-if="!props.filterFunction || props.filterFunction(node)" class="w-full">
            <div :class="[
              'my-0.5 flex items-center rounded-md px-1 py-1.5 transition-colors duration-150 hover:bg-slate-100 dark:hover:bg-zinc-700',
              node.selected
                ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300'
                : '',
              node.level === 0
                ? 'pl-0'
                : node.level === 1
                  ? 'pl-4'
                  : node.level === 2
                    ? 'pl-8'
                    : node.level === 3
                      ? 'pl-12'
                      : node.level === 4
                        ? 'pl-16'
                        : node.level === 5
                          ? 'pl-20'
                          : 'pl-24',
              // Add cursor styles based on selectability
              node.type === 'directory' ||
                (props.isSelectableFunction && props.isSelectableFunction(node))
                ? 'cursor-pointer'
                : 'cursor-not-allowed opacity-60',
            ]" @click="
              node.type === 'directory' ||
                (props.isSelectableFunction && props.isSelectableFunction(node))
                ? handleSelectNode(node)
                : undefined
              ">
              <span v-if="node.type === 'directory'" :class="[
                'mr-1.5 w-4 shrink-0 cursor-pointer text-center transition-transform duration-200 ease-in-out',
                node.expanded ? 'mir-expand_more rotate-180' : 'mir-expand_more',
              ]" @click.stop="handleToggleNode(node)"></span>
              <!-- Placeholder for files to align with folders -->
              <span v-else class="mr-1.5 w-4 shrink-0"></span>

              <!-- Icon based on node type with enhanced visuals -->
              <span :class="[
                'mr-2 shrink-0',
                node.type === 'directory'
                  ? node.expanded
                    ? 'mir-folder_open text-amber-500'
                    : 'mir-folder text-amber-500'
                  : getFileIcon(node.name),
              ]"></span>

              <!-- Node name with better truncation -->
              <span class="flex-1 truncate">{{ node.name }}</span>

              <!-- Selection indicator for selectable files -->
              <span v-if="node.selected && node.type === 'file'"
                class="mir-check_circle ml-1.5 shrink-0 text-sm text-blue-500"></span>
            </div>

            <!-- Render children if expanded with animation -->
            <div v-if="node.expanded && node.children?.length"
              class="overflow-hidden pl-4 transition-all duration-200 ease-in-out">
              <template v-for="childNode in node.children" :key="childNode.key">
                <div v-if="!props.filterFunction || props.filterFunction(childNode)" class="w-full">
                  <div :class="[
                    'my-0.5 flex items-center rounded-md px-1 py-1.5 transition-colors duration-150 hover:bg-slate-100 dark:hover:bg-zinc-700',
                    childNode.selected
                      ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300'
                      : '',
                    childNode.level === 0
                      ? 'pl-0'
                      : childNode.level === 1
                        ? 'pl-4'
                        : childNode.level === 2
                          ? 'pl-8'
                          : childNode.level === 3
                            ? 'pl-12'
                            : childNode.level === 4
                              ? 'pl-16'
                              : childNode.level === 5
                                ? 'pl-20'
                                : 'pl-24',
                    // Add cursor styles based on selectability
                    childNode.type === 'directory' ||
                      (props.isSelectableFunction && props.isSelectableFunction(childNode))
                      ? 'cursor-pointer'
                      : 'cursor-not-allowed opacity-60',
                  ]" @click="
                    childNode.type === 'directory' ||
                      (props.isSelectableFunction && props.isSelectableFunction(childNode))
                      ? handleSelectNode(childNode)
                      : undefined
                    ">
                    <!-- Fixed spacing for files to align with folder toggle buttons -->
                    <span v-if="childNode.type === 'directory'" :class="[
                      'mr-1.5 w-4 shrink-0 cursor-pointer text-center transition-transform duration-200 ease-in-out',
                      childNode.expanded ? 'mir-expand_more rotate-180' : 'mir-expand_more',
                    ]" @click.stop="handleToggleNode(childNode)"></span>
                    <!-- Placeholder for files to align with folders -->
                    <span v-else class="mr-1.5 w-4 shrink-0"></span>

                    <!-- Icon based on node type -->
                    <span :class="[
                      'mr-2 shrink-0',
                      childNode.type === 'directory'
                        ? childNode.expanded
                          ? 'mir-folder_open text-amber-500'
                          : 'mir-folder text-amber-500'
                        : getFileIcon(childNode.name),
                    ]"></span>

                    <!-- Node name -->
                    <span class="flex-1 truncate">{{ childNode.name }}</span>

                    <!-- Selection indicator -->
                    <span v-if="childNode.selected && childNode.type === 'file'"
                      class="mir-check_circle ml-1.5 shrink-0 text-sm text-blue-500"></span>
                  </div>

                  <!-- Recursive rendering for deeper levels -->
                  <div v-if="childNode.expanded && childNode.children?.length"
                    class="overflow-hidden pl-4 transition-all duration-200 ease-in-out">
                    <div v-for="grandchildNode in childNode.children" :key="grandchildNode.key" class="w-full">
                      <div v-if="!props.filterFunction || props.filterFunction(grandchildNode)" :class="[
                        'my-0.5 flex items-center rounded-md px-1 py-1.5 transition-colors duration-150 hover:bg-slate-100 dark:hover:bg-zinc-700',
                        grandchildNode.selected
                          ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300'
                          : '',
                        grandchildNode.level === 0
                          ? 'pl-0'
                          : grandchildNode.level === 1
                            ? 'pl-4'
                            : grandchildNode.level === 2
                              ? 'pl-8'
                              : grandchildNode.level === 3
                                ? 'pl-12'
                                : grandchildNode.level === 4
                                  ? 'pl-16'
                                  : grandchildNode.level === 5
                                    ? 'pl-20'
                                    : 'pl-24',
                        // Add cursor styles based on selectability
                        grandchildNode.type === 'directory' ||
                          (props.isSelectableFunction && props.isSelectableFunction(grandchildNode))
                          ? 'cursor-pointer'
                          : 'cursor-not-allowed opacity-60',
                      ]" @click="
                        grandchildNode.type === 'directory' ||
                          (props.isSelectableFunction && props.isSelectableFunction(grandchildNode))
                          ? handleSelectNode(grandchildNode)
                          : undefined
                        ">
                        <!-- Fixed spacing for files to align with folder toggle buttons -->
                        <span v-if="grandchildNode.type === 'directory'" :class="[
                          'mr-1.5 w-4 shrink-0 cursor-pointer text-center transition-transform duration-200 ease-in-out',
                          grandchildNode.expanded
                            ? 'mir-expand_more rotate-180'
                            : 'mir-expand_more',
                        ]" @click.stop="handleToggleNode(grandchildNode)"></span>
                        <!-- Placeholder for files to align with folders -->
                        <span v-else class="mr-1.5 w-4 shrink-0"></span>

                        <!-- Icon based on node type -->
                        <span :class="[
                          'mr-2 shrink-0',
                          grandchildNode.type === 'directory'
                            ? grandchildNode.expanded
                              ? 'mir-folder_open text-amber-500'
                              : 'mir-folder text-amber-500'
                            : getFileIcon(grandchildNode.name),
                        ]"></span>

                        <!-- Node name -->
                        <span class="flex-1 truncate">{{ grandchildNode.name }}</span>

                        <!-- Selection indicator -->
                        <span v-if="grandchildNode.selected && grandchildNode.type === 'file'"
                          class="mir-check_circle ml-1.5 shrink-0 text-sm text-blue-500"></span>
                      </div>

                      <!-- Even deeper levels (handled with recursion in real implementation) -->
                      <div v-if="grandchildNode.expanded && grandchildNode.children?.length"
                        class="overflow-hidden pl-4 transition-all duration-200 ease-in-out">
                        <template v-for="deepNode in grandchildNode.children" :key="deepNode.key">
                          <div v-if="!props.filterFunction || props.filterFunction(deepNode)" :class="[
                            'my-0.5 flex items-center rounded-md px-1 py-1.5 transition-colors duration-150 hover:bg-slate-100 dark:hover:bg-zinc-700',
                            deepNode.selected
                              ? 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300'
                              : '',
                            // Add cursor styles based on selectability
                            deepNode.type === 'directory' ||
                              (props.isSelectableFunction && props.isSelectableFunction(deepNode))
                              ? 'cursor-pointer'
                              : 'cursor-not-allowed opacity-60',
                          ]" @click="
                            deepNode.type === 'directory' ||
                              (props.isSelectableFunction && props.isSelectableFunction(deepNode))
                              ? handleSelectNode(deepNode)
                              : undefined
                            ">
                            <!-- Fixed spacing for files to align with folder toggle buttons -->
                            <span v-if="deepNode.type === 'directory'" :class="[
                              'mr-1.5 w-4 shrink-0 cursor-pointer text-center transition-transform duration-200 ease-in-out',
                              deepNode.expanded
                                ? 'mir-expand_more rotate-180'
                                : 'mir-expand_more',
                            ]" @click.stop="handleToggleNode(deepNode)"></span>
                            <!-- Placeholder for files to align with folders -->
                            <span v-else class="mr-1.5 w-4 shrink-0"></span>

                            <span :class="[
                              'mr-2 shrink-0',
                              deepNode.type === 'directory'
                                ? 'mir-folder text-amber-500'
                                : getFileIcon(deepNode.name),
                            ]"></span>
                            <span class="flex-1 truncate">{{ deepNode.name }}</span>
                            <span v-if="deepNode.children?.length" class="ml-1 shrink-0 text-xs text-slate-400">({{
                              deepNode.children.length }})</span>
                            <span v-if="deepNode.selected && deepNode.type === 'file'"
                              class="mir-check_circle ml-1.5 shrink-0 text-sm text-blue-500"></span>
                          </div>
                        </template>
                      </div>
                    </div>
                  </div>
                </div>
              </template>
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
            {{ t('loading') }}
          </p>
        </div>
      </div>

      <!-- Enhanced empty state -->
      <div v-if="hasScanned && isEmpty"
        class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-white/60 backdrop-blur-sm dark:bg-zinc-900/60">
        <div class="flex flex-col items-center gap-3 rounded-lg bg-white p-6 shadow-sm dark:bg-zinc-800">
          <span class="mir-folder_off text-4xl text-slate-400 dark:text-slate-500"></span>
          <p class="text-sm font-medium text-slate-700 dark:text-slate-300">
            {{ t('zip_preview.no_files') }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
