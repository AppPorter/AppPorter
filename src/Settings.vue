<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useSettingsStore } from './stores/settings'

const settings = useSettingsStore()
const isSettingsChanged = ref(false)
const initialSettings = ref({})

const saveSettings = async () => {
  await settings.saveSettings()
  window.location.reload()
}

const languageOptions = [
  { label: 'English', value: 'en' },
  { label: '中文', value: 'zh' },
]

const themeOptions = [
  { label: 'System', value: 'system' },
  { label: 'Light', value: 'light' },
  { label: 'Dark', value: 'dark' },
]

onMounted(() => {
  initialSettings.value = JSON.parse(JSON.stringify(settings.$state))
})

watch(
  () => settings.$state,
  (newValue) => {
    isSettingsChanged.value = JSON.stringify(newValue) !== JSON.stringify(initialSettings.value)
  },
  { deep: true }
)
</script>

<template>
  <Panel class="h-full w-full overflow-auto">
    <!-- Header Section -->
    <template #header>
      <div class="flex items-center gap-2">
        <div class="p-1 rounded-md">
          <span class="mir settings text-xl"></span>
        </div>
        <div>
          <h2 class="text-lg font-medium text-surface-900 dark:text-surface-0">Settings</h2>
          <p class="text-xs text-surface-600 dark:text-surface-400 mt-0.5">
            Manage your application preferences
          </p>
        </div>
      </div>
    </template>

    <!-- Content Section -->
    <div class="space-y-2">
      <!-- Basic Settings -->
      <Panel header="Basic Settings">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir tune"></span>
            <span>Basic Settings</span>
          </div>
        </template>
        <div class="*:h-12">
          <div class="flex items-center justify-between">
            <label>Language</label>
            <Select
              v-model="settings.language"
              :options="languageOptions"
              optionLabel="label"
              optionValue="value"
              class="w-48"
            />
          </div>
          <div class="flex items-center justify-between">
            <label>Theme</label>
            <Select
              v-model="settings.theme"
              :options="themeOptions"
              optionLabel="label"
              optionValue="value"
              class="w-48"
            />
          </div>
          <div class="flex items-center justify-between">
            <label>Minimize to Tray on Close</label>
            <ToggleSwitch v-model="settings.minimize_to_tray_on_close" />
          </div>
        </div>
      </Panel>

      <!-- Installation Settings -->
      <Panel header="Installation Settings">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir install_desktop"></span>
            <span>Installation Settings</span>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label>Current User Only</label>
            <ToggleSwitch v-model="settings.installation.current_user_only" />
          </div>

          <div class="flex gap-4">
            <!-- Current User Settings -->
            <Panel class="flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir person"></span>
                  <span>Current User Settings</span>
                </div>
              </template>
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label>Create Desktop Shortcut</label>
                  <ToggleSwitch
                    v-model="settings.installation.current_user.create_desktop_shortcut"
                  />
                </div>
                <div class="flex items-center justify-between">
                  <label>Create Registry Key</label>
                  <ToggleSwitch v-model="settings.installation.current_user.create_registry_key" />
                </div>
                <div class="flex items-center justify-between">
                  <label>Create Start Menu Shortcut</label>
                  <ToggleSwitch
                    v-model="settings.installation.current_user.create_start_menu_shortcut"
                  />
                </div>
              </div>
            </Panel>

            <!-- All Users Settings -->
            <Panel class="flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir group"></span>
                  <span>All Users Settings</span>
                </div>
              </template>
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label>Create Desktop Shortcut</label>
                  <ToggleSwitch v-model="settings.installation.all_users.create_desktop_shortcut" />
                </div>
                <div class="flex items-center justify-between">
                  <label>Create Registry Key</label>
                  <ToggleSwitch v-model="settings.installation.all_users.create_registry_key" />
                </div>
                <div class="flex items-center justify-between">
                  <label>Create Start Menu Shortcut</label>
                  <ToggleSwitch
                    v-model="settings.installation.all_users.create_start_menu_shortcut"
                  />
                </div>
              </div>
            </Panel>
          </div>
        </div>
      </Panel>
    </div>

    <!-- Footer Section -->
    <div class="pt-4 flex justify-end">
      <Button
        icon="mir save"
        label="Save Changes"
        @click="saveSettings"
        severity="primary"
        :disabled="!isSettingsChanged"
      />
    </div>
  </Panel>
</template>
