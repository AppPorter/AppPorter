<script setup lang="ts">
import { goTo } from '@/router'
import { InstallConfigStore } from '@/stores/install_config'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { useI18n } from 'vue-i18n'

const installConfig = InstallConfigStore()
installConfig.page = 'Home'
const { zip_path } = storeToRefs(installConfig)
const { t } = useI18n()

async function selectZipFile() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: t('archives.self'),
        extensions: ['zip', '7z', 'rar', 'tar', 'gz', 'bz2', 'xz', 'cab'],
      },
    ],
  })

  if (selected) {
    zip_path.value = selected
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
            <h2 class="text-lg font-medium">{{ t('install.self') }}</h2>
            <p class="mt-0.5 text-xs">{{ t('install.description') }}</p>
          </div>
        </div>
      </template>

      <!-- Content Section -->
      <div class="space-y-6">
        <!-- File Selection Input -->
        <div class="flex items-center gap-2">
          <InputText v-model="zip_path" :placeholder="t('install.select_placeholder')" class="h-9 flex-1 text-sm" />
          <Button @click="selectZipFile" severity="secondary" class="h-9 px-4" icon="mir-folder_open"
            :label="t('basic.browse')" />
        </div>

        <!-- Navigation Button -->
        <div class="flex justify-end gap-x-2">
          <Button @click="goTo('/Install/Preview')" :disabled="!zip_path" severity="primary" class="h-9 px-6"
            icon="mir-install_desktop" :label="t('basic.next')" />
        </div>
      </div>
    </Panel>
  </div>
</template>
