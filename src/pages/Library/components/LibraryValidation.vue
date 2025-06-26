<script setup lang="ts">
import type { InstallTypes } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import { inject, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const triggerUninstall = inject('triggerUninstall') as (apptype: InstallTypes, timestamp: number) => Promise<void>

interface LibraryValidationProps {
    app?: {
        timestamp: number
        type: InstallTypes
        installed: boolean
        details: {
            info: {
                name: string
            }
            validation_status: {
                file_exists: boolean
                registry_valid: boolean
                path_exists: boolean
            }
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

function handleStatusClick(app: LibraryValidationProps['app']) {
    if (!app?.installed) {
        return
    }

    appToValidate.value = app

    if (app.type === 'tool') {
        return;
    }

    const validation = app.details.validation_status
    const fileExists = validation.file_exists
    const registryValid = validation.registry_valid
    const pathExists = validation.path_exists

    if (!fileExists || !registryValid || !pathExists) {
        showDialog.value = true
    }
}

async function handleValidationAction(action: 'reinstall' | 'repair' | 'uninstall') {
    if (!appToValidate.value) return

    if (action === 'uninstall') {
        await triggerUninstall(appToValidate.value.type, appToValidate.value.timestamp)
    } else {
        await invoke('execute_command', {
            command: {
                name: action === 'reinstall' ? 'Reinstall' : 'Repair',
                timestamp: appToValidate.value.timestamp,
            },
        })
        emit('loadLibrary')
    }

    showDialog.value = false
}

function getActionType() {
    if (!appToValidate.value) return 'repair'
    return !appToValidate.value.details.validation_status.file_exists ? 'reinstall' : 'repair'
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
                {{ t('ui.validation.issue', { name: appToValidate.details.info.name }) }}
            </div>
            <div class="mb-6 space-y-2">
                <div class="flex items-center justify-between rounded bg-surface-50 p-2">
                    <span>File Exists</span>
                    <i
                        :class="appToValidate.details.validation_status.file_exists ? 'mir-check text-green-500' : 'mir-close text-red-500'"></i>
                </div>
                <div class="flex items-center justify-between rounded bg-surface-50 p-2">
                    <span>Registry Valid</span>
                    <i
                        :class="appToValidate.details.validation_status.registry_valid ? 'mir-check text-green-500' : 'mir-close text-red-500'"></i>
                </div>
                <div class="flex items-center justify-between rounded bg-surface-50 p-2">
                    <span>Path Exists</span>
                    <i
                        :class="appToValidate.details.validation_status.path_exists ? 'mir-check text-green-500' : 'mir-close text-red-500'"></i>
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
</template>