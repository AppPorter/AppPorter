<script setup lang="ts">
import type { AppDetails, ToolDetails } from '@/stores/library'
import { type AppTypes } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import Menu from 'primevue/menu'
import { computed, inject, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const triggerUninstall = inject('triggerUninstall') as (apptype: AppTypes, timestamp: number) => Promise<void>

interface LibraryContextMenuProps {
    selectedApp?:
    | {
        timestamp: number
        url: string
        type: 'app'
        installed: boolean
        details: AppDetails
    }
    | {
        timestamp: number
        url: string
        type: 'tool'
        installed: boolean
        details: ToolDetails
    }
}

interface LibraryContextMenuEmits {
    installApp: []
    loadLibrary: []
}

const props = defineProps<LibraryContextMenuProps>()
const emit = defineEmits<LibraryContextMenuEmits>()

const contextMenu = ref()

const menuItems = computed(() => [
    {
        label: t('cls.install.self'),
        icon: 'mir-install_desktop',
        command: () => emit('installApp'),
        visible: () => props.selectedApp && !props.selectedApp.installed,
    },
    {
        label: t('g.open'),
        icon: 'mir-terminal',
        command: () => openApp(),
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'app',
    },
    {
        label: t('ui.library.open_install_folder'),
        icon: 'mir-folder',
        command: () => openInstallFolder(),
        visible: () => props.selectedApp?.installed,
    },
    {
        label: t('ui.library.open_registry'),
        icon: 'mir-app_registration',
        command: () => openRegistry(),
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'app' && props.selectedApp.details.config.create_registry_key,
    },
    {
        label: props.selectedApp?.type === 'tool' ? t('g.delete') : (props.selectedApp?.installed ? t('cls.uninstall.self') : t('g.remove')),
        icon: 'mir-delete',
        command: () => triggerUninstall(props.selectedApp!.type, props.selectedApp!.timestamp),
        visible: () => props.selectedApp !== undefined,
    },
])

async function openApp() {
    if (!props.selectedApp) return
    if (props.selectedApp.type === 'app') {
        await invoke('execute_command', {
            command: {
                name: 'OpenApp',
                path: props.selectedApp.details.paths.full_path,
            },
        })
    }
}

async function openInstallFolder() {
    if (!props.selectedApp) return
    await invoke('execute_command', {
        command: {
            name: 'OpenFolder',
            path: props.selectedApp.details.paths.install_path,
        },
    })
}

async function openRegistry() {
    if (!props.selectedApp) return
    if (props.selectedApp.type === 'app') {
        await invoke('execute_command', {
            command: {
                name: 'OpenRegistry',
                app_name: props.selectedApp.details.info.name,
                current_user_only: props.selectedApp.details.config.current_user_only,
            },
        })
    }
}

function show(event: Event) {
    contextMenu.value?.show(event)
}

defineExpose({
    show
})
</script>

<template>
    <Menu ref="contextMenu" :model="menuItems" :popup="true" />
</template>
