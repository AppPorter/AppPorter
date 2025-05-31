<script setup lang="ts">
import { goTo } from '@/router'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useConfirm } from 'primevue/useconfirm'
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { AppListStore } from '@/stores/app_list'
import { InstallConfigStore } from '@/stores/install_config'

const confirm = useConfirm()
const { t } = useI18n()
const appListStore = AppListStore()
const installConfig = InstallConfigStore()

// Setup event listeners after component is mounted
onMounted(async () => {
    await listen('install', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }
        installConfig.zip_path = payload[0]
        installConfig.timestamp = payload[1]
        goTo('/Installation/App/Config')
    })

    await listen('install_app', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }
        installConfig.zip_path = payload[0]
        installConfig.timestamp = payload[1]
        goTo('/Installation/App/Config')
    })

    await listen('install_lib', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }
        installConfig.zip_path = payload[0]
        installConfig.timestamp = payload[1]
        goTo('/Installation/Lib/Config')
    })

    await listen('uninstall_app', async (event) => {
        goTo('/AppList')
        await appListStore.loadAppList()
        const app = appListStore.getAppByTimestamp(event.payload as number)
        if (!app) return
        await new Promise((resolve, reject) => {
            confirm.require({
                message: t('uninstall.confirm.message', {
                    name: app.details.info.name,
                }),
                group: 'dialog',
                header: t('uninstall.confirm.header'),
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
                    await appListStore.executeUninstall(event.payload as number)
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