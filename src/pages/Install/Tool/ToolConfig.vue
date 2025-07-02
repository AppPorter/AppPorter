<script setup lang="ts">
import DirectorySelectorDrawer from '@/components/Drawer/DirectorySelector.vue'
import { exec } from '@/exec'
import { generalStore, installConfig, settingsStore } from '@/main'
import { goTo } from '@/router'
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const { tool_install } = settingsStore
const confirm = useConfirm()
const parent_install_path = ref('')

generalStore.page = 'Install_Tool_Config'

const pathError = ref(false)
const nameError = ref(false)

const directoryDrawerVisible = ref(false)

const formatted_final_path = computed(() => {
    const parentPath = parent_install_path.value
    const toolName = installConfig.tool_details.name

    if (!parentPath || !toolName) return ''

    return `${parentPath}\\${toolName}`
})

onMounted(async () => {
    parent_install_path.value = tool_install.install_path
    installConfig.tool_details.add_to_path[0] = tool_install.add_to_path
    installConfig.tool_details.add_to_path[1] = ''

    if (installConfig.zip_path) {
        const pathParts = installConfig.zip_path.split('\\')
        const filename = pathParts[pathParts.length - 1]
        installConfig.tool_details.name = filename.replace(/\.[^/.]+$/, '')
    }
})

function handleBackClick() {
    goTo('/Home')
}

async function select_extract_path() {
    const selected = await open({
        directory: true,
        multiple: false,
    })
    if (selected) {
        parent_install_path.value = String(selected)
        pathError.value = false
    }
}

async function handleInstallClick() {
    pathError.value = false
    nameError.value = false

    nameError.value = !installConfig.tool_details.name
    pathError.value = !parent_install_path.value
    try {
        const validatedPath = (await exec<string>('ValidatePath', {
            path: parent_install_path.value,
        }))
        parent_install_path.value = validatedPath
        installConfig.tool_details.install_path = formatted_final_path.value
    } catch (error) {
        globalThis.$errorHandler.showError(error)
        pathError.value = true
    }
    if (nameError.value || pathError.value) {
        return
    }
    try {
        await exec('ValidatePath', {
            path: parent_install_path.value,
        })
        await new Promise((resolve, reject) => {
            confirm.require({
                message: t('ui.install.confirm_install', {
                    name: installConfig.tool_details.name,
                }),
                group: 'dialog',
                icon: 'mir-folder_copy',
                header: t('ui.install.start_install'),
                rejectProps: {
                    label: t('g.cancel'),
                    severity: 'secondary',
                    outlined: true,
                    icon: 'mir-close',
                },
                acceptProps: {
                    label: t('cls.install.self'),
                    icon: 'mir-navigate_next',
                },
                accept: () => resolve(true),
                reject: () => reject(),
            })
        })
    } catch (error) {
        if (error === 'Directory is not empty') {
            await new Promise((resolve, reject) => {
                confirm.require({
                    message: t('ui.install.force_install_confirm', {
                        name: installConfig.tool_details.name,
                    }),
                    group: 'dialog',
                    icon: 'mir-warning',
                    header: t('g.warning'),
                    rejectProps: {
                        label: t('g.cancel'),
                        severity: 'secondary',
                        outlined: true,
                        icon: 'mir-close',
                    },
                    acceptProps: {
                        label: t('ui.install.force_install'),
                        severity: 'danger',
                        icon: 'mir-folder_copy',
                    },
                    accept: () => resolve(true),
                    reject: () => reject(),
                })
            })
        } else {
            globalThis.$errorHandler.showError(error)
            return
        }
    }
    goTo('/Install/Tool/Progress')
}
</script>

<template>
    <div class="flex size-full flex-col overflow-hidden">
        <div class="flex-1 overflow-auto">
            <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
                <div class="min-w-72 flex-1 space-y-2">
                    <Panel class="shadow-sm">
                        <template #header>
                            <div class="flex flex-col">
                                <div class="flex items-center gap-1.5">
                                    <span class="mir-folder_copy text-lg" />
                                    <h2 class="text-base font-medium">
                                        {{ t('ui.install.tool_details') }}
                                    </h2>
                                </div>
                                <p class="ml-6 mt-0.5 text-xs">
                                    {{ t('ui.install.selected_file') }}:
                                    <span class="break-all font-medium">{{ installConfig.zip_path }}</span>
                                </p>
                            </div>
                        </template>

                        <div class="space-y-4 p-1">
                            <div class="flex items-center gap-2">
                                <label class="w-24 text-sm font-medium">
                                    {{ t('cls.app.name') }}
                                </label>
                                <div class="w-full">
                                    <InputText v-model="installConfig.tool_details.name"
                                        :placeholder="t('cls.app.name')" class="h-8 w-full text-sm"
                                        :invalid="nameError" />
                                </div>
                            </div>

                            <div class="flex items-center gap-2">
                                <label class="w-24 text-sm font-medium">{{ t('cls.install.config.install_path')
                                    }}</label>
                                <div class="w-full">
                                    <div class="flex items-center gap-2">
                                        <InputText v-model="parent_install_path" :placeholder="t('g.browse')"
                                            class="h-8 w-full text-sm" :invalid="pathError"
                                            @input="pathError = false" />
                                        <Button class="h-8 w-36" severity="secondary" @click="select_extract_path"
                                            icon="mir-folder_open" :label="t('g.browse')" />
                                    </div>

                                    <div v-if="formatted_final_path" class="mt-1 text-xs text-gray-500">
                                        {{ t('ui.install.final_path') }}: {{ formatted_final_path }}
                                    </div>
                                </div>
                            </div>

                            <div class="mt-2 flex items-start gap-2">
                                <label class="mt-1 w-24 text-sm font-medium">{{ t('ui.install.environment') }}</label>
                                <div class="w-full">
                                    <div class="flex-1 space-y-1 rounded-lg p-1.5">
                                        <div class="flex flex-col gap-1">
                                            <div class="flex items-center gap-2">
                                                <Checkbox v-model="installConfig.tool_details.add_to_path[0]"
                                                    :binary="true" inputId="add_to_path" />
                                                <label for="add_to_path" class="text-sm">{{
                                                    t('cls.install.shortcuts.add_to_path')
                                                    }}</label>
                                            </div>
                                            <div v-if="installConfig.tool_details.add_to_path[0]" class="ml-6 mt-1">
                                                <div class="flex gap-2">
                                                    <InputText v-model="installConfig.tool_details.add_to_path[1]"
                                                        :placeholder="t('ui.select_placeholder.path_directory')"
                                                        class="h-8 w-full text-sm" />
                                                    <Button class="h-8 w-36" severity="secondary"
                                                        @click="directoryDrawerVisible = true" icon="mir-folder_open"
                                                        :label="t('g.browse')" />
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Panel>
                </div>
            </div>

            <div class="mt-4 flex justify-end px-1 pb-2">
            </div>
        </div>

        <div class="flex items-center justify-between px-4 py-3">
            <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleBackClick"
                icon="mir-arrow_back" :label="t('g.back')" outlined />

            <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleInstallClick"
                icon="mir-install_desktop" :label="t('cls.install.self')" />
        </div>

        <DirectorySelectorDrawer v-model:visible="directoryDrawerVisible" :zip-path="installConfig.zip_path"
            @directory-select="installConfig.tool_details.add_to_path[1] = $event" />
    </div>
</template>
