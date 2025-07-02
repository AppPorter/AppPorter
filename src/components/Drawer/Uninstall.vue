<script setup lang="ts">
import { generalStore, libraryStore } from '@/main'
import type { InstallTypes } from '@/stores/library'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const drawerState = computed({
    get: () => generalStore.drawer.uninstall,
    set: (val: [boolean, string]) => generalStore.drawer.uninstall = val
})
const visible = computed({
    get: () => drawerState.value[0],
    set: v => drawerState.value = [v, drawerState.value[1]]
})
const appId = computed(() => drawerState.value[1])
const appInfo = ref<{ apptype: InstallTypes, id: string } | null>(null)
const app = ref()
watch(appId, async (id) => {
    if (id) {
        appInfo.value = { apptype: 'app', id }
        app.value = await libraryStore.getById(id)
    } else {
        appInfo.value = null
        app.value = null
    }
})
const loading = ref(false)
const error = ref('')

function show(apptype: InstallTypes, id: string) {
    drawerState.value = [true, id]
    error.value = ''
}

async function handleUninstall() {
    if (!app.value || !appInfo.value) return
    loading.value = true
    if (app.value.installed) {
        await libraryStore.executeUninstall(appInfo.value.apptype, appInfo.value.id)
    } else {
        await libraryStore.remove(appInfo.value.id)
    }
    visible.value = false
    drawerState.value = [false, '']
}

defineExpose({ show })
</script>

<template>
    <Drawer v-model:visible="visible" position="bottom" :style="{ height: '40vh' }" class="rounded-lg">
        <template #header>
            <div class="flex items-center gap-2">
                <span class="mir-warning text-xl"></span>
                <div>
                    <h3 class="text-lg font-medium">{{ app && app.installed ? (appInfo?.apptype === 'tool' ?
                        t('ui.library.confirm_delete_header') : t('ui.library.confirm_uninstall_header', {
                            name:
                                app.details.info?.name
                        })) : t('ui.library.confirm_remove_header') }}</h3>
                    <p class="mt-0.5 text-xs text-surface-500" v-if="app">
                        {{ appInfo?.apptype === 'app' ? app.details.info.name : app.details.name }}
                    </p>
                </div>
            </div>
        </template>
        <div v-if="app" class="space-y-6">
            <div class="text-base">
                <span v-if="app.installed">
                    {{ appInfo?.apptype === 'tool' ? t('ui.library.confirm_delete_message', { name: app.details.name })
                        :
                        t('ui.library.confirm_uninstall_message', { name: app.details.info.name }) }}
                </span>
                <span v-else>
                    {{ t('ui.library.confirm_remove_message', {
                        name: appInfo?.apptype === 'app' ? app.details.info.name
                            :
                            app.details.name
                    }) }}
                </span>
            </div>
            <div v-if="error" class="text-sm text-red-500">{{ error }}</div>
            <div class="flex justify-end gap-2">
                <Button @click="visible = false" severity="secondary" outlined class="h-9 px-4"
                    :label="t('g.cancel')" />
                <Button @click="handleUninstall" :loading="loading" severity="danger" class="h-9 px-6"
                    :icon="appInfo?.apptype === 'tool' || !app.installed ? 'mir-delete' : 'mir-warning'"
                    :label="app && app.installed ? (appInfo?.apptype === 'tool' ? t('g.delete') : t('cls.uninstall.self')) : t('g.remove')" />
            </div>
        </div>
    </Drawer>
</template>