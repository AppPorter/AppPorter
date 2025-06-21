<script setup lang="ts">
import { LibraryStore } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import Menu from 'primevue/menu'
import { useConfirm } from 'primevue/useconfirm'
import { computed, inject, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const libraryStore = LibraryStore()
const { t } = useI18n()
const confirm = useConfirm()
const triggerUninstall = inject('triggerUninstall') as (timestamp: number) => Promise<void>

interface LibraryContextMenuProps {
    selectedApp?: {
        timestamp: number
        url: string
        type: 'app' | 'tool'
        installed: boolean
        details: {
            info: {
                name: string
                icon: string
                publisher: string
                version: string
            }
            config: {
                current_user_only: boolean
            }
            paths: {
                install_path: string
                full_path: string
            }
        }
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
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'app',
    },
    {
        label: props.selectedApp?.type === 'tool' ? t('g.delete') : (props.selectedApp?.installed ? t('cls.uninstall.self') : t('g.remove')),
        icon: 'mir-delete',
        command: () => (props.selectedApp?.installed ? (props.selectedApp?.type === 'tool' ? confirmDelete() : confirmUninstall()) : confirmRemove()),
        visible: () => props.selectedApp !== undefined,
    },
])

async function openApp() {
    if (!props.selectedApp) return

    const targetPath = props.selectedApp.type === 'tool'
        ? props.selectedApp.details.paths.install_path
        : props.selectedApp.details.paths.full_path

    await invoke('execute_command', {
        command: {
            name: 'OpenApp',
            path: targetPath,
        },
    })
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

    await invoke('execute_command', {
        command: {
            name: 'OpenRegistry',
            app_name: props.selectedApp.details.info.name,
            current_user_only: props.selectedApp.details.config.current_user_only,
        },
    })
}

async function confirmUninstall() {
    if (!props.selectedApp) return
    await triggerUninstall(props.selectedApp.timestamp)
}

async function confirmDelete() {
    if (!props.selectedApp) return

    const item = props.selectedApp.type === 'app'
        ? libraryStore.getAppByTimestamp(props.selectedApp.timestamp)
        : libraryStore.getToolByTimestamp(props.selectedApp.timestamp)
    if (!item) return

    await new Promise((resolve, reject) => {
        confirm.require({
            message: t('ui.library.confirm_delete_message', {
                name: props.selectedApp!.details.info.name,
            }),
            group: 'dialog',
            header: t('ui.library.confirm_delete_header'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('g.cancel'),
                severity: 'secondary',
                outlined: true,
                icon: 'mir-close',
            },
            acceptProps: {
                label: t('g.delete'),
                severity: 'danger',
                icon: 'mir-delete',
            },
            accept: async () => {
                if (props.selectedApp!.type === 'tool') {
                    await libraryStore.removeTool(props.selectedApp!.timestamp)
                } else {
                    await libraryStore.removeApp(props.selectedApp!.timestamp)
                }
                resolve(true)
            },
            reject: () => reject(),
        })
    })
}

async function confirmRemove() {
    if (!props.selectedApp) return

    await new Promise((resolve, reject) => {
        confirm.require({
            message: t('ui.library.confirm_remove_message', {
                name: props.selectedApp!.details.info.name,
            }),
            group: 'dialog',
            header: t('ui.library.confirm_remove_header'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('g.cancel'),
                severity: 'secondary',
                outlined: true,
                icon: 'mir-close',
            },
            acceptProps: {
                label: t('g.remove'),
                severity: 'danger',
                icon: 'mir-delete',
            },
            accept: async () => {
                await libraryStore.removeApp(props.selectedApp!.timestamp)
                resolve(true)
            },
            reject: () => reject(),
        })
    })
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
