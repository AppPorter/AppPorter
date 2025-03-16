<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { useSettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import ToggleSwitch from 'primevue/toggleswitch'
import { watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  pathError: string
}>()

// Store setup
const settingsStore = useSettingsStore()
const {
  installation: { current_user_only: settings_current_user_only, all_users, current_user },
} = settingsStore

const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
const {
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
} = storeToRefs(installationConfig)

const { t } = useI18n()

// Sync settings between installation modes
function updateConfig(isCurrentUser: boolean) {
  const sourceConfig = isCurrentUser ? current_user : all_users

  create_desktop_shortcut.value = sourceConfig.create_desktop_shortcut
  create_registry_key.value = sourceConfig.create_registry_key
  create_start_menu_shortcut.value = sourceConfig.create_start_menu_shortcut
  install_path.value = sourceConfig.install_path
}

// Initialize with settings
current_user_only.value = settings_current_user_only
updateConfig(current_user_only.value)

// Select installation directory using file dialog
async function select_install_path() {
  const selected = await open({
    directory: true,
    multiple: false,
  })
  if (selected) {
    install_path.value = String(selected)
  }
}

// Update configuration when installation mode changes
function handleInstallModeChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked
  current_user_only.value = checked
  updateConfig(checked)
}

// Ensure configuration stays in sync with installation mode
watchEffect(() => {
  updateConfig(current_user_only.value)
})
</script>

<template>
  <Panel class="border shadow-sm">
    <template #header>
      <div class="flex flex-col">
        <div class="flex items-center gap-1.5">
          <span class="mir settings text-lg"></span>
          <h2 class="text-base font-medium">
            {{ t('installation.config.installation_options') }}
          </h2>
        </div>
        <p class="ml-6 mt-0.5 text-xs">
          {{ t('installation.config.selected_file') }}:
          <span class="overflow-wrap-anywhere break-all font-medium">{{ zip_path }}</span>
        </p>
      </div>
    </template>

    <div class="space-y-2 p-2">
      <!-- Install Mode -->
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('installation.config.install_mode') }}</label>
        <div class="flex items-center gap-2 rounded-lg px-2 py-1">
          <span class="text-sm">{{ t('installation.config.all_users') }}</span>
          <ToggleSwitch
            v-model="current_user_only"
            @change="handleInstallModeChange"
            class="mx-1"
          />
          <span class="text-sm">{{ t('installation.config.current_user') }}</span>
        </div>
      </div>

      <!-- Install Path -->
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('installation.config.install_path') }}</label>
        <div class="flex flex-1 gap-2">
          <InputText
            v-model="install_path"
            :placeholder="t('installation.config.choose_dir')"
            class="h-8 w-full text-sm"
            :invalid="!!pathError"
            :title="pathError"
          />
          <Button
            class="h-8 w-36"
            severity="secondary"
            @click="select_install_path"
            icon="mir folder_open"
            :label="t('installation.config.browse')"
          />
        </div>
      </div>

      <!-- Shortcuts Section -->
      <div class="flex items-start gap-2">
        <label class="mt-1 w-24 text-sm font-medium">{{
          t('installation.config.shortcuts')
        }}</label>
        <div class="flex-1 space-y-1 rounded-lg p-1.5">
          <div class="flex items-center gap-2">
            <Checkbox v-model="create_desktop_shortcut" :binary="true" inputId="desktop_shortcut" />
            <label for="desktop_shortcut" class="text-sm">{{
              t('installation.config.desktop_shortcut')
            }}</label>
          </div>
          <div class="flex items-center gap-2">
            <Checkbox
              v-model="create_start_menu_shortcut"
              :binary="true"
              inputId="start_menu_shortcut"
            />
            <label for="start_menu_shortcut" class="text-sm">{{
              t('installation.config.start_menu_shortcut')
            }}</label>
          </div>
          <div class="flex items-center gap-2">
            <Checkbox v-model="create_registry_key" :binary="true" inputId="registry_key" />
            <label for="registry_key" class="text-sm">{{
              t('installation.config.registry_entry')
            }}</label>
          </div>
        </div>
      </div>
    </div>
  </Panel>
</template>
