<script setup lang="ts">
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { FileTreeNode } from '#/FileTreeNode'

interface FileNode {
  key: string
  name: string
  path: string
  type: 'file' | 'directory'
  children?: FileNode[]
  expanded?: boolean
  level: number
}

const props = defineProps<{
  zipPath: string
  password?: string
  filterFunction?: (node: FileNode) => boolean
  selectedPath?: string
  isSelectableFunction?: (node: FileNode) => boolean
  fileTree: FileTreeNode[]
}>()

const emits = defineEmits<{
  (event: 'node-click', node: FileNode): void
  (event: 'update-file-tree', fileTree: FileTreeNode[]): void
}>()

const { t } = useI18n()

const status = ref<'ready' | 'loading' | 'error'>('ready')
const error = ref<string>('')

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

const hasData = computed(() => props.fileTree?.length > 0)
const isEmpty = computed(() => status.value === 'ready' && (!props.fileTree || props.fileTree.length === 0))
const fileTree = computed(() => {
  return props.fileTree?.map(convertToFileNode) || []
})

function getFileIcon(fileName: string): string {
  const lowerName = fileName.toLowerCase()

  if (lowerName.endsWith('.exe')) return 'mir-terminal text-indigo-500'
  if (lowerName.endsWith('.bat') || lowerName.endsWith('.cmd')) return 'mir-terminal text-green-600'
  if (lowerName.endsWith('.ps1')) return 'mir-terminal text-blue-600'
  if (lowerName.endsWith('.sh')) return 'mir-terminal text-amber-600'

  if (['.zip', '.rar', '.7z', '.tar', '.gz'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-archive text-amber-600'

  if (['.pdf', '.doc', '.docx', '.txt', '.rtf'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-description text-blue-500'

  if (['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-image text-green-500'

  if (['.xml', '.json', '.ini', '.config', '.yml', '.yaml'].some((ext) => lowerName.endsWith(ext)))
    return 'mir-code text-slate-500'

  return 'mir-draft text-slate-500 dark:text-slate-400'
}

function handleToggleNode(node: FileNode) {
  function updateNodeExpansion(nodes: FileTreeNode[], targetKey: string): boolean {
    for (const treeNode of nodes) {
      if (treeNode.key === targetKey) {
        treeNode.expanded = !treeNode.expanded
        return true
      }
      if (treeNode.children && updateNodeExpansion(treeNode.children, targetKey)) {
        return true
      }
    }
    return false
  }

  const updatedTree = [...props.fileTree]
  updateNodeExpansion(updatedTree, node.key)
  emits('update-file-tree', updatedTree)
}

function handleSelectNode(node: FileNode) {
  const isSelectable = !props.isSelectableFunction || props.isSelectableFunction(node)

  if (isSelectable) {
    emits('node-click', node)
  }

  if (node.type === 'directory') {
    handleToggleNode(node)
  }
}

function flattenTree(nodes: FileNode[], level = 0): FileNode[] {
  const result: FileNode[] = []

  const sortedNodes = sortNodes([...nodes])

  for (const node of sortedNodes) {
    result.push({ ...node, level })

    if (node.type === 'directory' && node.expanded && node.children) {
      result.push(...flattenTree(node.children, level + 1))
    }
  }

  return result
}

function sortNodes(nodes: FileNode[]): FileNode[] {
  const executableExtensions = ['.exe', '.bat', '.cmd', '.ps1', '.sh', '.jar']

  return nodes.sort((a, b) => {
    if (a.type === 'directory' && b.type !== 'directory') return -1
    if (a.type !== 'directory' && b.type === 'directory') return 1

    if (a.type === 'directory' && b.type === 'directory') {
      return a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' })
    }

    const aIsExecutable = executableExtensions.some(ext => a.name.toLowerCase().endsWith(ext))
    const bIsExecutable = executableExtensions.some(ext => b.name.toLowerCase().endsWith(ext))

    if (aIsExecutable && !bIsExecutable) return -1
    if (!aIsExecutable && bIsExecutable) return 1

    return a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' })
  })
}

const flattenedTree = computed(() => {
  return flattenTree(fileTree.value)
})
</script>

<template>
  <div class="flex h-full min-h-0 flex-col">
    <div class="relative min-h-0 flex-1 overflow-hidden rounded-md border border-surface-200 dark:border-surface-700">
      <div v-if="hasData && !isEmpty" class="h-full overflow-auto px-3 py-2">
        <template v-for="node in flattenedTree" :key="node.key">
          <div v-if="!props.filterFunction || props.filterFunction(node)" class="w-full">
            <div :class="[
              'flex h-8 items-center rounded px-2 transition-colors duration-150',
              // Check if node is selectable - directories are always visually selectable
              node.type === 'directory' || (!props.isSelectableFunction || props.isSelectableFunction(node))
                ? 'cursor-pointer hover:bg-surface-100 dark:hover:bg-surface-700'
                : 'cursor-not-allowed opacity-50',
              props.selectedPath === node.path ? 'bg-primary-50 dark:bg-primary-900/20' : '',
            ]" :style="{ paddingLeft: `${node.level * 16 + 4}px` }" @click="handleSelectNode(node)">
              <span v-if="node.type === 'directory' && node.children && node.children.length > 0" :class="[
                'mr-2 text-sm transition-transform duration-200',
                node.expanded ? 'mir-expand_more' : 'mir-chevron_right'
              ]"></span>
              <span v-else class="mr-2"></span>

              <span :class="[
                'mr-2 text-sm',
                node.type === 'directory'
                  ? node.expanded
                    ? 'mir-folder_open text-amber-500'
                    : 'mir-folder text-amber-500'
                  : getFileIcon(node.name),
              ]"></span>

              <span class="flex-1 truncate text-sm">{{ node.name }}</span>

              <span v-if="node.type === 'file' && props.isSelectableFunction && !props.isSelectableFunction(node)"
                class="mir-block ml-2 text-sm opacity-50"></span>
            </div>
          </div>
        </template>
      </div>

      <div v-if="status === 'loading'" class="absolute inset-0 flex items-center justify-center">
        <div class="flex items-center gap-2">
          <span class="mir-progress_activity animate-spin text-xl"></span>
          <span class="text-sm">{{ t('g.loading') }}</span>
        </div>
      </div>

      <div v-if="hasData && isEmpty" class="absolute inset-0 flex items-center justify-center">
        <div class="flex flex-col items-center gap-2">
          <span class="mir-folder_off text-2xl opacity-50"></span>
          <span class="text-sm opacity-75">{{ t('ui.zip_preview.no_files') }}</span>
        </div>
      </div>

      <div v-if="status === 'error'" class="absolute inset-0 flex items-center justify-center">
        <div class="flex flex-col items-center gap-2">
          <span class="mir-error text-2xl text-red-500"></span>
          <span class="text-sm">{{ error || t('g.error') }}</span>
        </div>
      </div>
    </div>
  </div>
</template>