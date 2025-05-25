<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from 'primevue'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import { useConfirm } from 'primevue/useconfirm'
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import AppDetails from './components/AppDetails.vue'
import Options from './components/Options.vue'

const installConfig = InstallConfigStore()
installConfig.page = 'Install_App_Config'
const toast = useToast()
const confirm = useConfirm()
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

// UI state management
const detailsLoading = ref(false)
const detailsLoadingProgress = ref(0)
const progressMode = ref<'indeterminate' | 'determinate'>('indeterminate')

// Validation states
const pathError = ref('')
const nameError = ref(false)
const executablePathError = ref(false)

function handleBackClick() {
  goTo('/Home')
}

// Handle install process initiation
async function handleInstallClick() {
  // Reset validation errors
  nameError.value = false
  pathError.value = ''
  executablePathError.value = false

  let hasErrors = false

  // Validate required fields
  if (!installConfig.executable_path) {
    executablePathError.value = true
    toast.add({
      severity: 'error',
      summary: t('validation.executable_missing'),
      detail: t('validation.select_executable'),
      life: 3000,
    })
    hasErrors = true
  }

  if (!installConfig.name) {
    nameError.value = true
    toast.add({
      severity: 'error',
      summary: t('validation.name_required'),
      detail: t('validation.enter_name'),
      life: 3000,
    })
    hasErrors = true
  }

  if (!installConfig.install_path) {
    pathError.value = t('validation.select_path')
    toast.add({
      severity: 'error',
      summary: t('validation.path_required'),
      detail: t('validation.select_path'),
      life: 3000,
    })
    hasErrors = true
  }

  if (hasErrors) {
    return
  }

  try {
    const validatedPath = (await invoke('execute_command', {
      command: {
        name: 'ValidatePath',
        path: installConfig.install_path,
      },
    })) as string

    installConfig.details.paths.parent_install_path = validatedPath
    const fullPath = `${validatedPath}\\${installConfig.name}`

    try {
      await invoke('execute_command', {
        command: {
          name: 'CheckPathEmpty',
          path: fullPath,
        },
      })

      await new Promise((resolve, reject) => {
        confirm.require({
          message: t('install.config.confirm_install'),
          group: 'dialog',
          icon: 'mir-install_desktop',
          header: t('install.config.start_install'),
          rejectProps: {
            label: t('cancel'),
            severity: 'secondary',
            outlined: true,
            icon: 'mir-close',
          },
          acceptProps: {
            label: t('install'),
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
            message: t('install.config.directory_not_empty'),
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
              label: t('install.config.force_install'),
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
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <!-- Main scrollable container -->
    <div class="flex-1 overflow-auto">
      <!-- Content wrapper -->
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
          <AppDetails :name-error="nameError" :details-loading="detailsLoading"
            :details-loading-progress="detailsLoadingProgress" :progress-mode="progressMode"
            :executable-path-error="executablePathError" />
          <Options :path-error="pathError" @update:path-error="(val) => (pathError = val)" />
        </div>
      </div>

      <!-- Button container -->
      <div class="mt-4 flex justify-between px-1 pb-2">
        <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleBackClick"
          icon="mir-arrow_back" :label="t('back')" outlined />
        <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleInstallClick"
          icon="mir-install_desktop" :label="t('install')" />
      </div>
    </div>

    <!-- Error Dialog -->
    <Dialog v-model:visible="showErrorDialog" :modal="true" :closable="false" :header="t('validation.invalid_archive')"
      class="w-[30rem]">
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
          <Button @click="goTo('/Home')" :label="t('cancel')" severity="secondary" outlined icon="mir-close" />
          <Button @click="handlePasswordSubmit" :label="t('submit')" icon="mir-check" />
        </div>
      </template>
    </Dialog>
  </div>
</template>
