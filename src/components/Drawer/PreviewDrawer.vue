<script setup lang="ts">
import ExecutableSelector from '@/components/ZipPreview/ExecutableSelector.vue'
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from 'primevue'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import Drawer from 'primevue/drawer'
import InputText from 'primevue/inputtext'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const installConfig = InstallConfigStore()
const toast = useToast()
const { t } = useI18n()
const showPasswordDialog = ref(false)
const archivePassword = ref('')
const passwordError = ref(false)
const detailsLoading = ref(false)

// Computed properties
const drawerVisible = computed({
    get: () => installConfig.show_preview_drawer,
    set: (value: boolean) => {
        installConfig.show_preview_drawer = value
        // Clear temp data when drawer is closed without confirmation
        if (!value) {
            installConfig.clearTempData()
        }
    }
})

// Handle button actions
function handleExecutableSelected() {
    // Confirm temp data when user selects executable
    installConfig.confirmTempData()
    installConfig.show_preview_drawer = false
    goTo('/Install/App/Config')
}

function handleNoExecutable() {
    // Confirm temp data when user selects no executable
    installConfig.confirmTempData()
    installConfig.show_preview_drawer = false
    goTo('/Install/Tool/Config')
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}

// Watch for drawer visibility changes to load content
watch(drawerVisible, async (visible) => {
    if (visible && installConfig.temp.zip_path && installConfig.temp.file_tree.length === 0) {
        try {
            await GetArchiveContent('')
        } catch (error) {
            if (error === 'Wrong password') {
                showPasswordDialog.value = true
            } else {
                toast.add({
                    severity: 'error',
                    summary: t('g.error'),
                    detail: String(error),
                    life: 0,
                })
                installConfig.show_preview_drawer = false
            }
        }
    }
})

// Watch for password changes to reset error
watch(archivePassword, () => {
    passwordError.value = false
})

async function handlePasswordSubmit() {
    if (!archivePassword.value) {
        passwordError.value = true
        return
    }

    try {
        await GetArchiveContent(archivePassword.value)
        installConfig.setTempData({ archive_password: archivePassword.value })
        showPasswordDialog.value = false
        archivePassword.value = ''
    } catch (error) {
        if (error === 'Wrong password') {
            passwordError.value = true
        } else {
            toast.add({
                severity: 'error',
                summary: t('g.error'),
                detail: String(error),
                life: 0,
            })
            showPasswordDialog.value = false
            installConfig.show_preview_drawer = false
        }
    }
}

async function GetArchiveContent(password: string) {
    const result = await invoke('execute_command', {
        command: {
            name: 'GetArchiveTree',
            path: installConfig.temp.zip_path,
            password: password,
        },
    })

    const treeData = JSON.parse(result as string)
    installConfig.setTempData({ file_tree: treeData })
}
</script>

<template>
    <Drawer v-model:visible="drawerVisible" position="bottom" :style="{ height: '95vh' }" class="rounded-lg"
        :header="t('ui.preview.header')">
        <!-- Password Dialog -->
        <Dialog v-model:visible="showPasswordDialog" :modal="true" :closable="false"
            :header="t('ui.archive.password_required')" class="w-[30rem]">
            <div class="flex flex-col gap-3">
                <p class="text-sm">{{ t('ui.archive.enter_password') }}</p>
                <div class="flex flex-col gap-1">
                    <InputText v-model="archivePassword" type="password" :class="{ 'border-red-500': passwordError }"
                        @keydown.enter="handlePasswordSubmit" class="w-full" :invalid="passwordError" />
                    <small v-if="passwordError" class="text-red-500">{{ t('ui.archive.password_required') }}</small>
                </div>
            </div>
            <template #footer>
                <div class="flex justify-end gap-2">
                    <Button @click="installConfig.show_preview_drawer = false" :label="t('g.cancel')"
                        severity="secondary" outlined icon="mir-close" />
                    <Button @click="handlePasswordSubmit" :label="t('g.submit')" icon="mir-check" />
                </div>
            </template>
        </Dialog>

        <!-- Main content area -->
        <div class="flex h-full flex-col gap-4">
            <!-- File info header -->
            <div v-if="installConfig.temp.zip_path" class="flex flex-col gap-1">
                <h3 class="text-lg font-semibold">
                    {{ installConfig.temp.zip_path.split(/[/\\]/).pop() }}
                </h3>
                <p class="text-sm text-slate-500 dark:text-slate-400">{{ installConfig.temp.url ||
                    installConfig.temp.zip_path
                    }}
                </p>
            </div>

            <!-- ExecutableSelector embedded directly -->
            <div class="min-h-0 flex-1">
                <ExecutableSelector v-if="installConfig.temp.zip_path && installConfig.temp.file_tree.length > 0"
                    :zip-path="installConfig.temp.zip_path" :password="installConfig.temp.archive_password"
                    :file-tree="installConfig.temp.file_tree" :details-loading="detailsLoading"
                    @loading="handleDetailsLoading" @executable-selected="handleExecutableSelected"
                    @no-executable="handleNoExecutable"
                    @update-file-tree="installConfig.setTempData({ file_tree: $event })" />
            </div>
        </div>
    </Drawer>
</template>
