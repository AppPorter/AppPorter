<script setup lang="ts">
import DirectorySelector from '@/components/ZipPreview/DirectorySelector.vue'
import { goTo } from '@/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useToast } from 'primevue'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import Dialog from 'primevue/dialog'
import Drawer from 'primevue/drawer'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { useConfirm } from 'primevue/useconfirm'
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const installationConfig = useInstallationConfigStore()
installationConfig.page = 'Config' // Reuse Config page state
const toast = useToast()
const confirm = useConfirm()
const { t } = useI18n()
const showErrorDialog = ref(false)
const showPasswordDialog = ref(false)
const archivePassword = ref('')
const passwordError = ref(false)

// UI state management
const pathError = ref('')
const directoryDrawerVisible = ref(false)
const detailsLoading = ref(false)

// Load archive content when component is mounted
onMounted(async () => {
    if (!installationConfig.zip_path) {
        goTo('/Home')
        return
    }

    try {
        // First attempt without password
        await GetArchiveContent('')
    } catch (error) {
        if (error === 'Wrong password') {
            showPasswordDialog.value = true
        } else {
            toast.add({
                severity: 'error',
                summary: t('error'),
                detail: String(error),
                life: 0,
            })
            goTo('/Home')
        }
    }
})

// Watch for password changes to reset error
watch(archivePassword, () => {
    passwordError.value = false
})

async function handleDialogClose() {
    showErrorDialog.value = false
    goTo('/Home')
}

async function handlePasswordSubmit() {
    if (!archivePassword.value) {
        passwordError.value = true
        return
    }

    try {
        await GetArchiveContent(archivePassword.value)
        // Store password in the installation config for later use
        installationConfig.archive_password = archivePassword.value
        showPasswordDialog.value = false
        archivePassword.value = ''
    } catch (error) {
        if (error === 'Wrong password') {
            passwordError.value = true
        } else {
            toast.add({
                severity: 'error',
                summary: t('error'),
                detail: String(error),
                life: 0,
            })
            showPasswordDialog.value = false
            goTo('/Home')
        }
    }
}

async function GetArchiveContent(password: string) {
    const result = await invoke('execute_command', {
        command: {
            name: 'GetArchiveContent',
            path: installationConfig.zip_path,
            password: password,
        },
    })

    const content = JSON.parse(result as string)
    installationConfig.archive_content = content
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}

function handleBackClick() {
    goTo('/Home')
}

// Handle extraction process initiation
async function handleExtractClick() {
    // Reset validation errors
    pathError.value = ''

    if (!installationConfig.install_path) {
        pathError.value = t('validation.select_path')
        toast.add({
            severity: 'error',
            summary: t('validation.path_required'),
            detail: t('validation.select_path'),
            life: 3000,
        })
        return
    }

    try {
        const validatedPath = (await invoke('execute_command', {
            command: {
                name: 'ValidatePath',
                path: installationConfig.install_path,
            },
        })) as string

        installationConfig.install_path = validatedPath
        const fullPath = `${validatedPath}\\${installationConfig.name || 'Extracted-Files'}`

        try {
            await invoke('execute_command', {
                command: {
                    name: 'CheckPathEmpty',
                    path: fullPath,
                },
            })

            await new Promise((resolve, reject) => {
                confirm.require({
                    message: t('copyonly.confirm_extract'),
                    group: 'dialog',
                    icon: 'mir-folder_copy',
                    header: t('copyonly.start_extraction'),
                    rejectProps: {
                        label: t('cancel'),
                        severity: 'secondary',
                        outlined: true,
                        icon: 'mir-close',
                    },
                    acceptProps: {
                        label: t('extract'),
                        icon: 'mir-navigate_next',
                    },
                    accept: () => resolve(true),
                    reject: () => reject(),
                })
            })
        } catch (error) {
            if (error === 'Installation directory is not empty') {
                await new Promise((resolve, reject) => {
                    confirm.require({
                        message: t('installation.config.directory_not_empty'),
                        group: 'dialog',
                        icon: 'mir-warning',
                        header: t('warning'),
                        rejectProps: {
                            label: t('cancel'),
                            severity: 'secondary',
                            outlined: true,
                            icon: 'mir-close',
                        },
                        acceptProps: {
                            label: t('copyonly.force_extract'),
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

        // Since we're just copying files, we can go directly to the progress page
        goTo('/CopyOnly/Progress')
    } catch (error) {
        pathError.value = error as string
    }
}

// Select extraction directory using file dialog
async function select_extract_path() {
    const selected = await open({
        directory: true,
        multiple: false,
    })
    if (selected) {
        installationConfig.install_path = String(selected)
    }
}
</script>

<template>
    <div class="flex size-full flex-col overflow-hidden">
        <!-- Main scrollable container -->
        <div class="flex-1 overflow-auto">
            <!-- Content wrapper -->
            <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
                <div class="min-w-72 flex-1 space-y-2">
                    <!-- Combined extraction panel -->
                    <Panel class="shadow-sm">
                        <template #header>
                            <div class="flex flex-col">
                                <div class="flex items-center gap-1.5">
                                    <span class="mir-folder_copy text-lg" />
                                    <h2 class="text-base font-medium">
                                        {{ t('copyonly.file_details') }}
                                    </h2>
                                </div>
                                <p class="ml-6 mt-0.5 text-xs">
                                    {{ t('selected_file') }}:
                                    <span class="break-all font-medium">{{ installationConfig.zip_path }}</span>
                                </p>
                            </div>
                        </template>

                        <div class="space-y-4 p-2">
                            <!-- Name input (optional for extraction) -->
                            <div class="flex items-center gap-2">
                                <label class="w-24 text-sm font-medium">
                                    {{ t('app.name') }}
                                    <p class="text-xs font-normal">{{ t('optional') }}</p>
                                </label>
                                <div class="w-full">
                                    <InputText v-model="installationConfig.name" :placeholder="t('app.name')"
                                        class="h-8 w-full text-sm" />
                                </div>
                            </div>

                            <!-- Extract Path -->
                            <div class="flex items-center gap-2">
                                <label class="w-24 text-sm font-medium">{{ t('extract_path') }}</label>
                                <div class="w-full">
                                    <div class="flex flex-1 gap-2">
                                        <InputText v-model="installationConfig.install_path"
                                            :placeholder="t('choose_dir')" class="h-8 w-full text-sm"
                                            :invalid="!!pathError" @input="pathError = ''" :title="pathError" />
                                        <Button class="h-8 w-36" severity="secondary" @click="select_extract_path"
                                            icon="mir-folder_open" :label="t('browse')" />
                                    </div>
                                </div>
                            </div>

                            <!-- Add to PATH option -->
                            <div class="mt-2 flex items-start gap-2">
                                <label class="mt-1 w-24 text-sm font-medium">{{ t('environment') }}</label>
                                <div class="w-full">
                                    <div class="flex-1 space-y-1 rounded-lg p-1.5">
                                        <div class="flex flex-col gap-1">
                                            <div class="flex items-center gap-2">
                                                <Checkbox v-model="installationConfig.add_to_path" :binary="true"
                                                    inputId="add_to_path" />
                                                <label for="add_to_path" class="text-sm">{{ t('add_to_path')
                                                    }}</label>
                                            </div>
                                            <!-- PATH Directory Input - only shown when add_to_path is true -->
                                            <div v-if="installationConfig.add_to_path" class="ml-6 mt-1">
                                                <div class="flex gap-2">
                                                    <InputText v-model="installationConfig.path_directory"
                                                        :placeholder="t('select_path_directory')"
                                                        class="h-8 w-full text-sm" />
                                                    <Button class="h-8 w-36" severity="secondary"
                                                        @click="directoryDrawerVisible = true" icon="mir-folder_open"
                                                        :label="t('browse')" />
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

            <!-- Button container -->
            <div class="mt-4 flex justify-between px-1 pb-2">
                <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200"
                    @click="handleBackClick" icon="mir-arrow_back" :label="t('back')" outlined />
                <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200"
                    @click="handleExtractClick" icon="mir-folder_copy" :label="t('extract')" />
            </div>
        </div>

        <!-- Error Dialog -->
        <Dialog v-model:visible="showErrorDialog" :modal="true" :closable="false"
            :header="t('validation.invalid_archive')" class="w-[30rem]">
            <div class="flex items-start gap-3">
                <span class="mir-error text-3xl text-red-500"></span>
                <p class="text-sm">{{ t('validation.archive_error') }}</p>
            </div>
            <template #footer>
                <div class="flex justify-end">
                    <Button @click="handleDialogClose" :label="t('ok')" icon="mir-close" />
                </div>
            </template>
        </Dialog>

        <!-- Password Dialog -->
        <Dialog v-model:visible="showPasswordDialog" :modal="true" :closable="false"
            :header="t('archive.password_required')" class="w-[30rem]">
            <div class="flex flex-col gap-3">
                <p class="text-sm">{{ t('archive.enter_password') }}</p>
                <div class="flex flex-col gap-1">
                    <InputText v-model="archivePassword" type="password" :class="{ 'border-red-500': passwordError }"
                        @keydown.enter="handlePasswordSubmit" class="w-full" :invalid="passwordError" />
                    <small v-if="passwordError" class="text-red-500">{{ t('validation.password_required') }}</small>
                </div>
            </div>
            <template #footer>
                <div class="flex justify-end gap-2">
                    <Button @click="goTo('/Home')" :label="t('cancel')" severity="secondary" outlined
                        icon="mir-close" />
                    <Button @click="handlePasswordSubmit" :label="t('submit')" icon="mir-check" />
                </div>
            </template>
        </Dialog>

        <!-- Directory Selector Drawer -->
        <Drawer v-model:visible="directoryDrawerVisible" :header="t('select_path_directory')" position="bottom"
            :style="{ height: '95vh' }" class="rounded-lg">
            <div class="h-full overflow-hidden">
                <DirectorySelector :zip-path="installationConfig.zip_path" :details-loading="detailsLoading"
                    @close="directoryDrawerVisible = false" @loading="handleDetailsLoading"
                    @directory-select="installationConfig.path_directory = $event" />
            </div>
        </Drawer>
    </div>
</template>
