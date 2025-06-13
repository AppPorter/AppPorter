<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { LibraryStore } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useConfirm } from 'primevue/useconfirm'
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const confirm = useConfirm()
const { t } = useI18n()
const libraryStore = LibraryStore()
const installConfig = InstallConfigStore()

// Setup event listeners after component is mounted
onMounted(async () => {
    await listen('preview', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }

        installConfig.temp.zip_path = payload.zip_path
        installConfig.temp.timestamp = payload.timestamp

        installConfig.show_preview_drawer = true
    })

    await listen('preview_url', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number; url: string }

        installConfig.temp.zip_path = payload.zip_path
        installConfig.temp.timestamp = payload.timestamp
        installConfig.temp.url = payload.url

        installConfig.show_preview_drawer = true
    })

    await listen('uninstall_app', async (event) => {
        goTo('/Library')
        await libraryStore.loadLibrary()
        const app = libraryStore.getAppByTimestamp(event.payload as number)
        if (!app) return
        await new Promise((resolve, reject) => {
            confirm.require({
                message: t('ui.uninstall.confirm.msg', {
                    name: app.details.info.name,
                }),
                group: 'dialog',
                header: t('ui.uninstall.confirm.header'),
                icon: 'mir-warning',
                rejectProps: {
                    label: t('g.cancel'),
                    severity: 'secondary',
                    outlined: true,
                    icon: 'mir-close',
                },
                acceptProps: {
                    label: t('cls.uninstall.self'),
                    severity: 'danger',
                    icon: 'mir-warning',
                },
                accept: async () => {
                    await libraryStore.executeUninstall(event.payload as number)
                    resolve(true)
                },
                reject: () => reject(),
            })
        })
    })

    // Execute initial command
    invoke('execute_command', {
        command: {
            name: 'Cli',
        },
    })
})
</script>