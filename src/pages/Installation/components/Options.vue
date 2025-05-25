<script setup lang="ts">
import DirectorySelector from '@/components/ZipPreview/DirectorySelector.vue'
import { InstallConfigStore } from '@/stores/install_config'
import { SettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import Drawer from 'primevue/drawer'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import ToggleSwitch from 'primevue/toggleswitch'
import { computed, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  pathError: string
}>()

const settingsStore = SettingsStore()
const {
  app_install: { current_user_only: settings_current_user_only, all_users, current_user },
} = settingsStore

const installConfig = InstallConfigStore()
const {
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
  add_to_path,
  path_directory,
  zip_path,
} = storeToRefs(installConfig)

const { t } = useI18n()

// Directory selector drawer state
const directoryDrawerVisible = ref(false)
const detailsLoading = ref(false)

function handleDetailsLoading(loading: boolean) {
  detailsLoading.value = loading
}

// Format app path based on install path and app name
const formatted_app_path = computed(() => {
  return `${install_path.value}\\${installConfig.name.replace(/ /g, '-')}`
})

// Sync settings between install modes
function updateConfig(isCurrentUser: boolean) {
  const sourceConfig = isCurrentUser ? current_user : all_users

  installConfig.details.config.create_desktop_shortcut = sourceConfig.create_desktop_shortcut
  installConfig.details.config.create_registry_key = sourceConfig.create_registry_key
  installConfig.details.config.create_start_menu_shortcut = sourceConfig.create_start_menu_shortcut
  installConfig.details.paths.parent_install_path = sourceConfig.install_path
  installConfig.details.config.add_to_path = sourceConfig.add_to_path
  installConfig.details.config.path_directory = ''  // Reset path directory when switching modes
}

// Initialize with settings
installConfig.details.config.current_user_only = settings_current_user_only
updateConfig(current_user_only.value)

// Select install directory using file dialog
async function select_install_path() {
  const selected = await open({
    directory: true,
    multiple: false,
  })
  if (selected) {
    installConfig.details.paths.parent_install_path = String(selected)
  }
}

// Update configuration when install mode changes
function handleInstallModeChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked
  installConfig.details.config.current_user_only = checked
  updateConfig(checked)
}

// Ensure configuration stays in sync with install mode
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
            {{ t('install.config.install_options') }}
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
          <div v-if="install_path && installConfig.name" class="mt-1 text-xs text-gray-500">
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
                  <Button class="h-8 w-36" severity="secondary" @click="directoryDrawerVisible = true"
                    icon="mir-folder_open" :label="t('browse')" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Panel>

  <!-- Directory Selector Drawer -->
  <Drawer v-model:visible="directoryDrawerVisible" :header="t('select_path_directory')" position="bottom"
    :style="{ height: '95vh' }" class="rounded-lg">
    <div class="h-full overflow-hidden">
      <DirectorySelector :zip-path="zip_path" :details-loading="detailsLoading" @close="directoryDrawerVisible = false"
        @loading="handleDetailsLoading" @directory-select="path_directory = $event" />
    </div>
  </Drawer>
</template>
