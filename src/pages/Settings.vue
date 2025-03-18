<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '../stores/settings'

const settings = useSettingsStore()
const isSettingsChanged = ref(false)
const initialSettings = ref({})
const { t } = useI18n()

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

// Store initial settings for change detection
onMounted(() => {
  initialSettings.value = JSON.parse(JSON.stringify(settings.$state))
})

// Save settings and reload app when changes are detected
watch(
  () => settings.$state,
  async (newValue) => {
    isSettingsChanged.value = JSON.stringify(newValue) !== JSON.stringify(initialSettings.value)
    if (isSettingsChanged.value) {
      await settings.saveSettings()
      window.location.reload()
    }
  },
  { deep: true }
)

async function selectInstallPath(userType: 'current_user' | 'all_users') {
  const selected = await open({
    directory: true,
  })

  if (selected) {
    if (userType === 'current_user') {
      settings.installation.current_user.install_path = selected as string
    } else {
      settings.installation.all_users.install_path = selected as string
    }
  }
}
</script>

<template>
  <div class="flex size-full flex-col">
    <Panel class="size-full overflow-auto">
      <!-- Header Section -->
      <template #header>
        <div class="flex items-center gap-2">
          <span class="mir settings text-xl"></span>
          <div>
            <h2 class="text-lg font-medium">{{ t('settings.title') }}</h2>
            <p class="mt-0.5 text-xs">{{ t('settings.description') }}</p>
          </div>
        </div>
      </template>

      <div class="mx-auto max-w-5xl space-y-2">
        <!-- Basic Settings -->
        <Panel>
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
                v-model="settings.theme"
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
        <Panel>
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

            <div class="flex flex-wrap gap-4 md:flex-nowrap">
              <!-- Current User Settings -->
              <Panel class="min-w-[250px] flex-1">
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
                    <ToggleSwitch
                      v-model="settings.installation.current_user.create_registry_key"
                    />
                  </div>
                  <div class="flex items-center justify-between">
                    <label>{{ t('settings.installation.start_menu') }}</label>
                    <ToggleSwitch
                      v-model="settings.installation.current_user.create_start_menu_shortcut"
                    />
                  </div>
                  <div class="space-y-2">
                    <label>{{ t('settings.installation.install_path') }}</label>
                    <div class="flex items-center gap-2">
                      <InputText
                        v-model="settings.installation.current_user.install_path"
                        :placeholder="t('settings.installation.install_path')"
                        class="h-9 flex-1 text-sm"
                      />
                      <Button
                        @click="selectInstallPath('current_user')"
                        severity="secondary"
                        class="h-9 px-4"
                        icon="mir folder_open"
                        :label="t('installation.browse')"
                      />
                    </div>
                  </div>
                </div>
              </Panel>

              <!-- All Users Settings -->
              <Panel class="min-w-[250px] flex-1">
                <template #header>
                  <div class="flex items-center gap-2">
                    <span class="mir group"></span>
                    <span>{{ t('settings.installation.all_users.title') }}</span>
                  </div>
                </template>
                <div class="space-y-4">
                  <div class="flex items-center justify-between">
                    <label>{{ t('settings.installation.desktop_shortcut') }}</label>
                    <ToggleSwitch
                      v-model="settings.installation.all_users.create_desktop_shortcut"
                    />
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
                  <div class="space-y-2">
                    <label>{{ t('settings.installation.install_path') }}</label>
                    <div class="flex items-center gap-2">
                      <InputText
                        v-model="settings.installation.all_users.install_path"
                        :placeholder="t('settings.installation.install_path')"
                        class="h-9 flex-1 text-sm"
                      />
                      <Button
                        @click="selectInstallPath('all_users')"
                        severity="secondary"
                        class="h-9 px-4"
                        icon="mir folder_open"
                        :label="t('installation.browse')"
                      />
                    </div>
                  </div>
                </div>
              </Panel>
            </div>
          </div>
        </Panel>
      </div>
    </Panel>
  </div>
</template>
