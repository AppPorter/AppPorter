<script setup lang="ts">
import Button from 'primevue/button'
import { useI18n } from 'vue-i18n'
import LibraryStatusTag from './LibraryStatusTag.vue'

const { t } = useI18n()

interface LibraryItemProps {
    item: {
        timestamp: number
        url: string
        type: 'app' | 'tool'
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
    statusClick: [app: LibraryItemProps['item']]
}

defineProps<LibraryItemProps>()
defineEmits<LibraryItemEmits>()

function formatTimestamp(timestamp) {
    return new Date(timestamp * 1000).toLocaleDateString()
}
</script>

<template>
    <div class="w-full border-b border-surface-200 dark:border-surface-700">
        <div class="flex items-center p-4"
            @contextmenu.prevent="$emit('contextMenu', { originalEvent: $event, data: item })">
            <div class="mr-4 flex items-center justify-center">
                <div
                    class="flex size-10 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                    <img v-if="item.details.info.icon" :src="item.details.info.icon" class="size-8 object-contain"
                        alt="App Icon" />
                    <span v-else-if="item.type === 'tool'" class="mir-folder_zip text-2xl"></span>
                    <span v-else class="mir-apps text-2xl"></span>
                </div>
            </div>

            <div class="flex-1">
                <div class="mb-2 flex flex-col">
                    <span class="text-sm font-medium">{{ item.details.info.name || item.url }}</span>
                    <div v-if="item.type === 'app'"
                        class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                        <span>{{ item.details.info.version || t('cls.install.app.unknown_version') }}</span>
                        <span class="opacity-50">•</span>
                        <span>{{ item.details.info.publisher || t('cls.install.app.unknown_publisher') }}</span>
                    </div>
                    <div v-else-if="item.type === 'tool'"
                        class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                        <span>{{ t('ui.library.tool') }}</span>
                    </div>
                </div>

                <div class="flex items-center gap-2 text-xs">
                    <template v-if="item.details.paths.install_path">
                        <span class="break-all opacity-75">{{ item.details.paths.install_path }}</span>
                        <span class="opacity-50">•</span>
                    </template>
                    <span class="opacity-75">{{ formatTimestamp(item.timestamp) }}</span>
                </div>
            </div>

            <LibraryStatusTag :item="item" class="mr-2" @click="$emit('statusClick', item)" />

            <Button icon="mir-more_vert" outlined severity="secondary" class="size-8 p-0 shadow-sm"
                @click="$emit('contextMenu', { originalEvent: $event, data: item })" />
        </div>
    </div>
</template>
