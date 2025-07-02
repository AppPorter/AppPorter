<script setup lang="ts">
import { generalStore, libraryStore } from '@/main'
import { ItemTypes } from '@/stores/library'
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const itemId = ref()
const itemType = ref<ItemTypes>()
const item = ref()
const loading = ref(false)
const error = ref('')

watch(() => generalStore.drawer.uninstall[1], async (id) => {
    if (id) {
        itemId.value = id
        try {
            const app = await libraryStore.getApp(id)
            if (app) {
                itemType.value = 'app'
                item.value = app
            }
        } catch {
            const tool = await libraryStore.getTool(id)
            if (tool) {
                itemType.value = 'tool'
                item.value = tool
            }
        }
    } else {
        itemType.value = null
        item.value = null
    }
})

async function handleUninstall() {
    if (!item.value || !itemType.value) return
    loading.value = true
    if (item.value.installed) {
        await libraryStore.executeUninstall(itemType.value, item.value.id)
    } else {
        await libraryStore.remove(item.value.id)
    }
    generalStore.drawer.uninstall = [false, '']
}
</script>

<template>
    <Drawer v-model:visible="generalStore.drawer.uninstall[0]" position="bottom" :style="{ height: '40vh' }"
        class="rounded-lg">
        <template #header>
            <div class="flex items-center gap-2">
                <span class="mir-warning text-xl"></span>
                <div>
                    <h3 class="text-lg font-medium">{{ item && item.installed ? (itemType === 'tool' ?
                        t('ui.library.confirm_delete_header') : t('ui.library.confirm_uninstall_header', {
                            name:
                                item.details.info?.name
                        })) : t('ui.library.confirm_remove_header') }}</h3>
                    <p class="mt-0.5 text-xs text-surface-500" v-if="item">
                        {{ itemType === 'app' ? item.details.info?.name : item.details.name }}
                    </p>
                </div>
            </div>
        </template>
        <div v-if="item" class="space-y-6">
            <div class="text-base">
                <span v-if="item.installed">
                    {{ itemType === 'tool' ? t('ui.library.confirm_delete_message', { name: item.details.name })
                        :
                        t('ui.library.confirm_uninstall_message', { name: item.details.info?.name }) }}
                </span>
                <span v-else>
                    {{ t('ui.library.confirm_remove_message', {
                        name: itemType === 'app' ? item.details.info?.name
                            : item.details.name
                    }) }}
                </span>
            </div>
            <div v-if="error" class="text-sm text-red-500">{{ error }}</div>
            <div class="flex justify-end gap-2">
                <Button @click="generalStore.drawer.uninstall[0] = false" severity="secondary" outlined class="h-9 px-4"
                    :label="t('g.cancel')" />
                <Button @click="handleUninstall" :loading="loading" severity="danger" class="h-9 px-6"
                    :icon="itemType === 'tool' || !item.installed ? 'mir-delete' : 'mir-warning'"
                    :label="item && item.installed ? (itemType === 'tool' ? t('g.delete') : t('cls.uninstall.self')) : t('g.remove')" />
            </div>
        </div>
    </Drawer>
</template>