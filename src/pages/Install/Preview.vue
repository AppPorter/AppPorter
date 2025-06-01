<script setup lang="ts">
import ZipPreview from '@/components/ZipPreview/ZipPreview.vue'
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from 'primevue'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const installConfig = InstallConfigStore()
installConfig.page = 'Preview'
const toast = useToast()
const { t } = useI18n()
const showErrorDialog = ref(false)
const showPasswordDialog = ref(false)
const archivePassword = ref('')
const passwordError = ref(false)

// Check if Subscribe button should be shown  
const showSubscribeButton = computed(() => !!installConfig.url)

// Handle button actions
function handleSubscribe() {
    // Handle subscribe action
    console.log('Subscribe clicked')
}

function handleInstallAsApp() {
    // Navigate to app installation
    goTo('/Install/Config')
}

function handleInstallAsLibrary() {
    // Navigate to library installation
    goTo('/Install/Config')
}

// Load archive content when component is mounted
onMounted(async () => {
    if (!installConfig.zip_path) {
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
        // Store password in the install config for later use
        installConfig.app_details.config.archive_password = archivePassword.value
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
            path: installConfig.zip_path,
            password: password,
        },
    })

    const content = JSON.parse(result as string)
    const executableExtensions = ['.exe', '.bat', '.cmd', '.ps1', '.sh', '.jar']
    const hasExecutable = content.some((path) =>
        executableExtensions.some((ext) => path.toLowerCase().endsWith(ext))
    )

    if (!hasExecutable) {
        showErrorDialog.value = true
        return
    }

    installConfig.archive_content = content
}
</script>

<template>
    <div class="flex size-full flex-col overflow-hidden">
        <!-- Error Dialog -->
        <Dialog v-model:visible="showErrorDialog" :modal="true" :closable="false"
            :header="t('validation.invalid_archive')" class="w-[30rem]">
            <div class="flex items-start gap-3">
                <span class="mir-error text-3xl text-red-500"></span>
                <p class="text-sm">{{ t('validation.no_executable_file') }}</p>
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

        <!-- Main content area -->
        <div class="flex min-h-0 flex-1 flex-col gap-4 p-4">
            <!-- File info header -->
            <div v-if="installConfig.zip_path" class="flex flex-col gap-1">
                <h2 class="text-xl font-semibold text-gray-900">
                    {{ installConfig.zip_path.split(/[/\\]/).pop() }}
                </h2>
                <p class="text-sm text-gray-600">{{ installConfig.zip_path }}</p>
            </div>

            <!-- Zip preview component -->
            <div class="min-h-0 flex-1">
                <ZipPreview v-if="installConfig.zip_path && installConfig.archive_content"
                    :zip-path="installConfig.zip_path" />
            </div>

            <!-- Action buttons -->
            <div class="flex items-center justify-between">
                <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="goTo('/Home')"
                    icon="mir-arrow_back" :label="t('back')" outlined />

                <div class="flex gap-3">
                    <Button v-if="showSubscribeButton" @click="handleSubscribe" :label="t('subscribe')"
                        icon="mir-rss_feed" class="px-6" />
                    <Button @click="handleInstallAsApp" :label="t('install_as_app')" icon="mir-apps" class="px-6" />
                    <Button @click="handleInstallAsLibrary" :label="t('install_as_library')" icon="mir-library_books"
                        severity="secondary" class="px-6" />
                </div>
            </div>
        </div>
    </div>
</template>