<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import ProgressSpinner from 'primevue/progressspinner'
import SelectButton from 'primevue/selectbutton'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ZipPreview, { FileTreeNode } from './ZipPreview.vue'

const store = InstallConfigStore()

const { t } = useI18n()

// Props
const props = defineProps<{
    zipPath: string
    password?: string
    detailsLoading?: boolean
}>()

const emit = defineEmits<{
    (e: 'close'): void
    (e: 'loading', value: boolean): void
    (e: 'directory-select', path: string): void
    (e: 'update-file-tree', fileTree: FileTreeNode[]): void
}>()

// State
const filterMode = ref<'directory' | 'all'>('directory')
const selectedPath = ref('')
const isSelecting = ref(false)
const fileTree = ref<FileTreeNode[]>([])
const loading = ref(false)

// Load file tree when component mounts
onMounted(async () => {
    await loadFileTree()
})

async function loadFileTree() {
    if (!props.zipPath) return

    loading.value = true
    emit('loading', true)

    const result = await invoke('execute_command', {
        command: {
            name: 'GetArchiveTree',
            path: props.zipPath,
            password: props.password || '',
        },
    })

    const treeData = JSON.parse(result as string)
    fileTree.value = treeData
    emit('update-file-tree', treeData)
    loading.value = false
    emit('loading', false)
}

// Define the FileNode interface explicitly
interface FileNode {
    path?: string
    name: string
    type: string
}

// Filter modes
const FILTER_MODES = [
    {
        value: 'directory',
        label: t('ui.directory_selector.filter.directory'),
        icon: 'mir-folder',
    },
    {
        value: 'all',
        label: t('ui.directory_selector.filter.all'),
        icon: 'mir-description',
    },
]

// File filter function based on selected filter mode
const fileFilter = computed(() => {
    return (node: FileNode) => {
        // Always show directories for navigation
        if (node.type === 'directory') return true

        // For files, only show them if filter mode is 'all'
        return filterMode.value === 'all'
    }
})

// Determine if a node is selectable - only directories are selectable
const isSelectableNode = (node: FileNode): boolean => {
    return node.type === 'directory'
}

// Handle node selection with proper typing
function handleNodeSelect(node: FileNode) {
    // For directories, set as selected path
    if (node?.path && isSelectableNode(node)) {
        selectedPath.value = node.path
    }
}

async function handleSelect() {
    if (isSelecting.value || !selectedPath.value) return
    try {
        isSelecting.value = true
        emit('loading', true)

        // Set the path_directory value in the store
        if (store.page.includes('App')) {
            store.app_details.config.path_directory = selectedPath.value
        } else {
            store.tool_details.config.path_directory = selectedPath.value
        }

        // Emit selected directory
        emit('directory-select', selectedPath.value)

        // Close the drawer
        emit('close')
    } finally {
        isSelecting.value = false
        emit('loading', false)
    }
}
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
                :selected-path="selectedPath" :is-selectable-function="isSelectableNode" @node-click="handleNodeSelect"
                @update-file-tree="fileTree = $event" />
        </div>

        <!-- Bottom status and buttons -->
        <div class="mt-2 flex items-center justify-between gap-4">
            <div class="flex flex-1 items-center gap-2 text-sm">
                <span :class="[
                    selectedPath
                        ? 'mir-check_circle text-green-500'
                        : 'mir-info text-slate-500',
                ]"></span>
                <span class="font-medium">
                    {{
                        selectedPath
                            ? t('ui.directory_selector.selected')
                            : t('ui.directory_selector.select_prompt')
                    }}
                </span>
                <span v-if="selectedPath" class="truncate text-slate-600 dark:text-slate-400">
                    {{ selectedPath }}
                </span>
            </div>
            <div class="flex items-center">
                <ProgressSpinner v-if="isSelecting" style="width: 2rem; height: 2rem" strokeWidth="4" />
                <Button v-else severity="primary" :disabled="!selectedPath" @click="handleSelect" class="h-8 text-sm"
                    :label="t('g.select')" />
            </div>
        </div>
    </div>
</template>
