<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import { computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '../stores/settings'

const settings = useSettingsStore()
const installationConfig = useInstallationConfigStore()

async function handleRegisterContextMenu() {
  await invoke('execute_command', {
    command: { name: 'RegisterContextMenu' },
  })
}

async function handleUnregisterContextMenu() {
  await invoke('execute_command', {
    command: { name: 'UnregisterContextMenu' },
  })
}

const isSettingsChanged = computed(() => {
  if (!settings.initialSettings) return false
  return JSON.stringify(settings.$state) !== JSON.stringify(settings.initialSettings)
})
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
  { label: t('common.theme.system'), value: 'system' },
  { label: t('common.theme.light'), value: 'light' },
  { label: t('common.theme.dark'), value: 'dark' },
]

// Save settings and reload app when changes are detected
watch(
  () => settings.$state,
  async () => {
    settings.updateBasicSettingsChanged()
    if (isSettingsChanged.value) {
      await settings.saveSettings()
    }
  },
  { deep: true }
)

function reloadApp() {
  window.location.reload()
}

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

// Add computed property to check if reload button should be disabled
const isReloadDisabled = computed(() => installationConfig.page === 'Progress')
</script>

<template>
  <div class="flex size-full flex-col">
    <Panel class="flex size-full flex-col overflow-auto">
      <!-- Header Section -->
      <template #header>
        <div class="flex items-center gap-2">
          <span class="mir-settings text-xl"></span>
          <div>
            <h2 class="text-lg font-medium">{{ t('settings.title') }}</h2>
            <p class="mt-0.5 text-xs">{{ t('settings.description') }}</p>
          </div>
        </div>
      </template>

      <div class="mx-auto w-full max-w-5xl space-y-4 overflow-y-auto px-4 pb-16">
        <!-- Basic Settings -->
        <Panel class="w-full">
          <template #header>
            <div class="flex items-center gap-2">
              <span class="mir-tune"></span>
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
            <div class="flex items-center justify-between">
              <label>{{ t('settings.basic.context_menu') }}</label>
              <div class="flex gap-2">
                <Button
                  @click="handleRegisterContextMenu"
                  severity="secondary"
                  class="h-9 px-4"
                  :label="t('settings.basic.register_context_menu')"
                />
                <Button
                  @click="handleUnregisterContextMenu"
                  severity="secondary"
                  class="h-9 px-4"
                  :label="t('settings.basic.unregister_context_menu')"
                />
              </div>
            </div>
          </div>
        </Panel>

        <!-- Installation Settings -->
        <Panel class="w-full">
          <template #header>
            <div class="flex items-center gap-2">
              <span class="mir-install_desktop"></span>
              <span>{{ t('settings.installation.title') }}</span>
            </div>
          </template>
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <label>{{ t('settings.installation.current_user_only') }}</label>
              <ToggleSwitch v-model="settings.installation.current_user_only" />
            </div>

            <div class="flex min-w-0 flex-col gap-4 lg:flex-row">
              <!-- Current User Settings -->
              <Panel class="min-w-0 flex-1">
                <template #header>
                  <div class="flex items-center gap-2">
                    <span class="mir-person"></span>
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
                    <div class="flex min-w-0 items-center gap-2">
                      <InputText
                        v-model="settings.installation.current_user.install_path"
                        :placeholder="t('settings.installation.install_path')"
                        class="h-9 min-w-0 flex-1 text-sm"
                      />
                      <Button
                        @click="selectInstallPath('current_user')"
                        severity="secondary"
                        class="h-9 px-4"
                        icon="mir-folder_open"
                        :label="t('installation.browse')"
                      />
                    </div>
                  </div>
                </div>
              </Panel>

              <!-- All Users Settings -->
              <Panel class="min-w-0 flex-1">
                <template #header>
                  <div class="flex items-center gap-2">
                    <span class="mir-group"></span>
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
                    <div class="flex min-w-0 items-center gap-2">
                      <InputText
                        v-model="settings.installation.all_users.install_path"
                        :placeholder="t('settings.installation.install_path')"
                        class="h-9 min-w-0 flex-1 text-sm"
                      />
                      <Button
                        @click="selectInstallPath('all_users')"
                        severity="secondary"
                        class="h-9 px-4"
                        icon="mir-folder_open"
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

  <!-- Fixed position reload button -->
  <div class="fixed bottom-4 right-10 z-40">
    <Button
      v-if="settings.isBasicSettingsChanged"
      @click="reloadApp"
      class="h-8 w-28 text-sm transition-all duration-200"
      severity="primary"
      :label="t('settings.reload')"
      icon="mir-refresh"
      :disabled="isReloadDisabled"
      v-tooltip.top="isReloadDisabled ? t('settings.reload_disabled_during_install') : ''"
    />
  </div>
</template>
