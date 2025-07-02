<script setup lang="ts">
import { AppDetails } from '#/AppDetails'
import { ToolDetails } from '#/ToolDetails'
import { ItemTypes } from '@/stores/library'
import Button from 'primevue/button'
import ConfirmDialog from 'primevue/confirmdialog'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import LibraryStatusTag from './StatusTag.vue'
import LibraryValidation from './Validation.vue'

const { t } = useI18n()
const validationRef = ref()

interface LibraryItemProps {
    item: {
        id: string
        timestamp_add?: string
        timestamp_update?: string
        url: string
        type: ItemTypes
        installed?: boolean
        details?: AppDetails | ToolDetails
        validation_status?: {
            file_exists?: boolean
            registry_valid?: boolean
            path_exists?: boolean
        }
    }
}

interface LibraryItemEmits {
    contextMenu: [event: { originalEvent: Event; data: LibraryItemProps['item'] }]
    loadLibrary: []
}

defineProps<LibraryItemProps>()
defineEmits<LibraryItemEmits>()

function formatTimestamp(timestamp: string) {
    if (!timestamp) return ''
    const date = new Date(timestamp)
    if (isNaN(date.getTime())) return ''
    return date.toLocaleDateString()
}

function handleStatusClick(app: LibraryItemProps['item']) {
    if (validationRef.value) {
        validationRef.value.handleStatusClick(app)
    }
}

function getName(item: LibraryItemProps['item']) {
    if (item.type === 'app' && item.details && 'info' in item.details) {
        return (item.details as AppDetails).info.name || item.url
    } else if (item.type === 'tool' && item.details && 'name' in item.details) {
        return (item.details as ToolDetails).name || item.url
    } else if (item.type === 'url') {
        return item.url
    }
    return item.url
}

function getIcon(item: LibraryItemProps['item']) {
    if (item.type === 'app' && item.details && 'info' in item.details) {
        return (item.details as AppDetails).info.icon
    } else if (item.type === 'tool' && item.details && 'icon' in item.details) {
        return ''
    }
    return ''
}

function getVersion(item: LibraryItemProps['item']) {
    if (item.type === 'app' && item.details && 'info' in item.details) {
        return (item.details as AppDetails).info.version
    } else if (item.type === 'tool' && item.details && 'version' in item.details) {
        return ''
    }
    return ''
}

function getInstallPath(item: LibraryItemProps['item']) {
    if (item.type === 'app' && item.details && 'install_path' in item.details) {
        return (item.details as AppDetails).install_path
    } else if (item.type === 'tool' && item.details && 'install_path' in item.details) {
        return (item.details as ToolDetails).install_path
    }
    return ''
}
</script>

<template>
    <div class="w-full border-t border-surface-200 dark:border-surface-700">
        <div class="flex items-center p-4"
            @contextmenu.prevent="$emit('contextMenu', { originalEvent: $event, data: item })">
            <div class="mr-4 flex items-center justify-center">
                <div
                    class="flex size-10 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                    <img v-if="getIcon(item)" :src="getIcon(item)" class="size-8 object-contain" alt="App Icon" />
                    <span v-else-if="item.type === 'url'" class="mir-link text-2xl"></span>
                    <span v-else-if="item.type === 'tool'" class="mir-folder_zip text-2xl"></span>
                    <span v-else class="mir-apps text-2xl"></span>
                </div>
            </div>

            <div class="min-w-0 flex-1">
                <div class="mb-2 flex flex-col">
                    <span class="truncate text-sm font-medium">{{ getName(item) }}</span>
                    <div v-if="item.type === 'app'"
                        class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                        <span>{{ getVersion(item) || t('cls.install.app.unknown_version') }}</span>
                    </div>
                    <div v-else-if="item.type === 'tool'"
                        class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                        <span>{{ getVersion(item) || t('cls.install.app.unknown_version') }}</span>
                    </div>
                </div>

                <div class="flex flex-col gap-1 text-xs">
                    <span class="opacity-75">{{ formatTimestamp(item.timestamp_add) }}</span>
                    <span v-if="getInstallPath(item)" class="break-all opacity-75">{{
                        getInstallPath(item) }}</span>
                </div>
            </div>

            <div class="mr-2 flex items-center gap-2">
                <LibraryStatusTag :item="item" tag-type="type" @click="handleStatusClick(item)" />
                <LibraryStatusTag :item="item" tag-type="status" @click="handleStatusClick(item)" />
            </div>

            <Button icon="mir-more_vert" outlined severity="secondary" class="size-8 p-0 shadow-sm"
                @click="$emit('contextMenu', { originalEvent: $event, data: item })" />
        </div>
        <LibraryValidation ref="validationRef" :app="item" @load-library="$emit('loadLibrary')" />

        <ConfirmDialog />
    </div>
</template>
