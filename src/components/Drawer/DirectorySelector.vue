<script setup lang="ts">
import DirectorySelector from '@/components/ZipPreview/DirectorySelector.vue';
import { generalStore } from '@/main';
import Drawer from 'primevue/drawer';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n()
const detailsLoading = ref(false)

function handleClose() {
    generalStore.drawer.directory_selector = [false, '']
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}
</script>

<template>
    <Drawer :visible="generalStore.drawer.directory_selector[0]"
        @update:visible="generalStore.drawer.directory_selector[0] = $event"
        :header="t('ui.install.select_path_directory')" position="bottom" :style="{ height: '95vh' }"
        class="rounded-lg">
        <div class="h-full overflow-hidden">
            <DirectorySelector :zip-path="generalStore.drawer.directory_selector[1]" :details-loading="detailsLoading"
                @close="handleClose" @loading="handleDetailsLoading" @directory-select="handleClose" />
        </div>
    </Drawer>
</template>