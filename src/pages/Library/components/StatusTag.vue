<script setup lang="ts">
import { ItemTypes } from '@/stores/library'
import Tag from 'primevue/tag'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface LibraryStatusTagProps {
    item: {
        type: ItemTypes
        installed?: boolean
        validation_status?: {
            file_exists?: boolean
            registry_valid?: boolean
            path_exists?: boolean
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

    if (data.type === 'url') {
        return {
            icon: 'mir-link',
            severity: 'warn',
            value: t('cls.install.types.url'),
        }
    }

    if (data.type === 'tool') {
        return {
            icon: 'mir-folder_copy',
            severity: 'secondary',
            value: t('cls.install.types.tool'),
        }
    }

    return {
        icon: 'mir-help',
        severity: 'secondary',
        value: t('cls.install.types.app'),
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

    const validation = data.validation_status || {}

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

    if (data.type === 'tool') {
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

    return {
        icon: 'mir-help',
        severity: 'secondary',
        value: t('ui.library.not_installed'),
    }
}
</script>

<template>
    <Tag v-if="tagType === 'type'" :value="getLibraryType(item).value" :severity="getLibraryType(item).severity"
        class="flex cursor-pointer items-center gap-1 text-xs">
        <template #icon>
            <span :class="getLibraryType(item).icon + ' align-middle leading-none text-base'" />
        </template>
    </Tag>
    <Tag v-else :value="getLibraryStatus(item).value" :severity="getLibraryStatus(item).severity"
        class="flex cursor-pointer items-center gap-1 text-xs" @click="$emit('click', item)">
        <template #icon>
            <span :class="getLibraryStatus(item).icon + ' align-middle leading-none text-base'" />
        </template>
    </Tag>
</template>
