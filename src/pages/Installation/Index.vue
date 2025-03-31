<script setup lang="ts">
import { goTo } from '@/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const installationConfig = useInstallationConfigStore()
installationConfig.page = 'Home'
const { zip_path } = storeToRefs(installationConfig)
const { t } = useI18n()

// Dialog state
const showErrorDialog = ref(false)

async function selectZipFile() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: t('installation.archives'),
        extensions: ['zip', '7z', 'rar', 'tar', 'gz', 'bz2', 'xz', 'cab'],
      },
    ],
  })

  if (selected) {
    zip_path.value = selected
  }
}

async function handleNext() {
  const result = await invoke('execute_command', {
    command: {
      name: 'GetArchiveContent',
      path: zip_path.value,
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

  installationConfig.archive_content = content
  goTo('/Installation/Config')
}
</script>

<template>
  <div class="flex size-full items-center justify-center">
    <Panel class="h-52 w-full max-w-3xl px-4 shadow-sm">
      <template #header>
        <div class="flex items-center gap-2">
          <span class="mir-folder_zip text-xl"></span>
          <div>
            <h2 class="text-lg font-medium">{{ t('installation.title') }}</h2>
            <p class="mt-0.5 text-xs">{{ t('installation.description') }}</p>
          </div>
        </div>
      </template>

      <!-- Content Section -->
      <div class="space-y-6">
        <!-- File Selection Input -->
        <div class="flex items-center gap-2">
          <InputText
            v-model="zip_path"
            :placeholder="t('installation.select_placeholder')"
            class="h-9 flex-1 text-sm"
          />
          <Button
            @click="selectZipFile"
            severity="secondary"
            class="h-9 px-4"
            icon="mir-folder_open"
            :label="t('installation.browse')"
          />
        </div>

        <!-- Navigation Button -->
        <div class="flex justify-end">
          <Button
            @click="handleNext"
            :disabled="!zip_path"
            severity="primary"
            class="h-9 px-6"
            icon="mir-navigate_next"
            :label="t('installation.next')"
          />
        </div>
      </div>
    </Panel>

    <!-- Error Dialog -->
    <Dialog
      v-model:visible="showErrorDialog"
      :modal="true"
      :closable="false"
      :header="t('installation.error.invalid_archive')"
      class="w-[30rem]"
    >
      <div class="flex items-start gap-3">
        <span class="mir-error text-xl text-red-500"></span>
        <p class="text-sm">{{ t('installation.error.no_executable_file') }}</p>
      </div>
      <template #footer>
        <div class="flex justify-end">
          <Button
            @click="showErrorDialog = false"
            :label="t('installation.error.ok')"
            icon="mir-close"
          />
        </div>
      </template>
    </Dialog>
  </div>
</template>
