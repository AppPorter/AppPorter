<script setup lang="ts">
import type { AppDetails, ToolDetails } from '@/stores/library'
import { type InstallTypes, LibraryStore } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import Menu from 'primevue/menu'
import { computed, inject, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const triggerUninstall = inject('triggerUninstall') as (apptype: InstallTypes, timestamp: number) => Promise<void>
const libraryStore = LibraryStore()

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
    | {
        timestamp: number
        url: string
        type: 'url'
        installed: false
        details: {
            info: {
                name: string
                icon: string
                publisher: string
                version: string
            }
            config: {
                current_user_only: boolean
                create_desktop_shortcut: boolean
                create_start_menu_shortcut: boolean
                create_registry_key: boolean
                archive_exe_path: string
            }
            paths: {
                parent_install_path: string
                install_path: string
                full_path: string
            }
            validation_status: {
                file_exists: boolean
                registry_valid: boolean
                path_exists: boolean
            }
        }
    }
}

interface LibraryContextMenuEmits {
    loadLibrary: []
}

const props = defineProps<LibraryContextMenuProps>()
const emit = defineEmits<LibraryContextMenuEmits>()

const contextMenu = ref()

const menuItems = computed(() => {
    // Special menu for URL type items - only install and remove
    if (props.selectedApp?.type === 'url') {
        return [
            {
                label: t('cls.install.self'),
                icon: 'mir-install_desktop',
                command: () => previewUrl(),
                visible: () => true,
            },
            {
                label: t('g.remove'),
                icon: 'mir-delete',
                command: () => removeUrl(),
                visible: () => true,
            },
        ]
    }

    // Regular menu for app and tool types
    return [
        {
            label: t('cls.install.self'),
            icon: 'mir-install_desktop',
            command: () => previewUrl(),
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
    ]
})

async function previewUrl() {
    if (!props.selectedApp) return

    await invoke('execute_command', {
        command: {
            name: 'PreviewUrl',
            url: props.selectedApp.url,
        },
    })
}

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

async function removeUrl() {
    if (!props.selectedApp || props.selectedApp.type !== 'url') return

    // Remove the URL from the subscribed URLs
    libraryStore.urls = libraryStore.urls.filter(urlObj => urlObj.timestamp !== props.selectedApp!.timestamp)
    await libraryStore.saveLibrary()
    emit('loadLibrary')
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
