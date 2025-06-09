<script setup lang="ts">
import { AppListStore } from '@/stores/app_list'
import { invoke } from '@tauri-apps/api/core'
import Menu from 'primevue/menu'
import { useConfirm } from 'primevue/useconfirm'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const appListStore = AppListStore()
const { t } = useI18n()
const confirm = useConfirm()

interface AppListContextMenuProps {
    selectedApp?: {
        timestamp: number
        url: string
        type: 'app' | 'lib'
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

interface AppListContextMenuEmits {
    installApp: []
    loadAppList: []
}

const props = defineProps<AppListContextMenuProps>()
const emit = defineEmits<AppListContextMenuEmits>()

const contextMenu = ref()

const menuItems = computed(() => [
    {
        label: t('install'),
        icon: 'mir-install_desktop',
        command: () => emit('installApp'),
        visible: () => props.selectedApp && !props.selectedApp.installed,
    },
    {
        label: t('open'),
        icon: 'mir-terminal',
        command: () => openApp(),
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'app',
    },
    {
        label: t('open_folder'),
        icon: 'mir-folder',
        command: () => openInstallFolder(),
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'lib',
    },
    {
        label: t('open_install_folder'),
        icon: 'mir-folder',
        command: () => openInstallFolder(),
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'app',
    },
    {
        label: t('open_registry'),
        icon: 'mir-app_registration',
        command: () => openRegistry(),
        visible: () => props.selectedApp?.installed && props.selectedApp?.type === 'app',
    },
    {
        label: props.selectedApp?.type === 'lib' ? t('delete') : (props.selectedApp?.installed ? t('uninstall') : t('remove')),
        icon: 'mir-delete',
        command: () => (props.selectedApp?.installed ? (props.selectedApp?.type === 'lib' ? confirmDelete() : confirmUninstall()) : confirmRemove()),
        visible: () => props.selectedApp !== undefined,
    },
])

async function openApp() {
    if (!props.selectedApp) return

    const targetPath = props.selectedApp.type === 'lib'
        ? props.selectedApp.details.paths.install_path
        : props.selectedApp.details.paths.full_path

    await invoke('execute_command', {
        command: {
            name: 'Open',
            path: targetPath,
        },
    })
}

async function openInstallFolder() {
    if (!props.selectedApp) return

    const targetPath = props.selectedApp.type === 'lib'
        ? props.selectedApp.details.paths.install_path
        : props.selectedApp.details.paths.full_path

    await invoke('execute_command', {
        command: {
            name: 'OpenFolder',
            path: targetPath,
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

    const app = appListStore.getAppByTimestamp(props.selectedApp.timestamp)
    if (!app) return

    await new Promise((resolve, reject) => {
        confirm.require({
            message: t('app_list.confirm_uninstall_message', {
                name: app.details.info.name,
            }),
            group: 'dialog',
            header: t('app_list.confirm_uninstall_header'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cancel'),
                severity: 'secondary',
                outlined: true,
                icon: 'mir-close',
            },
            acceptProps: {
                label: t('uninstall'),
                severity: 'danger',
                icon: 'mir-warning',
            },
            accept: async () => {
                await appListStore.executeUninstall(props.selectedApp!.timestamp)
                resolve(true)
            },
            reject: () => reject(),
        })
    })
}

async function confirmDelete() {
    if (!props.selectedApp) return

    const item = props.selectedApp.type === 'app'
        ? appListStore.getAppByTimestamp(props.selectedApp.timestamp)
        : appListStore.getLibByTimestamp(props.selectedApp.timestamp)
    if (!item) return

    await new Promise((resolve, reject) => {
        confirm.require({
            message: t('app_list.confirm_delete_message', {
                name: props.selectedApp!.details.info.name,
            }),
            group: 'dialog',
            header: t('app_list.confirm_delete_header'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cancel'),
                severity: 'secondary',
                outlined: true,
                icon: 'mir-close',
            },
            acceptProps: {
                label: t('delete'),
                severity: 'danger',
                icon: 'mir-delete',
            },
            accept: async () => {
                if (props.selectedApp!.type === 'lib') {
                    await appListStore.removeLib(props.selectedApp!.timestamp)
                } else {
                    await appListStore.removeApp(props.selectedApp!.timestamp)
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
            message: t('app_list.confirm_remove_message', {
                name: props.selectedApp!.details.info.name,
            }),
            group: 'dialog',
            header: t('app_list.confirm_remove_header'),
            icon: 'mir-warning',
            rejectProps: {
                label: t('cancel'),
                severity: 'secondary',
                outlined: true,
                icon: 'mir-close',
            },
            acceptProps: {
                label: t('remove'),
                severity: 'danger',
                icon: 'mir-delete',
            },
            accept: async () => {
                await appListStore.removeApp(props.selectedApp!.timestamp)
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
