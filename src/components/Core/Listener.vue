<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { LibraryStore } from '@/stores/library'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { onMounted } from 'vue'

const libraryStore = LibraryStore()
const installConfig = InstallConfigStore()

// Setup event listeners after component is mounted
onMounted(async () => {
    await listen('preview', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number }

        installConfig.temp.zip_path = payload[0]
        installConfig.temp.timestamp = payload[1]

        installConfig.show_preview_drawer = true
    })

    await listen('preview_url', (event) => {
        const payload = event.payload as { zip_path: string; timestamp: number; url: string }

        installConfig.temp.zip_path = payload[0]
        installConfig.temp.timestamp = payload[1]
        installConfig.temp.url = payload[2]

        installConfig.show_preview_drawer = true
    })

    await listen('uninstall_app', async (event) => {
        goTo('/Library')
        await libraryStore.loadLibrary()
        await libraryStore.confirmAndExecuteUninstall(event.payload as number)
    })

    // Execute initial command
    invoke('execute_command', {
        command: {
            name: 'Cli',
        },
    })
})
</script>