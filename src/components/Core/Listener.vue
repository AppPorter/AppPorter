<script setup lang="ts">
import { window as tauriWindow } from '@/main'
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { type InstallTypes, LibraryStore } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { inject, onMounted } from 'vue'

const libraryStore = LibraryStore()
const installConfig = InstallConfigStore()
const triggerUninstall = inject('triggerUninstall') as (apptype: InstallTypes, timestamp: number) => Promise<void>

function showWindow() {
    tauriWindow.show()
    tauriWindow.unminimize()
    tauriWindow.setFocus()
}

onMounted(async () => {
    await listen('preview', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }

        installConfig.temp.zip_path = payload[0]
        installConfig.temp.timestamp = payload[1]

        installConfig.show_preview_drawer = true
        showWindow()
    })

    await listen('preview_url', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number; url: string }

        installConfig.temp.zip_path = payload[0]
        installConfig.temp.timestamp = payload[1]
        installConfig.temp.url = payload[2]

        installConfig.show_preview_drawer = true
        showWindow()
    })

    await listen('uninstall_app', async (event) => {
        goTo('/Library')
        await libraryStore.loadLibrary()
        await triggerUninstall('app', event.payload as number)
        showWindow()
    })

    invoke('exec', {
        cmd: {
            name: 'Cli',
        },
    })
})
</script>