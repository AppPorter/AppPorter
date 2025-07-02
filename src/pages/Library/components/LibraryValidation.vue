<script setup lang="ts">
import { AppDetails } from '#/AppDetails'
import { ToolDetails } from '#/ToolDetails'
import ReinstallDrawer from '@/components/Drawer/Reinstall.vue'
import { exec } from '@/exec'
import { triggerUninstall } from '@/main'
import type { InstallTypes } from '@/stores/library'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface LibraryValidationProps {
    app?: {
        id: string
        type?: InstallTypes
        installed?: boolean
        details?: AppDetails | ToolDetails
        validation_status?: {
            file_exists?: boolean
            registry_valid?: boolean
            path_exists?: boolean
        }
    }
}

interface LibraryValidationEmits {
    loadLibrary: []
}

defineProps<LibraryValidationProps>()
const emit = defineEmits<LibraryValidationEmits>()

const appToValidate = ref()
const showDialog = ref(false)
const reinstallDrawer = ref()

function handleStatusClick(app: LibraryValidationProps['app']) {
    if (!app?.installed) {
        return
    }

    const validation = app.validation_status || {}
    const fileExists = validation?.file_exists
    const pathExists = validation?.path_exists

    if (app.type === 'tool') {
        if (!fileExists || !pathExists) {
            showDialog.value = true
        }
    } else {
        const registryValid = validation?.registry_valid
        if (!fileExists || !registryValid || !pathExists) {
            showDialog.value = true
        }
    }
}

async function handleValidationAction(action: 'reinstall' | 'repair' | 'uninstall') {
    if (!appToValidate.value) return

    if (action === 'uninstall') {
        await triggerUninstall(appToValidate.value.type, appToValidate.value.id)
        showDialog.value = false
    } else if (action === 'reinstall') {
        reinstallDrawer.value?.show(appToValidate.value)
        showDialog.value = false
    } else {
        const commandName = appToValidate.value.type === 'app' ? 'RepairApp' : 'RepairTool'

        await exec(commandName, {
            id: appToValidate.value.id,
        })
        emit('loadLibrary')
        showDialog.value = false
    }
}

function getActionType() {
    if (!appToValidate.value) return 'repair'
    const details = appToValidate.value.details
    if (appToValidate.value.type === 'tool') {
        if ('name' in details && (!appToValidate.value.validation_status?.file_exists)) {
            return 'reinstall'
        }
        return 'repair'
    }
    if ('info' in details && (!appToValidate.value.validation_status?.file_exists)) {
        return 'reinstall'
    }
    return 'repair'
}

defineExpose({
    handleStatusClick
})
</script>

<template>
    <Dialog v-model:visible="showDialog" :header="t('ui.validation.validation_error')" class="w-[32rem] max-w-[90vw]"
        modal>
        <div class="flex flex-col" v-if="appToValidate">
            <div class="mb-4">
                <template v-if="appToValidate.details && 'info' in appToValidate.details">
                    {{ t('ui.validation.issue', { name: appToValidate.details.info.name }) }}
                </template>
                <template v-else-if="appToValidate.details && 'name' in appToValidate.details">
                    {{ t('ui.validation.issue', { name: appToValidate.details.name }) }}
                </template>
                <template v-else>
                    {{ t('ui.validation.issue', { name: '' }) }}
                </template>
            </div>
            <div class="mb-6 space-y-2">
                <div class="flex items-center justify-between rounded p-2">
                    <span>File Exists</span>
                    <i
                        :class="appToValidate.validation_status?.file_exists ? 'mir-check text-green-500' : 'mir-close text-red-500'"></i>
                </div>
                <div v-if="appToValidate.type === 'app' && appToValidate.details && 'config' in appToValidate.details && appToValidate.details.config?.create_registry_key"
                    class="flex items-center justify-between rounded p-2">
                    <span>Registry Valid</span>
                    <i
                        :class="appToValidate.validation_status?.registry_valid ? 'mir-check text-green-500' : 'mir-close text-red-500'"></i>
                </div>
                <div v-if="appToValidate.details && (('config' in appToValidate.details && appToValidate.details.config?.add_to_path) || ('add_to_path' in appToValidate.details && appToValidate.details.add_to_path))"
                    class="flex items-center justify-between rounded p-2">
                    <span>Path Exists</span>
                    <i
                        :class="appToValidate.validation_status?.path_exists ? 'mir-check text-green-500' : 'mir-close text-red-500'"></i>
                </div>
            </div>
            <div class="flex justify-end gap-2">
                <Button :label="t('cls.uninstall.self')" icon="mir-delete" severity="danger" outlined
                    @click="handleValidationAction('uninstall')" />
                <Button :label="getActionType() === 'reinstall' ? t('g.reinstall') : t('g.repair')"
                    :icon="getActionType() === 'reinstall' ? 'mir-refresh' : 'mir-build'"
                    @click="handleValidationAction(getActionType())" />
            </div>
        </div>
    </Dialog>

    <ReinstallDrawer ref="reinstallDrawer" />
</template>