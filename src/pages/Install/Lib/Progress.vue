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
installConfig.page = 'Install_Lib_Progress'
const { t } = useI18n()

const fullExtractPath = computed(() => {
    const base = installConfig.install_path
    const name = installConfig.name || 'Extracted-Files'
    return base && name ? `${base.replace(/\\$/, '')}\\${name}\\` : base
})

onMounted(() => {
    currentStatus.value = t('copyonly.progress.preparing')

    listen('copyonly', (event) => {
        if (event.payload === 0) {
            progressMode.value = 'indeterminate'
            currentStatus.value = t('copyonly.progress.preparing_extract')
        }
        if (event.payload === 101) {
            currentStatus.value = t('copyonly.progress.completed')
            isFinished.value = true
            canClose.value = true
        }
    })

    listen('copyonly_extract', (event) => {
        progressMode.value = 'determinate'
        extractProgress.value = event.payload as number
        currentStatus.value = t('copyonly.progress.extracting', { progress: extractProgress.value })
    })

    // Start copy-only process
    invoke('execute_command', {
        command: {
            name: 'CopyOnly',
            config: {
                zip_path: installConfig.zip_path,
                password: installConfig.archive_password,
                extract_path: installConfig.install_path,
                name: installConfig.name,
                timestamp: installConfig.timestamp,
            },
        },
    }).then((result) => {
        finalExtractPath.value = result as string
    })
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
                                <h2 class="text-lg font-medium">{{ t('copyonly.progress.title') }}</h2>
                                <p class="text-xs">{{ t('copyonly.progress.description') }}</p>
                            </div>
                        </div>
                        <div class="ml-4 flex shrink-0 select-text items-center gap-3">
                            <div
                                class="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                                <img v-if="installConfig.icon" :src="installConfig.icon" class="size-8 object-contain"
                                    alt="App Icon" />
                                <span v-else class="mir-folder_zip text-2xl"></span>
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
                                <span class="text-sm font-medium">{{ t('full_path') }}</span>
                            </div>
                            <Button severity="secondary" outlined v-tooltip.top="t('copyonly.progress.copy_path')"
                                class="h-7 w-8" icon="mir-content_copy" @click="handleCopyPath" />
                        </div>
                        <p class="select-text break-all text-sm font-medium">{{ fullExtractPath }}</p>
                    </div>
                </div>

                <div class="flex justify-end">
                    <Button v-if="canClose" @click="handleClose" :severity="isFinished ? 'success' : 'danger'"
                        class="h-8 w-24" :icon="isFinished ? 'mir-home' : 'mir-close'"
                        :label="isFinished ? t('finish') : t('close')" />
                </div>
            </Panel>
        </div>
    </div>
</template>
