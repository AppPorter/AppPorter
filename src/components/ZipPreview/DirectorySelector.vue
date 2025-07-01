<script setup lang="ts">
import { exec } from '@/exec'
import Button from 'primevue/button'
import ProgressSpinner from 'primevue/progressspinner'
import SelectButton from 'primevue/selectbutton'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ZipPreview from './ZipPreview.vue'
import type { FileTreeNode } from '#/FileTreeNode'
import { generalStore, installConfig } from '@/main'

const { t } = useI18n()

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

const filterMode = ref<'directory' | 'all'>('directory')
const selectedPath = ref('')
const isSelecting = ref(false)
const fileTree = ref<FileTreeNode[]>([])
const loading = ref(false)

onMounted(async () => {
    await loadFileTree()
})

async function loadFileTree() {
    if (!props.zipPath) return

    loading.value = true
    emit('loading', true)

    const treeData = await exec('GetArchiveTree', {
        zip_path: props.zipPath,
        password: props.password || '',
    }) as FileTreeNode[]

    fileTree.value = treeData
    emit('update-file-tree', treeData)
    loading.value = false
    emit('loading', false)
}

interface FileNode {
    path?: string
    name: string
    type: string
}

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

const fileFilter = computed(() => {
    return (node: FileNode) => {
        if (node.type === 'directory') return true

        return filterMode.value === 'all'
    }
})

const isSelectableNode = (node: FileNode): boolean => {
    return node.type === 'directory'
}

function handleNodeSelect(node: FileNode) {
    if (node?.path && isSelectableNode(node)) {
        selectedPath.value = node.path
    }
}

async function handleSelect() {
    if (isSelecting.value || !selectedPath.value) return
    try {
        isSelecting.value = true
        emit('loading', true)

        if (generalStore.page.includes('App')) {
            installConfig.archive_path_dir = selectedPath.value
        } else {
            installConfig.archive_path_dir = selectedPath.value
        }

        emit('directory-select', selectedPath.value)

        emit('close')
    } finally {
        isSelecting.value = false
        emit('loading', false)
    }
}
</script>

<template>
    <div class="flex h-full flex-col">
        <div class="mb-2">
            <SelectButton v-model="filterMode" :options="FILTER_MODES" optionLabel="label" optionValue="value"
                :allowEmpty="false" size="small" class="w-full" />
        </div>

        <div class="min-h-0 flex-1 overflow-hidden">
            <ZipPreview :zip-path="zipPath" :password="password" :file-tree="fileTree" :filter-function="fileFilter"
                :selected-path="selectedPath" :is-selectable-function="isSelectableNode" @node-click="handleNodeSelect"
                @update-file-tree="fileTree = $event" />
        </div>

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
