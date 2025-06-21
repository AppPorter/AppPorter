<script setup lang="ts">
import DirectorySelectorDrawer from '@/components/Drawer/DirectorySelectorDrawer.vue'
import { InstallConfigStore } from '@/stores/install_config'
import { SettingsStore } from '@/stores/settings'
import { open } from '@tauri-apps/plugin-dialog'
import { storeToRefs } from 'pinia'
import { computed, ref, toRef } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  pathError: boolean
}>()

defineEmits<{
  'update:pathError': [value: boolean]
}>()

const settingsStore = SettingsStore()
const installConfig = InstallConfigStore()
const { t } = useI18n()

// Extract settings for better readability
const {
  current_user_only: settings_current_user_only,
  all_users,
  current_user
} = settingsStore.app_install

// Use storeToRefs pattern like AppDetails
const { zip_path, app_details } = storeToRefs(installConfig)

// Direct refs to nested properties for v-model binding
const current_user_only = toRef(app_details.value.config, 'current_user_only')
const create_desktop_shortcut = toRef(app_details.value.config, 'create_desktop_shortcut')
const create_registry_key = toRef(app_details.value.config, 'create_registry_key')
const create_start_menu_shortcut = toRef(app_details.value.config, 'create_start_menu_shortcut')
const add_to_path = toRef(app_details.value.config, 'add_to_path')
const path_directory = toRef(app_details.value.config, 'path_directory')
const install_path = toRef(app_details.value.paths, 'parent_install_path')

// UI state
const directoryDrawerVisible = ref(false)

// Optimized formatted app path with null safety
const formatted_app_path = computed(() => {
  const installPath = install_path.value
  const appName = app_details.value.info.name

  if (!installPath || !appName) return ''

  return `${installPath}\\${appName}`
})

// Optimized config update function
function updateConfig(isCurrentUser: boolean) {
  const sourceConfig = isCurrentUser ? current_user : all_users

  // Update config values using the toRef references
  create_desktop_shortcut.value = sourceConfig.create_desktop_shortcut
  create_registry_key.value = sourceConfig.create_registry_key
  create_start_menu_shortcut.value = sourceConfig.create_start_menu_shortcut
  add_to_path.value = sourceConfig.add_to_path
  path_directory.value = '' // Reset path directory when switching modes
  install_path.value = sourceConfig.install_path
}

// Initialize with settings
current_user_only.value = settings_current_user_only
updateConfig(current_user_only.value)

// Optimized file selection
async function select_install_path() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    })
    if (selected) {
      install_path.value = String(selected)
    }
  } catch (error) {
    console.error('Failed to select install path:', error)
  }
}

// Simplified event handler
function handleInstallModeChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked
  current_user_only.value = checked
  updateConfig(checked)
}
</script>

<template>
  <Panel class="shadow-sm">
    <template #header>
      <div class="flex flex-col">
        <div class="flex items-center gap-1.5">
          <span class="mir-settings" />
          <h2 class="text-base font-medium">
            {{ t('ui.install.config.install_options') }}
          </h2>
        </div>
      </div>
    </template>

    <div class="space-y-2 p-1">
      <!-- Install Mode -->
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('cls.install.modes.self') }}</label>
        <div class="flex w-full items-center gap-2 rounded-lg px-2 py-1">
          <span class="text-sm">{{ t('cls.install.modes.all_users') }}</span>
          <ToggleSwitch v-model="current_user_only" @change="handleInstallModeChange" class="mx-1" />
          <span class="text-sm">{{ t('cls.install.modes.current_user') }}</span>
        </div>
      </div>

      <!-- Install Path -->
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('cls.install.config.install_path') }}</label>
        <div class="w-full">
          <div class="flex flex-1 gap-2">
            <InputText v-model="install_path" :placeholder="t('ui.select_placeholder.dir')" class="h-8 w-full text-sm"
              :invalid="pathError" @input="$emit('update:pathError', false)" />
            <Button class="h-8 w-36" severity="secondary" @click="select_install_path" icon="mir-folder_open"
              :label="t('g.browse')" />
          </div>

          <!-- Formatted App Path -->
          <div v-if="install_path && app_details.info.name" class="mt-1 text-xs text-gray-500">
            {{ t('ui.install.final_path') }}: {{ formatted_app_path }}
          </div>
        </div>
      </div>

      <!-- Shortcuts Section -->
      <div class="flex items-start gap-2">
        <label class="mt-1 w-24 text-sm font-medium">{{ t('g.option') }}</label>
        <div class="w-full">
          <div class="flex-1 space-y-1 rounded-lg p-1.5">
            <div class="flex items-center gap-2">
              <Checkbox v-model="create_desktop_shortcut" :binary="true" inputId="desktop_shortcut" />
              <label for="desktop_shortcut" class="text-sm">{{ t('cls.install.shortcuts.desktop') }}</label>
            </div>
            <div class="flex items-center gap-2">
              <Checkbox v-model="create_start_menu_shortcut" :binary="true" inputId="start_menu_shortcut" />
              <label for="start_menu_shortcut" class="text-sm">{{
                t('cls.install.shortcuts.start_menu')
              }}</label>
            </div>
            <div class="flex items-center gap-2">
              <Checkbox v-model="create_registry_key" :binary="true" inputId="registry_key" />
              <label for="registry_key" class="text-sm">{{ t('cls.install.shortcuts.registry_key') }}</label>
            </div>
            <div class="flex flex-col gap-1">
              <div class="flex items-center gap-2">
                <Checkbox v-model="add_to_path" :binary="true" inputId="add_to_path" />
                <label for="add_to_path" class="text-sm">{{ t('cls.install.shortcuts.add_to_path') }}</label>
              </div>
              <!-- PATH Directory Input - only shown when add_to_path is true -->
              <div v-if="add_to_path" class="ml-6 mt-1">
                <div class="flex gap-2">
                  <InputText v-model="path_directory" :placeholder="t('ui.select_placeholder.path_directory')"
                    class="h-8 w-full text-sm" />
                  <Button class="h-8 w-36" severity="secondary" @click="directoryDrawerVisible = true"
                    icon="mir-folder_open" :label="t('g.browse')" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Panel>

  <!-- Directory Selector Drawer -->
  <DirectorySelectorDrawer v-model:visible="directoryDrawerVisible" :zip-path="zip_path"
    @directory-select="path_directory = $event" />
</template>
