<script setup lang="ts">
import { exec } from '@/exec'
import { generalStore, installConfig, libraryStore, window as tauriWindow, triggerUninstall } from '@/main'
import { goTo } from '@/router'
import { listen } from '@tauri-apps/api/event'
import { onMounted } from 'vue'

function showWindow() {
    tauriWindow.show()
    tauriWindow.unminimize()
    tauriWindow.setFocus()
}

onMounted(async () => {
    await listen('preview', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }

        installConfig.temp.zip_path = payload[0]
        installConfig.temp.id = payload[1]

        generalStore.drawer.preview = true
        showWindow()
    })

    await listen('preview_url', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number; url: string }

        installConfig.temp.zip_path = payload[0]
        installConfig.temp.id = payload[1]
        installConfig.temp.url = payload[2]

        generalStore.drawer.preview = true
        showWindow()
    })

    await listen('uninstall_app', async (event) => {
        goTo('/Library')
        await libraryStore.loadLibrary()
        await triggerUninstall('app', event.payload as number)
        showWindow()
    })

    exec('Cli')
})
</script>