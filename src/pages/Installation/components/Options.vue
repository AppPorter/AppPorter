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
import { watchEffect, computed } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  pathError: string
}>()

const settingsStore = useSettingsStore()
const {
  installation: { current_user_only: settings_current_user_only, all_users, current_user },
} = settingsStore

const installationConfig = useInstallationConfigStore()
const {
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
  add_to_path,
  path_directory,
} = storeToRefs(installationConfig)

const { t } = useI18n()

// Format app path based on install path and app name
const formatted_app_path = computed(() => {
  return `${install_path.value}\\${installationConfig.name.replace(/ /g, '-')}`
})

// Sync settings between installation modes
function updateConfig(isCurrentUser: boolean) {
  const sourceConfig = isCurrentUser ? current_user : all_users

  create_desktop_shortcut.value = sourceConfig.create_desktop_shortcut
  create_registry_key.value = sourceConfig.create_registry_key
  create_start_menu_shortcut.value = sourceConfig.create_start_menu_shortcut
  install_path.value = sourceConfig.install_path
  add_to_path.value = sourceConfig.add_to_path
  path_directory.value = ''  // Reset path directory when switching modes
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
  <Panel class="shadow-sm">
    <template #header>
      <div class="flex flex-col">
        <div class="flex items-center gap-1.5">
          <span class="mir-settings" />
          <h2 class="text-base font-medium">
            {{ t('installation.config.installation_options') }}
          </h2>
        </div>
      </div>
    </template>

    <div class="space-y-2 p-2">
      <!-- Install Mode -->
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('install_mode') }}</label>
        <div class="flex w-full items-center gap-2 rounded-lg px-2 py-1">
          <span class="text-sm">{{ t('all_users') }}</span>
          <ToggleSwitch v-model="current_user_only" @change="handleInstallModeChange" class="mx-1" />
          <span class="text-sm">{{ t('current_user') }}</span>
        </div>
      </div>

      <!-- Install Path -->
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('install_path') }}</label>
        <div class="w-full">
          <div class="flex flex-1 gap-2">
            <InputText v-model="install_path" :placeholder="t('choose_dir')" class="h-8 w-full text-sm"
              :invalid="!!pathError" @input="$emit('update:pathError', '')" :title="pathError" />
            <Button class="h-8 w-36" severity="secondary" @click="select_install_path" icon="mir-folder_open"
              :label="t('browse')" />
          </div>

          <!-- Formatted App Path -->
          <div v-if="install_path && installationConfig.name" class="mt-1 text-xs text-gray-500">
            {{ t('final_path') }}: {{ formatted_app_path }}
          </div>
        </div>
      </div>

      <!-- Shortcuts Section -->
      <div class="flex items-start gap-2">
        <label class="mt-1 w-24 text-sm font-medium">{{ t('shortcuts.self') }}</label>
        <div class="w-full">
          <div class="flex-1 space-y-1 rounded-lg p-1.5">
            <div class="flex items-center gap-2">
              <Checkbox v-model="create_desktop_shortcut" :binary="true" inputId="desktop_shortcut" />
              <label for="desktop_shortcut" class="text-sm">{{ t('shortcuts.desktop') }}</label>
            </div>
            <div class="flex items-center gap-2">
              <Checkbox v-model="create_start_menu_shortcut" :binary="true" inputId="start_menu_shortcut" />
              <label for="start_menu_shortcut" class="text-sm">{{
                t('shortcuts.start_menu')
                }}</label>
            </div>
            <div class="flex items-center gap-2">
              <Checkbox v-model="create_registry_key" :binary="true" inputId="registry_key" />
              <label for="registry_key" class="text-sm">{{ t('shortcuts.registry_key') }}</label>
            </div>
            <div class="flex flex-col gap-1">
              <div class="flex items-center gap-2">
                <Checkbox v-model="add_to_path" :binary="true" inputId="add_to_path" />
                <label for="add_to_path" class="text-sm">{{ t('add_to_path') }}</label>
              </div>
              <!-- PATH Directory Input - only shown when add_to_path is true -->
              <div v-if="add_to_path" class="ml-6 mt-1">
                <div class="flex gap-2">
                  <InputText v-model="path_directory" :placeholder="t('select_path_directory')"
                    class="h-8 w-full text-sm" />
                  <Button class="h-8 w-36" severity="secondary" icon="mir-folder_open" :label="t('browse')" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Panel>
</template>
