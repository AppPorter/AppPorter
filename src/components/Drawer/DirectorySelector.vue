<script setup lang="ts">
import DirectorySelector from '@/components/ZipPreview/DirectorySelector.vue';
import { generalStore } from '@/main';
import Drawer from 'primevue/drawer';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n()
const detailsLoading = ref(false)

const drawerState = computed({
    get: () => generalStore.drawer.directory_selector,
    set: (val: [boolean, string]) => generalStore.drawer.directory_selector = val
})

const visible = computed({
    get: () => drawerState.value[0],
    set: v => drawerState.value = [v, drawerState.value[1]]
})
const zipPath = computed(() => drawerState.value[1])

function handleClose() {
    visible.value = false
    drawerState.value = [false, '']
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}

function handleDirectorySelect(_directory: string) {
    handleClose()
}
</script>

<template>
    <Drawer :visible="visible" @update:visible="visible = $event" :header="t('ui.install.select_path_directory')"
        position="bottom" :style="{ height: '95vh' }" class="rounded-lg">
        <div class="h-full overflow-hidden">
            <DirectorySelector :zip-path="zipPath" :details-loading="detailsLoading" @close="handleClose"
                @loading="handleDetailsLoading" @directory-select="handleDirectorySelect" />
        </div>
    </Drawer>
</template>