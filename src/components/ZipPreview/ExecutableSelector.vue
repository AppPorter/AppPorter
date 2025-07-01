<script setup lang="ts">
import { exec } from '@/exec'
import Button from 'primevue/button'
import SelectButton from 'primevue/selectbutton'
import SplitButton from 'primevue/splitbutton'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { FileTreeNode } from '#/FileTreeNode'
import type { ExeDetails } from '#/ExeDetails'
import { generalStore, installConfig } from '@/main'

const { t } = useI18n()

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

const filterMode = ref<'exe' | 'executable' | 'all'>('exe')
const selectedPath = ref('')
const isSelecting = ref(false)

interface FileNode {
  path?: string
  name: string
  type: string
}

const EXECUTABLE_EXTENSIONS = ['.exe', '.bat', '.cmd', '.ps1', '.sh', '.jar']

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

const fileFilter = computed(() => {
  return (node: FileNode) => {
    if (node.type === 'directory') return true

    switch (filterMode.value) {
      case 'exe':
        return node.name.toLowerCase().endsWith('.exe')
      case 'executable':
        return EXECUTABLE_EXTENSIONS.some((ext) => node.name.toLowerCase().endsWith(ext))
      case 'all':
      default:
        return true
    }
  }
})

const isSelectableFile = (node: FileNode): boolean => {
  if (node.type !== 'file') return false

  return EXECUTABLE_EXTENSIONS.some((ext) => node.name.toLowerCase().endsWith(ext))
}

const hasExecutableFiles = computed(() => {
  const checkNode = (node: FileTreeNode): boolean => {
    if (node.node_type === 'file') {
      return EXECUTABLE_EXTENSIONS.some((ext) => node.name.toLowerCase().endsWith(ext))
    }

    if (node.node_type === 'directory' && node.children) {
      return node.children.some((child) => checkNode(child))
    }

    return false
  }

  return props.fileTree.some((node) => checkNode(node))
})



function handleNodeSelect(node: FileNode) {
  if (node?.path && isSelectableFile(node)) {
    selectedPath.value = node.path
  }
}

async function handleSelect() {
  if (isSelecting.value) return
  isSelecting.value = true
  emit('loading', true)
  const details = await exec<ExeDetails>('GetDetails', {
    path: {
      zip_path: props.zipPath,
      executable_path: selectedPath.value,
      password: props.password,
    }
  })

  installConfig.app_details.info.name = details.product_name
  installConfig.app_details.info.version = details.version
  installConfig.app_details.info.publisher = details.copyright
  installConfig.app_details.info.icon = details.icon_data_url
  installConfig.archive_exe_path = selectedPath.value

  emit('executable-selected')
  isSelecting.value = false
  emit('loading', false)
}

function handleNoExecutable() {
  emit('no-executable')
}

async function handleInstallerMode() {
  if (isSelecting.value) return
  isSelecting.value = true
  emit('loading', true)

  await exec('RunInstaller', {
    path: {
      zip_path: props.zipPath,
      executable_path: selectedPath.value,
      password: props.password,
    }
  })

  isSelecting.value = false
  emit('loading', false)
  generalStore.drawer.preview = false
}

const menuItems = ref([
  {
    label: t('cls.install.installer_mode'),
    icon: 'mir-settings',
    command: handleInstallerMode
  }
])

function findExeFilesInDirectory(nodes: FileTreeNode[], maxDepth: number = 3, currentDepth: number = 0): string[] {
  if (currentDepth >= maxDepth) return []

  const exeFiles: string[] = []

  for (const node of nodes) {
    if (node.node_type === 'file' && node.name.toLowerCase().endsWith('.exe')) {
      exeFiles.push(node.path)
    }
  }

  return exeFiles
}

function findSingleDirectoryPath(nodes: FileTreeNode[]): string | null {
  const directories = nodes.filter(node => node.node_type === 'directory')
  return directories.length === 1 ? directories[0].path : null
}

async function autoSelectExe() {
  let currentNodes = props.fileTree
  let searchDepth = 0
  const maxDepth = 3

  while (searchDepth < maxDepth) {
    const exeFiles = findExeFilesInDirectory(currentNodes, 1, 0)

    if (exeFiles.length === 1) {
      selectedPath.value = exeFiles[0]
      return
    } else if (exeFiles.length > 1) {
      return
    }

    if (searchDepth < maxDepth - 1) {
      const singleDirPath = findSingleDirectoryPath(currentNodes)
      if (singleDirPath) {
        const dirNode = currentNodes.find(node => node.path === singleDirPath)
        if (dirNode && dirNode.children && dirNode.children.length > 0) {
          currentNodes = dirNode.children
          searchDepth++
          continue
        }
      }
    }

    break
  }
}

watch(() => props.fileTree, (newTree) => {
  if (newTree && newTree.length > 0 && !selectedPath.value) {
    autoSelectExe()
  }
}, { immediate: true })
</script>

<template>
  <div class="flex h-full flex-col">
    <div class="mb-2">
      <SelectButton v-model="filterMode" :options="FILTER_MODES" optionLabel="label" optionValue="value"
        :allowEmpty="false" size="small" class="w-full" />
    </div>

    <div class="min-h-0 flex-1 overflow-hidden">
      <ZipPreview :zip-path="zipPath" :password="password" :file-tree="fileTree" :filter-function="fileFilter"
        :selected-path="selectedPath" :is-selectable-function="isSelectableFile" @node-click="handleNodeSelect"
        @update-file-tree="emit('update-file-tree', $event)" />
    </div>

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
        <Button severity="secondary" @click="handleNoExecutable" :label="t('ui.executable_selector.no_executable')"
          class="h-8 text-sm" icon="mir-rule" outlined :disabled="isSelecting" />
        <SplitButton v-if="hasExecutableFiles" severity="primary" :disabled="!selectedPath || isSelecting"
          @click="handleSelect" class="h-8 text-sm" :model="menuItems">
          <template #default>
            <div class="flex items-center gap-2">
              <div v-if="isSelecting"
                class="size-4 animate-spin rounded-full border-2 border-white border-t-transparent"></div>
              <span v-else class="mir-check"></span>
              <span>{{ t('g.select') }}</span>
            </div>
          </template>
        </SplitButton>
      </div>
    </div>
  </div>
</template>
