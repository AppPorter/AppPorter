<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from 'primevue'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const installConfig = InstallConfigStore()
installConfig.page = 'Install_App_Preview'
const toast = useToast()
const { t } = useI18n()
const showErrorDialog = ref(false)
const showPasswordDialog = ref(false)
const archivePassword = ref('')
const passwordError = ref(false)

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
        installConfig.details.config.archive_password = archivePassword.value
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
    </div>
</template>