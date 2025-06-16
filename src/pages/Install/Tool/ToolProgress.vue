<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import Button from 'primevue/button'
import Panel from 'primevue/panel'
import ProgressBar from 'primevue/progressbar'
import Tooltip from 'primevue/tooltip'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')
const extractProgress = ref(0)
const isFinished = ref(false)
const currentStatus = ref('')
const canClose = ref(false)
const finalExtractPath = ref('')

const installConfig = InstallConfigStore()
installConfig.page = 'Install_Tool_Progress'
const { t } = useI18n()

const fullExtractPath = computed(() => {
    const base = installConfig.tool_details.paths.install_path
    const name = installConfig.tool_details.name || 'Extracted-Files'
    return base && name ? `${base.replace(/\\$/, '')}\\${name}\\` : base
})

onMounted(async () => {
    // Initial install setup
    currentStatus.value = t('ui.install.progress.preparing')

    // Setup event listener for tool install progress
    const toolInstallUnlisten = await listen('tool_install_progress', (event) => {
        const payload = event.payload as number
        if (payload === 0) {
            progressMode.value = 'indeterminate'
            currentStatus.value = t('ui.install.progress.preparing_extract')
        } else if (payload === 101) {
            extractProgress.value = 100
            currentStatus.value = t('ui.install.progress.completed')
            isFinished.value = true
            canClose.value = true
        } else if (payload > 0 && payload <= 100) {
            progressMode.value = 'determinate'
            extractProgress.value = payload
            currentStatus.value = t('ui.install.progress.extracting', { progress: extractProgress.value })
        }
    })

    // Start tool install process
    try {
        const result = await invoke('execute_command', {
            command: {
                name: 'InstallTool',
                config: {
                    zip_path: installConfig.zip_path,
                    password: installConfig.archive_password,
                    extract_path: installConfig.tool_details.paths.install_path,
                    name: installConfig.tool_details.name,
                    timestamp: installConfig.timestamp,
                },
            },
        })
        finalExtractPath.value = result as string
    } catch (error) {
        console.error('Tool install failed:', error)
        currentStatus.value = t('ui.install.progress.failed')
        canClose.value = true
    }

    // Cleanup listeners on unmount
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

// Add copy method for template usage
function handleCopyPath() {
    // Copy full extract path to clipboard
    navigator.clipboard.writeText(fullExtractPath.value)
}
</script>

<template>
    <div class="flex size-full flex-col">
        <div class="flex-1 overflow-auto p-1.5 pb-6">
            <Panel class="mx-auto w-full max-w-5xl shadow-sm">
                <template #header>
                    <div class="flex w-full min-w-0 items-center justify-between py-1">
                        <div class="flex min-w-0 shrink items-center gap-2">
                            <div class="shrink-0 rounded-md p-1.5">
                                <span class="mir-text-xl" :class="[
                                    isFinished ? 'mir-task_alt' : 'mir-folder_copy',
                                    isFinished
                                        ? 'text-green-600 dark:text-green-400'
                                        : 'text-primary-600 dark:text-primary-400',
                                ]"></span>
                            </div>
                            <div class="min-w-0 shrink">
                                <h2 class="text-lg font-medium">{{ t('ui.install.progress.title') }}</h2>
                                <p class="text-xs">{{ t('ui.install.progress.description') }}</p>
                            </div>
                        </div>
                        <div class="ml-4 flex shrink-0 select-text items-center gap-3">
                            <div
                                class="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                                <span class="mir-folder_zip text-2xl"></span>
                            </div>
                        </div>
                    </div>
                </template>

                <div class="space-y-4">
                    <div class="space-y-2">
                        <p :class="[isFinished ? 'text-green-600 dark:text-green-400' : '']" class="text-sm">
                            {{ currentStatus }}
                        </p>
                        <ProgressBar :mode="progressMode" :value="extractProgress" class="h-1.5" />
                    </div>

                    <div v-if="isFinished"
                        class="rounded-lg border border-slate-200 p-4 shadow-sm dark:border-zinc-600">
                        <div class="flex w-full items-center justify-between py-1">
                            <div class="flex items-center gap-2">
                                <span class="mir-terminal"></span>
                                <span class="text-sm font-medium">{{ t('cls.install.config.full_path') }}</span>
                            </div>
                            <Button severity="secondary" outlined v-tooltip.top="t('ui.install.progress.install_path')"
                                class="h-7 w-8" icon="mir-content_copy" @click="handleCopyPath" />
                        </div>
                        <p class="select-text break-all text-sm font-medium">{{ fullExtractPath }}</p>
                    </div>
                </div>

                <div class="flex justify-end">
                    <Button v-if="canClose" @click="handleClose" :severity="isFinished ? 'success' : 'danger'"
                        class="h-8 w-24" :icon="isFinished ? 'mir-home' : 'mir-close'"
                        :label="isFinished ? t('g.finish') : t('g.close')" />
                </div>
            </Panel>
        </div>
    </div>
</template>
