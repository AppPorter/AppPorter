<script setup lang="ts">
import { InstallTypes } from '@/stores/library'
import Button from 'primevue/button'
import ConfirmDialog from 'primevue/confirmdialog'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import LibraryStatusTag from './LibraryStatusTag.vue'
import LibraryValidation from './LibraryValidation.vue'

const { t } = useI18n()
const validationRef = ref()

interface LibraryItemProps {
    item: {
        timestamp: number
        url: string
        type: InstallTypes
        installed: boolean
        details: {
            info: {
                name: string
                icon: string
                publisher: string
                version: string
            }
            paths: {
                install_path: string
            }
            validation_status: {
                file_exists: boolean
                registry_valid: boolean
                path_exists: boolean
            }
        }
    }
}

interface LibraryItemEmits {
    contextMenu: [event: { originalEvent: Event; data: LibraryItemProps['item'] }]
    loadLibrary: []
}

defineProps<LibraryItemProps>()
defineEmits<LibraryItemEmits>()

function formatTimestamp(timestamp) {
    return new Date(timestamp * 1000).toLocaleDateString()
}

function handleStatusClick(app: LibraryItemProps['item']) {
    if (validationRef.value) {
        validationRef.value.handleStatusClick(app)
    }
}
</script>

<template>
    <div class="w-full border-t border-surface-200 dark:border-surface-700">
        <div class="flex items-center p-4"
            @contextmenu.prevent="$emit('contextMenu', { originalEvent: $event, data: item })">
            <div class="mr-4 flex items-center justify-center">
                <div
                    class="flex size-10 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                    <img v-if="item.details.info.icon" :src="item.details.info.icon" class="size-8 object-contain"
                        alt="App Icon" />
                    <span v-else-if="item.type === 'url'" class="mir-link text-2xl"></span>
                    <span v-else-if="item.type === 'tool'" class="mir-folder_zip text-2xl"></span>
                    <span v-else class="mir-apps text-2xl"></span>
                </div>
            </div>

            <div class="min-w-0 flex-1">
                <div class="mb-2 flex flex-col">
                    <span class="truncate text-sm font-medium">{{ item.details.info.name || item.url }}</span>
                    <div v-if="item.type === 'app'"
                        class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                        <span>{{ item.details.info.version || t('cls.install.app.unknown_version') }}</span>
                    </div>
                </div>

                <div class="flex flex-col gap-1 text-xs">
                    <span class="opacity-75">{{ formatTimestamp(item.timestamp) }}</span>
                    <span v-if="item.details.paths?.install_path" class="break-all opacity-75">{{
                        item.details.paths.install_path }}</span>
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
