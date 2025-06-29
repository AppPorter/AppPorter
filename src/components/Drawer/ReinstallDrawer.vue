<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import Drawer from 'primevue/drawer'
import InputText from 'primevue/inputtext'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { exec } from '@/exec'
import type { App, Tool } from '@/stores/library'

const { t } = useI18n()
const visible = ref(false)
const zipPath = ref('')
const inputError = ref(false)
const currentApp = ref<(App & { type: 'app' }) | (Tool & { type: 'tool' })>()

async function selectZipFile() {
    const selected = await open({
        multiple: false,
        directory: false,
        filters: [
            {
                name: t('g.archive'),
                extensions: ['zip', '7z', 'rar', 'tar', 'gz', 'bz2', 'xz', 'cab'],
            },
        ],
    })

    if (selected) {
        zipPath.value = selected
        inputError.value = false
    }
}

async function handleReinstall() {
    if (!zipPath.value) {
        inputError.value = true
        return
    }

    if (currentApp.value.type === 'app') {
        await exec('ReinstallApp', {
            timestamp: currentApp.value.timestamp,
            zip_path: zipPath.value
        })
    } else if (currentApp.value.type === 'tool') {
        await exec('ReinstallTool', {
            timestamp: currentApp.value.timestamp,
            zip_path: zipPath.value
        })
    }

    visible.value = false
}

function show(app: (App & { type: 'app' }) | (Tool & { type: 'tool' })) {
    currentApp.value = app
    zipPath.value = ''
    inputError.value = false
    visible.value = true
}

defineExpose({
    show
})
</script>

<template>
    <Drawer v-model:visible="visible" position="bottom" :style="{ height: '95vh' }" class="rounded-lg">
        <template #header>
            <div class="flex items-center gap-2">
                <span class="mir-refresh text-xl"></span>
                <div>
                    <h3 class="text-lg font-medium">{{ t('g.reinstall') }}</h3>
                    <p class="mt-0.5 text-xs text-surface-500" v-if="currentApp">
                        {{ currentApp.type === 'app' ? currentApp.details.info.name : currentApp.details.name }}
                    </p>
                </div>
            </div>
        </template>

        <div class="space-y-6">
            <div class="space-y-2">
                <label class="text-sm font-medium">{{ t('ui.select_placeholder.archive_or_url') }}</label>
                <div class="flex items-center gap-2">
                    <InputText v-model="zipPath" :placeholder="t('ui.select_placeholder.archive_or_url')"
                        :invalid="inputError" class="h-9 flex-1 text-sm" />
                    <Button @click="selectZipFile" severity="secondary" class="h-9 px-4" icon="mir-folder_open"
                        :label="t('g.browse')" />
                </div>
            </div>

            <div class="flex justify-end gap-2">
                <Button @click="visible = false" severity="secondary" outlined class="h-9 px-4"
                    :label="t('g.cancel')" />
                <Button @click="handleReinstall" :disabled="!zipPath" severity="primary" class="h-9 px-6"
                    icon="mir-refresh" :label="t('g.reinstall')" />
            </div>
        </div>
    </Drawer>
</template>