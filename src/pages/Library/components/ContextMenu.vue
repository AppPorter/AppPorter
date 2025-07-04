<script setup lang="ts">
import type { AppDetails } from '#/AppDetails'
import type { ToolDetails } from '#/ToolDetails'
import { exec } from '@/exec'
import { generalStore, libraryStore } from '@/main'
import Menu from 'primevue/menu'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface LibraryContextMenuProps {
    selectedApp?:
    | {
        id: string
        url: string
        type: 'app'
        installed: boolean
        details: AppDetails
    }
    | {
        id: string
        url: string
        type: 'tool'
        installed: boolean
        details: ToolDetails
    }
    | {
        id: string
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
            }
            install_path: string
            full_path: string
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
    if (props.selectedApp?.type === 'url') {
        return [
            {
                label: t('cls.install.self'),
                icon: 'mir-install_desktop',
                command: () => previewUrl(),
                visible: true,
            },
            {
                label: t('g.remove'),
                icon: 'mir-delete',
                command: () => removeUrl(),
                visible: true,
            },
        ]
    }

    const items = [
        {
            label: t('cls.install.self'),
            icon: 'mir-install_desktop',
            command: () => previewUrl(),
            visible: props.selectedApp ? !props.selectedApp.installed : false,
        },
        {
            label: t('g.open'),
            icon: 'mir-terminal',
            command: () => openApp(),
            visible: props.selectedApp ? (props.selectedApp.installed && props.selectedApp.type === 'app') : false,
        },
        {
            label: t('ui.library.open_install_folder'),
            icon: 'mir-folder',
            command: () => openInstallFolder(),
            visible: props.selectedApp ? props.selectedApp.installed : false,
        },
        {
            label: t('ui.library.open_registry'),
            icon: 'mir-app_registration',
            command: () => openRegistry(),
            visible: props.selectedApp ? (props.selectedApp.installed && props.selectedApp.type === 'app' && props.selectedApp.details.config.create_registry_key) : false,
        },
        {
            label: t('g.modify'),
            icon: 'mir-edit',
            command: () => handleModify(),
            visible: props.selectedApp ? (props.selectedApp.installed && (props.selectedApp.type === 'app' || props.selectedApp.type === 'tool')) : false,
        },
        {
            label: props.selectedApp?.type === 'tool' ? t('g.delete') : (props.selectedApp?.installed ? t('cls.uninstall.self') : t('g.remove')),
            icon: 'mir-delete',
            command: () => generalStore.drawer.uninstall = [true, props.selectedApp!.id],
            visible: !!props.selectedApp,
        },
    ]
    return items
})
function handleModify() {
    if (!props.selectedApp) return
    if (props.selectedApp.type === 'app') {
        generalStore.drawer.app_modify = [true, props.selectedApp.id]
    } else if (props.selectedApp.type === 'tool') {
        generalStore.drawer.tool_modify = [true, props.selectedApp.id]
    }
}

async function previewUrl() {
    if (!props.selectedApp) return

    await exec('PreviewUrl', {
        url: props.selectedApp.url,
    })
}

async function openApp() {
    if (!props.selectedApp) return
    if (props.selectedApp.type === 'app') {
        await exec('OpenApp', {
            path: props.selectedApp.details.full_path,
        })
    }
}

async function openInstallFolder() {
    if (!props.selectedApp) return
    await exec('OpenFolder', {
        path: props.selectedApp.details.install_path,
    })
}

async function openRegistry() {
    if (!props.selectedApp) return
    if (props.selectedApp.type === 'app') {
        await exec('OpenRegistry', {
            app_name: props.selectedApp.details.info.name,
            current_user_only: props.selectedApp.details.current_user_only,
        })
    }
}

async function removeUrl() {
    if (!props.selectedApp || props.selectedApp.type !== 'url') return

    libraryStore.urls = libraryStore.urls.filter(urlObj => urlObj.id !== props.selectedApp!.id)
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
