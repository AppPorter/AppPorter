<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import SelectButton from 'primevue/selectbutton'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import AppConfig from './App/AppConfig.vue'
import LibConfig from './Lib/LibConfig.vue'

const { t } = useI18n()
const installConfig = InstallConfigStore()
const confirm = useConfirm()

// Validation states
const pathError = ref('')
const nameError = ref(false)
const executablePathError = ref(false)

// Mode options for SelectButton
const modeOptions = [
    { label: t('cls.install.types.app'), value: 'app', icon: 'mir-install_desktop' },
    { label: t('cls.install.types.lib'), value: 'lib', icon: 'mir-folder_copy' }
]

// Current mode based on install_config page
const currentMode = computed({
    get: () => {
        return installConfig.page === 'Install_App_Config' ? 'app' : 'lib'
    },
    set: (value: string) => {
        installConfig.page = value === 'app' ? 'Install_App_Config' : 'Install_Lib_Config'
    }
})

onMounted(() => {
    // Set default mode if not already set
    if (!installConfig.page || (installConfig.page !== 'Install_App_Config' && installConfig.page !== 'Install_Lib_Config')) {
        installConfig.page = 'Install_App_Config'
    }
})

// Handle back button click
function handleBackClick() {
    goTo('/Install/Preview')
}

// Handle install process for both app and lib modes
async function handleInstallClick() {
    if (currentMode.value === 'app') {
        await handleAppInstall()
    } else {
        await handleLibInstall()
    }
}

// Handle app installation
async function handleAppInstall() {
    // Reset validation errors
    nameError.value = false
    pathError.value = ''
    executablePathError.value = false

    // Validate required fields
    if (!installConfig.app_details.config.archive_exe_path) {
        executablePathError.value = true
    }

    if (!installConfig.app_details.info.name) {
        nameError.value = true
    }

    if (!installConfig.app_details.paths.parent_install_path) {
        pathError.value = t('ui.valid.select_path')
    }

    // If any validation failed, return early
    if (executablePathError.value || nameError.value || pathError.value) {
        return
    }

    try {
        const validatedPath = (await invoke('execute_command', {
            command: {
                name: 'ValidatePath',
                path: installConfig.app_details.paths.parent_install_path,
            },
        })) as string

        installConfig.app_details.paths.parent_install_path = validatedPath
        const fullPath = `${validatedPath}\\${installConfig.app_details.info.name}`

        try {
            await invoke('execute_command', {
                command: {
                    name: 'CheckPathEmpty',
                    path: fullPath,
                },
            })

            await new Promise((resolve, reject) => {
                confirm.require({
                    message: t('ui.install.confirm_install'),
                    group: 'dialog',
                    icon: 'mir-install_desktop',
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
            if (error === 'Install directory is not empty') {
                await new Promise((resolve, reject) => {
                    confirm.require({
                        message: t('ui.valid.directory_not_empty'),
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
                            icon: 'mir-warning',
                        },
                        accept: () => resolve(true),
                        reject: () => reject(),
                    })
                })
            } else {
                pathError.value = error as string
                return
            }
        }

        goTo('/Install/Progress')
    } catch (error) {
        pathError.value = error as string
    }
}

// Handle library installation
async function handleLibInstall() {
    // Reset validation errors
    pathError.value = ''

    // Validate required fields
    if (!installConfig.lib_details.name) {
        return
    }

    if (!installConfig.lib_details.paths.parent_install_path) {
        pathError.value = t('ui.valid.select_path')
        return
    }

    try {
        const validatedPath = (await invoke('execute_command', {
            command: {
                name: 'ValidatePath',
                path: installConfig.lib_details.paths.parent_install_path,
            },
        })) as string

        installConfig.lib_details.paths.parent_install_path = validatedPath
        const fullPath = `${validatedPath}\\${installConfig.lib_details.name || 'Extracted-Files'}`

        try {
            await invoke('execute_command', {
                command: {
                    name: 'CheckPathEmpty',
                    path: fullPath,
                },
            })

            await new Promise((resolve, reject) => {
                confirm.require({
                    message: t('ui.install.confirm_install'),
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
            if (error === 'Install directory is not empty') {
                await new Promise((resolve, reject) => {
                    confirm.require({
                        message: t('ui.valid.directory_not_empty'),
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
                            icon: 'mir-warning',
                        },
                        accept: () => resolve(true),
                        reject: () => reject(),
                    })
                })
            } else {
                pathError.value = error as string
                return
            }
        }

        goTo('/CopyOnly/Progress')
    } catch (error) {
        pathError.value = error as string
    }
}
</script>

<template>
    <div class="flex size-full flex-col overflow-hidden">
        <!-- Dynamic content based on selected mode -->
        <div class="flex-1 overflow-hidden">
            <AppConfig v-if="currentMode === 'app'" :name-error="nameError" :executable-path-error="executablePathError"
                :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
            <LibConfig v-else :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
        </div>

        <!-- Bottom bar with mode selector and buttons -->
        <div class="flex items-center justify-between border-t bg-surface-0 px-4 py-3">
            <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleBackClick"
                icon="mir-arrow_back" :label="t('g.back')" outlined />

            <SelectButton v-model="currentMode" :options="modeOptions" :allowEmpty=false option-label="label"
                size="small" option-value="value" />

            <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleInstallClick"
                icon="mir-install_desktop" :label="t('cls.install.self')" />
        </div>
    </div>
</template>