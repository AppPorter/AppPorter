<script setup lang="ts">
import { exec } from '@/exec'
import { generalStore, installConfig } from '@/main'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

generalStore.page = 'Home'
const { t } = useI18n()
const inputError = ref(false)

async function selectZipFile() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: t('g.archive'),
        extensions: ['zip', '7z', 'rar', 'tar', 'gz', 'bz2', 'xz', 'cab'],
      },
    ],
  })

  if (selected) {
    installConfig.zip_path = selected
  }
}

async function handleContinueClick() {
  if (!installConfig.zip_path) {
    return
  }

  type InputType = 'Url' | 'File' | 'Invalid'

  const inputType = await exec('DetermineInputType', {
    input: installConfig.zip_path
  }) as InputType

  if (inputType === 'Invalid') {
    inputError.value = true
    return
  }

  if (inputType === 'Url') {
    await invoke('exec', {
      cmd: {
        name: 'PreviewUrl',
        url: installConfig.zip_path
      }
    })
  } else {
    generalStore.drawer.preview = true
  }
}
</script>

<template>
  <div class="flex size-full items-center justify-center">
    <Panel class="h-52 w-full max-w-3xl px-4 shadow-sm">
      <template #header>
        <div class="flex items-center gap-2">
          <span class="mir-folder_zip text-xl"></span>
          <div>
            <h2 class="text-lg font-medium">{{ t('cls.install.self') }}</h2>
            <p class="mt-0.5 text-xs">{{ t('ui.install.description') }}</p>
          </div>
        </div>
      </template>

      <div class="space-y-6">
        <div class="flex items-center gap-2">
          <InputText v-model="installConfig.zip_path" :placeholder="t('ui.select_placeholder.archive_or_url')"
            :invalid="inputError" class="h-9 flex-1 text-sm" />
          <Button @click="selectZipFile" severity="secondary" class="h-9 px-4" icon="mir-folder_open"
            :label="t('g.browse')" />
        </div>

        <div class="flex justify-end gap-x-2">
          <Button @click="handleContinueClick" :disabled="!installConfig.zip_path" severity="primary" class="h-9 px-6"
            icon="mir-install_desktop" :label="t('g.continue')" />
        </div>
      </div>
    </Panel>
  </div>
</template>
