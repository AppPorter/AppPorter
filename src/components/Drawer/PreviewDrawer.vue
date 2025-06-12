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
    get: () => installConfig.showPreviewDrawer,
    set: (value: boolean) => {
        installConfig.showPreviewDrawer = value
    }
})

// Handle button actions
function handleExecutableSelected() {
    installConfig.showPreviewDrawer = false
    goTo('/Install/App/Config')
}

function handleNoExecutable() {
    installConfig.showPreviewDrawer = false
    goTo('/Install/Tool/Config')
}

function handleDetailsLoading(loading: boolean) {
    detailsLoading.value = loading
}

// Watch for drawer visibility changes to load content
watch(drawerVisible, async (visible) => {
    if (visible && installConfig.zip_path && !installConfig.archive_content) {
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
                installConfig.showPreviewDrawer = false
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
        installConfig.app_details.config.archive_password = archivePassword.value
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
            installConfig.showPreviewDrawer = false
        }
    }
}

async function GetArchiveContent(password: string) {
    const result = await invoke('execute_command', {
        command: {
            name: 'GetArchiveContent',
            path: installConfig.zip_path,
            password: password,
        },
    })

    const content = JSON.parse(result as string)
    installConfig.archive_content = content
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
                    <Button @click="installConfig.showPreviewDrawer = false" :label="t('g.cancel')" severity="secondary"
                        outlined icon="mir-close" />
                    <Button @click="handlePasswordSubmit" :label="t('g.submit')" icon="mir-check" />
                </div>
            </template>
        </Dialog>

        <!-- Main content area -->
        <div class="flex h-full flex-col gap-4">
            <!-- File info header -->
            <div v-if="installConfig.zip_path" class="flex flex-col gap-1">
                <h3 class="text-lg font-semibold text-gray-900">
                    {{ installConfig.zip_path.split(/[/\\]/).pop() }}
                </h3>
                <p class="text-sm text-gray-600">{{ installConfig.url || installConfig.zip_path }}</p>
            </div>

            <!-- ExecutableSelector embedded directly -->
            <div class="min-h-0 flex-1">
                <ExecutableSelector v-if="installConfig.zip_path && installConfig.archive_content"
                    :zip-path="installConfig.zip_path" :details-loading="detailsLoading" @loading="handleDetailsLoading"
                    @executable-selected="handleExecutableSelected" @no-executable="handleNoExecutable" />
            </div>
        </div>
    </Drawer>
</template>
