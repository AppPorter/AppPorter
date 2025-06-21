<script setup lang="ts">
import { AppTypes } from '@/stores/library'
import Tag from 'primevue/tag'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface LibraryStatusTagProps {
    item: {
        type: AppTypes
        installed: boolean
        details: {
            validation_status: {
                file_exists: boolean
                registry_valid: boolean
                path_exists: boolean
            }
        }
    }
    tagType: 'type' | 'status'
}

interface LibraryStatusTagEmits {
    click: [item: LibraryStatusTagProps['item']]
}

defineProps<LibraryStatusTagProps>()
defineEmits<LibraryStatusTagEmits>()

function getLibraryType(data: LibraryStatusTagProps['item']) {
    if (data.type === 'app') {
        return {
            icon: 'mir-apps',
            severity: 'info',
            value: t('cls.install.types.app'),
        }
    }

    return {
        icon: 'mir-folder_copy',
        severity: 'secondary',
        value: t('cls.install.types.tool'),
    }
}

function getLibraryStatus(data: LibraryStatusTagProps['item']) {
    if (!data.installed) {
        return {
            icon: 'mir-cloud_download',
            severity: 'secondary',
            value: t('ui.library.not_installed'),
        }
    }

    const validation = data.details.validation_status

    // For apps, check both file existence and registry validity
    if (data.type === 'app') {
        const isValid = validation.file_exists && validation.registry_valid && validation.path_exists

        if (isValid) {
            return {
                icon: 'mir-check',
                severity: 'success',
                value: t('ui.library.installed'),
            }
        }

        return {
            icon: 'mir-error',
            severity: 'warn',
            value: t('ui.validation.validation_error'),
        }
    }

    // For tools, only check file and path existence
    const isToolValid = validation.file_exists && validation.path_exists

    if (isToolValid) {
        return {
            icon: 'mir-check',
            severity: 'success',
            value: t('ui.library.installed'),
        }
    }

    return {
        icon: 'mir-error',
        severity: 'warn',
        value: t('ui.validation.validation_error'),
    }
}
</script>

<template>
    <Tag v-if="tagType === 'type'" :value="getLibraryType(item).value" :severity="getLibraryType(item).severity"
        :icon="getLibraryType(item).icon" class="cursor-pointer text-center text-xs" @click="$emit('click', item)" />
    <Tag v-else :value="getLibraryStatus(item).value" :severity="getLibraryStatus(item).severity"
        :icon="getLibraryStatus(item).icon" class="cursor-pointer text-center text-xs" @click="$emit('click', item)" />
</template>
