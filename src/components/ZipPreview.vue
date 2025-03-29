<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
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

// State
const status = ref<FileStatus>('ready')
const fileTree = ref<FileNode[]>([])
const fileCache = ref<string[] | null>(null)

// Computed properties
const hasScanned = computed(() => fileCache.value !== null)
const isEmpty = computed(() => hasScanned.value && fileTree.value.length === 0)

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
          selected: false
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
  if (!props.zipPath) return

  status.value = 'loading'
  emits('loading', true)

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
  emits('loading', false)
}

// Toggle node expansion
function handleToggleNode(node: FileNode) {
  node.expanded = !node.expanded
}

// Handle node selection
function handleSelectNode(node: FileNode) {
  // For directories, just toggle expansion instead of selecting
  if (node.type === 'directory') {
    handleToggleNode(node)
    return
  }
  
  // For files, proceed with selection logic
  // Deselect all nodes first
  const deselectAll = (nodes: FileNode[]) => {
    nodes.forEach(node => {
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
  if (!props.zipPath) return
  fileCache.value = null
  loadZipContent()
})

// Setup event listeners and load initial content
onMounted(() => {
  if (props.zipPath) loadZipContent()
})

// Expose component interface
defineExpose({
  // Exporting types as string literals for parent components to reference
  nodeTypes: ['file', 'directory'] as const,
  // Provide methods that parent can use
  selectNode: handleSelectNode,
  toggleNode: handleToggleNode
})
</script>

<template>
  <div class="flex h-full min-h-0 flex-col">
    <!-- File Tree Container with improved scrolling -->
    <div class="relative min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 shadow-sm dark:border-zinc-600">
      <!-- Scrollable Tree Content -->
      <div v-if="hasScanned && !isEmpty" class="h-full overflow-auto p-2 text-sm">
        <!-- Recursive Tree Node Template -->
        <template v-for="node in fileTree" :key="node.key">
          <div v-if="!props.filterFunction || props.filterFunction(node)" class="w-full">
            <div 
              :class="[
                'flex items-center py-1 hover:bg-slate-100 dark:hover:bg-zinc-700',
                node.selected ? 'bg-blue-50 dark:bg-blue-900/20' : '',
                node.level === 0 ? 'pl-0' : 
                node.level === 1 ? 'pl-4' : 
                node.level === 2 ? 'pl-8' : 
                node.level === 3 ? 'pl-12' : 
                node.level === 4 ? 'pl-16' : 
                node.level === 5 ? 'pl-20' : 'pl-24',
                // Add cursor styles based on selectability
                node.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(node))
                  ? 'cursor-pointer' 
                  : 'cursor-not-allowed opacity-60'
              ]"
              @click="node.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(node)) 
                ? handleSelectNode(node) 
                : undefined"
            >
              <!-- Toggle button for directories -->
              <span 
                v-if="node.type === 'directory'" 
                :class="[
                  'mr-1 cursor-pointer transition-transform',
                  node.expanded ? 'mir-expand_more rotate-180' : 'mir-expand_more'
                ]"
                @click.stop="handleToggleNode(node)"
              ></span>
              
              <!-- Icon based on node type -->
              <span 
                :class="[
                  'mr-2', 
                  node.type === 'directory' 
                    ? (node.expanded ? 'mir-folder_open text-amber-500' : 'mir-folder text-amber-500') 
                    : 'mir-draft text-slate-500 dark:text-slate-400'
                ]"
              ></span>
              
              <!-- Node name -->
              <span class="truncate">{{ node.name }}</span>
            </div>
            
            <!-- Render children if expanded -->
            <div v-if="node.expanded && node.children?.length" class="pl-4">
              <template v-for="childNode in node.children" :key="childNode.key">
                <div v-if="!props.filterFunction || props.filterFunction(childNode)" class="w-full">
                  <div 
                    :class="[
                      'flex items-center py-1 hover:bg-slate-100 dark:hover:bg-zinc-700',
                      childNode.selected ? 'bg-blue-50 dark:bg-blue-900/20' : '',
                      childNode.level === 0 ? 'pl-0' : 
                      childNode.level === 1 ? 'pl-4' : 
                      childNode.level === 2 ? 'pl-8' : 
                      childNode.level === 3 ? 'pl-12' : 
                      childNode.level === 4 ? 'pl-16' : 
                      childNode.level === 5 ? 'pl-20' : 'pl-24',
                      // Add cursor styles based on selectability
                      childNode.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(childNode))
                        ? 'cursor-pointer' 
                        : 'cursor-not-allowed opacity-60'
                    ]"
                    @click="childNode.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(childNode)) 
                      ? handleSelectNode(childNode) 
                      : undefined"
                  >
                    <!-- Toggle button for directories -->
                    <span 
                      v-if="childNode.type === 'directory'" 
                      :class="[
                        'mr-1 cursor-pointer transition-transform',
                        childNode.expanded ? 'mir-expand_more rotate-180' : 'mir-expand_more'
                      ]"
                      @click.stop="handleToggleNode(childNode)"
                    ></span>
                    
                    <!-- Icon based on node type -->
                    <span 
                      :class="[
                        'mr-2', 
                        childNode.type === 'directory' 
                          ? (childNode.expanded ? 'mir-folder_open text-amber-500' : 'mir-folder text-amber-500') 
                          : 'mir-draft text-slate-500 dark:text-slate-400'
                      ]"
                    ></span>
                    
                    <!-- Node name -->
                    <span class="truncate">{{ childNode.name }}</span>
                  </div>
                  
                  <!-- Recursive rendering for deeper levels -->
                  <div v-if="childNode.expanded && childNode.children?.length" class="pl-4">
                    <div v-for="grandchildNode in childNode.children" :key="grandchildNode.key" class="w-full">
                      <div v-if="!props.filterFunction || props.filterFunction(grandchildNode)" 
                        :class="[
                          'flex items-center py-1 hover:bg-slate-100 dark:hover:bg-zinc-700',
                          grandchildNode.selected ? 'bg-blue-50 dark:bg-blue-900/20' : '',
                          grandchildNode.level === 0 ? 'pl-0' : 
                          grandchildNode.level === 1 ? 'pl-4' : 
                          grandchildNode.level === 2 ? 'pl-8' : 
                          grandchildNode.level === 3 ? 'pl-12' : 
                          grandchildNode.level === 4 ? 'pl-16' : 
                          grandchildNode.level === 5 ? 'pl-20' : 'pl-24',
                          // Add cursor styles based on selectability
                          grandchildNode.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(grandchildNode))
                            ? 'cursor-pointer' 
                            : 'cursor-not-allowed opacity-60'
                        ]"
                        @click="grandchildNode.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(grandchildNode)) 
                          ? handleSelectNode(grandchildNode) 
                          : undefined"
                      >
                        <!-- Toggle button for directories -->
                        <span 
                          v-if="grandchildNode.type === 'directory'" 
                          :class="[
                            'mr-1 cursor-pointer transition-transform',
                            grandchildNode.expanded ? 'mir-expand_more rotate-180' : 'mir-expand_more'
                          ]"
                          @click.stop="handleToggleNode(grandchildNode)"
                        ></span>
                        
                        <!-- Icon based on node type -->
                        <span 
                          :class="[
                            'mr-2', 
                            grandchildNode.type === 'directory' 
                              ? (grandchildNode.expanded ? 'mir-folder_open text-amber-500' : 'mir-folder text-amber-500') 
                              : 'mir-draft text-slate-500 dark:text-slate-400'
                          ]"
                        ></span>
                        
                        <!-- Node name -->
                        <span class="truncate">{{ grandchildNode.name }}</span>
                      </div>
                      
                      <!-- Even deeper levels (handled with recursion in real implementation) -->
                      <div v-if="grandchildNode.expanded && grandchildNode.children?.length" class="pl-4">
                        <template v-for="deepNode in grandchildNode.children" :key="deepNode.key">
                          <div 
                            v-if="!props.filterFunction || props.filterFunction(deepNode)"
                            :class="[
                              'flex items-center py-1 pl-4 hover:bg-slate-100 dark:hover:bg-zinc-700',
                              deepNode.selected ? 'bg-blue-50 dark:bg-blue-900/20' : '',
                              // Add cursor styles based on selectability
                              deepNode.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(deepNode))
                                ? 'cursor-pointer' 
                                : 'cursor-not-allowed opacity-60'
                            ]"
                            @click="deepNode.type === 'directory' || (props.isSelectableFunction && props.isSelectableFunction(deepNode)) 
                              ? handleSelectNode(deepNode) 
                              : undefined"
                          >
                            <span 
                              :class="[
                                'mr-2',
                                deepNode.type === 'directory' ? 'mir-folder text-amber-500' : 'mir-draft text-slate-500 dark:text-slate-400'
                              ]"
                            ></span>
                            <span class="truncate">{{ deepNode.name }}</span>
                            <span v-if="deepNode.children?.length" class="ml-1 text-xs text-slate-400">({{ deepNode.children.length }} items)</span>
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

      <!-- Loading indicator -->
      <div 
        v-if="status === 'loading'" 
        class="absolute inset-0 flex items-center justify-center bg-white/80 backdrop-blur-[0.125rem] dark:bg-zinc-900/80"
      >
        <div class="flex flex-col items-center gap-2">
          <span class="mir-progress_activity text-2xl"></span>
          <p class="text-sm text-slate-600 dark:text-slate-300">
            {{ t('common.loading') }}
          </p>
        </div>
      </div>

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
