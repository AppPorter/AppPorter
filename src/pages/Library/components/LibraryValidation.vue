<script setup lang="ts">
import type { InstallTypes } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import { useConfirm } from 'primevue/useconfirm'
import { inject, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const confirm = useConfirm()
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

function handleStatusClick(app: LibraryValidationProps['app']) {
    if (!app?.installed) {
        return
    }

    appToValidate.value = app

    // Skip validation for tools as they don't need registry validation
    if (app.type === 'tool') {
        return;
    }

    const validation = app.details.validation_status
    const fileExists = validation.file_exists
    const registryValid = validation.registry_valid
    const pathExists = validation.path_exists

    if (!fileExists && !registryValid) {
        confirm.require({
            message: t('ui.validation.issue', { name: app.details.info.name }) + t('ui.validation.missing_both'),
            header: t('ui.validation.validation_error'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cls.uninstall.self'),
                icon: 'mir-delete',
                severity: 'danger',
                variant: 'outlined',
            },
            acceptProps: {
                label: t('g.reinstall'),
                icon: 'mir-refresh',
            },
            accept: () => handleValidationAction('reinstall'),
            reject: () => handleValidationAction('uninstall'),
        })
    } else if (!fileExists) {
        confirm.require({
            message: t('ui.validation.issue', { name: app.details.info.name }) + t('ui.validation.missing_file'),
            header: t('ui.validation.validation_error'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cls.uninstall.self'),
                icon: 'mir-delete',
                severity: 'danger',
                variant: 'outlined',
            },
            acceptProps: {
                label: t('g.reinstall'),
                icon: 'mir-refresh',
            },
            accept: () => handleValidationAction('reinstall'),
            reject: () => handleValidationAction('uninstall'),
        })
    } else if (!registryValid) {
        confirm.require({
            message:
                t('ui.validation.issue', { name: app.details.info.name }) + t('ui.validation.missing_registry'),
            header: t('ui.validation.validation_error'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cls.uninstall.self'),
                icon: 'mir-delete',
                severity: 'danger',
                variant: 'outlined',
            },
            acceptProps: {
                label: t('g.reinstall'),
                icon: 'mir-build',
            },
            accept: () => handleValidationAction('repair'),
            reject: () => handleValidationAction('uninstall'),
        })
    } else if (!pathExists) {
        confirm.require({
            message:
                t('ui.validation.issue', { name: app.details.info.name }) + t('ui.validation.missing_path'),
            header: t('ui.validation.validation_error'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cls.uninstall.self'),
                icon: 'mir-delete',
                severity: 'danger',
                variant: 'outlined',
            },
            acceptProps: {
                label: t('g.repair'),
                icon: 'mir-build',
            },
            accept: () => handleValidationAction('repair'),
            reject: () => handleValidationAction('uninstall'),
        })
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
}

defineExpose({
    handleStatusClick
})
</script>