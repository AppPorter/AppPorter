<script setup lang="ts">
import DirectorySelector from '@/components/ZipPreview/DirectorySelector.vue';
import Drawer from 'primevue/drawer';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

defineProps<{
    visible: boolean
    zipPath: string
}>()

const emit = defineEmits<{
    'update:visible': [value: boolean]
    'directory-select': [directory: string]
}>()

const { t } = useI18n()
const detailsLoading = ref(false)

function handleClose() {
    emit('update:visible', false)
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}

function handleDirectorySelect(directory: string) {
    emit('directory-select', directory)
}
</script>

<template>
    <Drawer :visible="visible" @update:visible="$emit('update:visible', $event)"
        :header="t('ui.install.select_path_directory')" position="bottom" :style="{ height: '95vh' }"
        class="rounded-lg">
        <div class="h-full overflow-hidden">
            <DirectorySelector :zip-path="zipPath" :details-loading="detailsLoading" @close="handleClose"
                @loading="handleDetailsLoading" @directory-select="handleDirectorySelect" />
        </div>
    </Drawer>
</template>