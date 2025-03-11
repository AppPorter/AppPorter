<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from './stores/settings'

const settings = useSettingsStore()
const isSettingsChanged = ref(false)
const initialSettings = ref({})
const { t } = useI18n()

// Computed property for theme binding
const currentTheme = computed({
  get: () => {
    if (settings.theme === true) return 'dark'
    if (settings.theme === false) return 'light'
    return 'system'
  },
  set: (value: string) => {
    if (value === 'dark') settings.theme = true
    else if (value === 'light') settings.theme = false
    else settings.theme = 'system'
  },
})

const saveSettings = async () => {
  await settings.saveSettings()
  window.location.reload()
}

const languageOptions = [
  { label: 'English', value: 'en' },
  { label: '中文', value: 'zh' },
  { label: 'Français', value: 'fr' },
  { label: 'Deutsch', value: 'de' },
  { label: 'Español', value: 'es' },
  { label: '日本語', value: 'ja' },
  { label: '한국어', value: 'ko' },
  { label: 'Русский', value: 'ru' },
]

const themeOptions = [
  { label: t('settings.theme.system'), value: 'system' },
  { label: t('settings.theme.light'), value: 'light' },
  { label: t('settings.theme.dark'), value: 'dark' },
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
          <h2 class="text-lg font-medium">{{ t('settings.title') }}</h2>
          <p class="text-xs mt-0.5">{{ t('settings.description') }}</p>
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
            <span>{{ t('settings.basic.title') }}</span>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label>{{ t('settings.basic.language') }}</label>
            <Select
              v-model="settings.language"
              :options="languageOptions"
              optionLabel="label"
              optionValue="value"
              class="w-48"
            />
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('settings.basic.theme') }}</label>
            <Select
              v-model="currentTheme"
              :options="themeOptions"
              optionLabel="label"
              optionValue="value"
              class="w-48"
            />
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('settings.basic.minimize_tray') }}</label>
            <ToggleSwitch v-model="settings.minimize_to_tray_on_close" />
          </div>
        </div>
      </Panel>

      <!-- Installation Settings -->
      <Panel header="Installation Settings">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir install_desktop"></span>
            <span>{{ t('settings.installation.title') }}</span>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label>{{ t('settings.installation.current_user_only') }}</label>
            <ToggleSwitch v-model="settings.installation.current_user_only" />
          </div>

          <div class="flex gap-4">
            <!-- Current User Settings -->
            <Panel class="flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir person"></span>
                  <span>{{ t('settings.installation.current_user.title') }}</span>
                </div>
              </template>
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label>{{ t('settings.installation.desktop_shortcut') }}</label>
                  <ToggleSwitch
                    v-model="settings.installation.current_user.create_desktop_shortcut"
                  />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('settings.installation.registry_key') }}</label>
                  <ToggleSwitch v-model="settings.installation.current_user.create_registry_key" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('settings.installation.start_menu') }}</label>
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
                  <span>{{ t('settings.installation.all_users.title') }}</span>
                </div>
              </template>
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label>{{ t('settings.installation.desktop_shortcut') }}</label>
                  <ToggleSwitch v-model="settings.installation.all_users.create_desktop_shortcut" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('settings.installation.registry_key') }}</label>
                  <ToggleSwitch v-model="settings.installation.all_users.create_registry_key" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('settings.installation.start_menu') }}</label>
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
        :label="t('settings.save_changes')"
        @click="saveSettings"
        severity="primary"
        :disabled="!isSettingsChanged"
      />
    </div>
  </Panel>
</template>
