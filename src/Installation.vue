<script setup lang="ts">
// Core imports
import { goTo } from '@/plugins/router'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'

// PrimeVue components
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'

// Store initialization
const installationConfig = useInstallationConfigStore()
installationConfig.page = 'Home'
const { zip_path } = storeToRefs(installationConfig)
const { t } = useI18n()

// File selection handler
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
</script>

<template>
  <Panel class="w-full max-w-3xl mx-auto shadow-sm h-52">
    <template #header>
      <div class="flex items-center gap-2">
        <span class="mir folder_zip text-xl"></span>
        <div>
          <h2 class="text-lg font-medium">{{ t('installation.title') }}</h2>
          <p class="text-xs mt-0.5">{{ t('installation.description') }}</p>
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
          class="flex-1 text-sm h-9"
        />
        <Button
          @click="selectZipFile"
          severity="secondary"
          class="h-9 px-4"
          icon="mir folder_open"
          :label="t('installation.browse')"
        />
      </div>

      <!-- Navigation Button -->
      <div class="flex justify-end">
        <Button
          @click="goTo('/Installation/Config')"
          :disabled="!zip_path"
          severity="primary"
          class="h-9 px-6"
          icon="mir navigate_next"
          :label="t('installation.next')"
        />
      </div>
    </div>
  </Panel>
</template>
