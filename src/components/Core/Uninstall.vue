<script setup lang="ts">
import { LibraryStore } from '@/stores/library'
import { useConfirm } from 'primevue/useconfirm'
import { useI18n } from 'vue-i18n'

const confirm = useConfirm()
const { t } = useI18n()
const libraryStore = LibraryStore()

// The actual implementation of confirmAndExecuteUninstall
const confirmAndExecuteUninstall = async (timestamp: number): Promise<void> => {
    const app = libraryStore.getAppByTimestamp(timestamp)
    if (!app) return

    return new Promise<void>((resolve, reject) => {
        confirm.require({
            message: t('ui.library.confirm_uninstall_message', {
                name: app.details.info.name,
            }),
            group: 'dialog',
            header: t('ui.library.confirm_uninstall_header', {
                name: app.details.info.name,
            }),
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
                await libraryStore.executeUninstall(timestamp)
                resolve()
            },
            reject: () => reject(),
        })
    })
}
defineExpose({ confirmAndExecuteUninstall })
</script>