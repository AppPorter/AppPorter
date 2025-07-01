<script setup lang="ts">
import { goTo } from '@/router'
import { exec } from '@/exec'
import { listen } from '@tauri-apps/api/event'
import Button from 'primevue/button'
import Panel from 'primevue/panel'
import ProgressBar from 'primevue/progressbar'
import Tooltip from 'primevue/tooltip'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { generalStore, installConfig } from '@/main'

const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const extractProgress = ref(0)
const isFinished = ref(false)
const currentStatus = ref('')
const canClose = ref(false)
const installPathCopied = ref(false)

const installPath = ref('')

generalStore.page = 'Install_Tool_Progress'
const { t } = useI18n()

const handleOpenInstallFolder = async () => {
    await exec('OpenFolder', {
        path: installPath.value
    })
}

onMounted(async () => {
    currentStatus.value = t('ui.install.progress.preparing')

    const toolInstallUnlisten = await listen('tool_install_progress', (event) => {
        const payload = event.payload as number
        if (payload === 0) {
            progressMode.value = 'indeterminate'
            currentStatus.value = t('ui.install.progress.preparing_extract')
        } else if (payload === 101) {
            progressMode.value = 'determinate'
            extractProgress.value = 100
            currentStatus.value = ''
            isFinished.value = true
            canClose.value = true
        } else if (payload > 0 && payload <= 100) {
            progressMode.value = 'determinate'
            extractProgress.value = payload
            currentStatus.value = t('ui.install.progress.extracting', { progress: extractProgress.value })
        }
    })

    try {
        const result = await exec<string>('InstallTool', {
            config: {
                tool: {
                    timestamp: installConfig.timestamp,
                    installed: false,
                    url: installConfig.url,
                    archive_password: installConfig.archive_password,
                    details: installConfig.tool_details,
                    validation_status: {
                        file_exists: false,
                        path_exists: false
                    }
                },
                archive_path_dir: installConfig.tool_details.add_to_path[1],
                zip_path: installConfig.zip_path,
            }
        })
        installPath.value = result
    } catch (error) {
        globalThis.$errorHandler.showError(error)
        currentStatus.value = t('ui.install.progress.failed')
        canClose.value = true
    }

    return () => {
        toolInstallUnlisten()
    }
})

const handleClose = () => {
    goTo('/Home')
}

defineOptions({
    directives: { tooltip: Tooltip },
})

async function handleCopyInstallPath() {
    await navigator.clipboard.writeText(installPath.value)
    installPathCopied.value = true
    setTimeout(() => {
        installPathCopied.value = false
    }, 2000)
}
</script>

<template>
    <div class="flex size-full flex-col overflow-hidden">
        <div class="flex-1 overflow-auto">
            <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
                <div class="min-w-72 flex-1 space-y-2">
                    <Panel class="shadow-sm">
                        <template #header>
                            <div class="flex items-center gap-1.5">
                                <span class="mir-text-lg" :class="[
                                    isFinished ? 'mir-task_alt' : 'mir-folder_copy',
                                    'text-primary-600 dark:text-primary-400',
                                ]"></span>
                                <h2 class="text-base font-medium">
                                    {{ t('ui.install.progress.title') }}
                                </h2>
                            </div>
                        </template>

                        <div class="space-y-4 p-2">
                            <div class="flex items-center gap-3">
                                <div
                                    class="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                                    <span class="mir-folder_zip text-2xl"></span>
                                </div>
                                <div class="min-w-0 flex-1">
                                    <h3 class="text-base font-medium leading-none">
                                        {{ installConfig.tool_details.name || 'Extracted Files' }}
                                    </h3>
                                    <p class="mt-1 text-xs">
                                        {{ t('ui.install.tool_extraction') }}
                                    </p>
                                </div>
                            </div>

                            <div class="space-y-2">
                                <p class="text-sm">
                                    {{ currentStatus }}
                                </p>
                                <ProgressBar :mode="progressMode" :value="extractProgress" class="h-1.5" />
                            </div>

                            <div class="space-y-2">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <span class="mir-folder text-sm"></span>
                                        <span class="text-sm font-medium">{{ t('cls.install.config.install_path')
                                        }}</span>
                                    </div>
                                    <Button outlined v-tooltip.top="t('ui.install.progress.copy_path')" class="h-7 w-8"
                                        :icon="installPathCopied ? 'mir-check' : 'mir-content_copy'"
                                        :severity="installPathCopied ? 'success' : 'secondary'"
                                        @click="handleCopyInstallPath" />
                                </div>
                                <p
                                    class="select-text break-all rounded bg-surface-50 p-2 text-sm font-medium dark:bg-surface-800">
                                    {{ installPath }}
                                </p>
                            </div>
                        </div>
                    </Panel>
                </div>
            </div>
        </div>

        <div class="flex items-center justify-between px-4 py-3">
            <div v-if="isFinished" class="flex items-center gap-2">
                <Button @click="handleOpenInstallFolder" severity="secondary" outlined class="h-8" icon="mir-folder"
                    :label="t('ui.library.open_install_folder')" />
            </div>

            <div class="flex items-center">
                <Button v-if="canClose" @click="handleClose" :severity="isFinished ? 'primary' : 'danger'"
                    class="h-8 w-24" :icon="isFinished ? 'mir-home' : 'mir-close'"
                    :label="isFinished ? t('g.finish') : t('g.close')" />
            </div>
        </div>
    </div>
</template>
