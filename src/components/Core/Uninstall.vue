<script setup lang="ts">
import { type InstallTypes, LibraryStore } from '@/stores/library'
import { useConfirm } from 'primevue/useconfirm'
import { useI18n } from 'vue-i18n'

const confirm = useConfirm()
const { t } = useI18n()
const libraryStore = LibraryStore()

const confirmAndUninstall = async (apptype: InstallTypes, timestamp: number): Promise<void> => {
    const app = apptype === 'app' ? libraryStore.getAppByTimestamp(timestamp) : libraryStore.getToolByTimestamp(timestamp)
    if (!app) return

    return new Promise<void>((resolve, reject) => {
        let message: string
        let header: string
        let acceptLabel: string
        let action: () => Promise<void>

        if (app.installed) {
            switch (apptype) {
                case 'tool':
                    message = t('ui.library.confirm_delete_message', {
                        name: app.details.name,
                    })
                    header = t('ui.library.confirm_delete_header')
                    acceptLabel = t('g.delete')
                    action = () => libraryStore.executeUninstall(apptype, timestamp)
                    break
                case 'app':
                    message = t('ui.library.confirm_uninstall_message', {
                        name: app.details.info.name,
                    })
                    header = t('ui.library.confirm_uninstall_header', {
                        name: app.details.info.name,
                    })
                    acceptLabel = t('cls.uninstall.self')
                    action = () => libraryStore.executeUninstall(apptype, timestamp)
                    break
            }
        } else {
            header = t('ui.library.confirm_remove_header')
            acceptLabel = t('g.remove')

            switch (apptype) {
                case 'tool':
                    message = t('ui.library.confirm_remove_message', {
                        name: app.details.name,
                    })
                    action = () => libraryStore.removeTool(timestamp)
                    break
                case 'app':
                    message = t('ui.library.confirm_remove_message', {
                        name: app.details.info.name,
                    })
                    action = () => libraryStore.removeApp(timestamp)
                    break
            }
        }

        confirm.require({
            message,
            group: 'dialog',
            header,
            icon: 'mir-warning',
            rejectProps: {
                label: t('g.cancel'),
                severity: 'secondary',
                outlined: true,
                icon: 'mir-close',
            },
            acceptProps: {
                label: acceptLabel,
                severity: 'danger',
                icon: apptype === 'tool' || !app.installed ? 'mir-delete' : 'mir-warning',
            },
            accept: async () => {
                await action()
                resolve()
            },
            reject: () => reject(),
        })
    })
}
defineExpose({ confirmAndUninstall })
</script>