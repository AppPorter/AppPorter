<script setup lang="ts">
import Tag from 'primevue/tag'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface AppStatusTagProps {
    item: {
        type: 'app' | 'lib'
        installed: boolean
        details: {
            validation_status: {
                file_exists: boolean
                registry_valid: boolean
                path_exists: boolean
            }
        }
    }
}

interface AppStatusTagEmits {
    click: [item: AppStatusTagProps['item']]
}

defineProps<AppStatusTagProps>()
defineEmits<AppStatusTagEmits>()

function getAppStatus(data: AppStatusTagProps['item']) {
    if (!data.installed) {
        return {
            icon: 'mir-cloud_download',
            severity: 'secondary',
            value: t('app_list.not_installed'),
        }
    }

    // Handle libs (library type)
    if (data.type === 'lib') {
        return {
            icon: 'mir-folder_copy',
            severity: 'info',
            value: t('app_list.library'),
        }
    }

    const validation = data.details.validation_status

    // For apps, check both file existence and registry validity
    if (data.type === 'app') {
        const isValid = validation.file_exists && validation.registry_valid && validation.path_exists

        if (isValid) {
            return {
                icon: 'mir-check',
                value: t('app_list.installed'),
            }
        }

        return {
            icon: 'mir-error',
            severity: 'warn',
            value: t('app_list.validation_error'),
        }
    }

    // For libs, only check file and path existence
    const isLibValid = validation.file_exists && validation.path_exists

    if (isLibValid) {
        return {
            icon: 'mir-check',
            value: t('app_list.installed'),
        }
    }

    return {
        icon: 'mir-error',
        severity: 'warn',
        value: t('app_list.validation_error'),
    }
}
</script>

<template>
    <Tag :value="getAppStatus(item).value" :severity="getAppStatus(item).severity" :icon="getAppStatus(item).icon"
        class="cursor-pointer text-center text-xs" @click="$emit('click', item)" />
</template>
