<script setup lang="ts">
import { EnvStore } from '@/stores/env'
import { InstallConfigStore } from '@/stores/install_config'
import { SettingsStore } from '@/stores/settings'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import Chip from 'primevue/chip'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import Select from 'primevue/select'
import ToggleSwitch from 'primevue/toggleswitch'
import { computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const settings = SettingsStore()
const env = EnvStore()
const installConfig = InstallConfigStore()

const isSettingsChanged = computed(() => {
  if (!env.initialSettings) return false
  return JSON.stringify(settings.$state) !== JSON.stringify(env.initialSettings)
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
  { label: t('cls.theme.system'), value: 'system' },
  { label: t('cls.theme.light'), value: 'light' },
  { label: t('cls.theme.dark'), value: 'dark' },
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

async function selectAppInstallPath(userType: 'current_user' | 'all_users') {
  const selected = await open({
    directory: true,
  })

  if (selected) {
    if (userType === 'current_user') {
      settings.app_install.current_user.install_path = selected as string
    } else {
      settings.app_install.all_users.install_path = selected as string
    }
  }
}

async function selectLibInstallPath() {
  const selected = await open({
    directory: true,
  })
  if (selected) {
    settings.lib_install.install_path = selected as string
  }
}

// Watch for context_menu changes and register/unregister accordingly
watch(
  () => settings.context_menu,
  async (newValue, oldValue) => {
    if (newValue === oldValue) return

    if (newValue) {
      await invoke('execute_command', {
        command: { name: 'RegisterContextMenu' },
      })
    } else {
      await invoke('execute_command', {
        command: { name: 'UnregisterContextMenu' },
      })
    }
  }
)

// Watch for auto_startup changes and set/remove startup accordingly
watch(
  () => settings.auto_startup,
  async (newValue, oldValue) => {
    if (newValue === oldValue) return

    if (newValue) {
      await invoke('execute_command', {
        command: { name: 'SetStartup' },
      })
    } else {
      await invoke('execute_command', {
        command: { name: 'RemoveStartup' },
      })
    }
  }
)

// Add computed property to check if reload button should be disabled
const isReloadDisabled = computed(() => installConfig.page === 'Install_App_Progress' || installConfig.page === 'Install_Lib_Progress')

// Get GitHub icon based on current theme and isDarkMode from settings store
const githubIcon = computed(() => {
  // Use import.meta.url to ensure proper path resolution
  return env.isDarkMode
    ? new URL('../assets/github-white.svg', import.meta.url).href
    : new URL('../assets/github-black.svg', import.meta.url).href
})
</script>

<template>
  <div class="flex size-full flex-col">
    <div class="mx-auto w-full max-w-5xl space-y-4 overflow-y-auto px-4 pt-2">
      <!-- Basic Settings -->
      <Panel class="w-full">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir-tune text-xl"></span>
            <h3 class="text-base font-medium">{{ t('ui.settings.basic') }}</h3>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label>{{ t('cls.settings.basic.language') }}</label>
            <Select v-model="settings.language" :options="languageOptions" optionLabel="label" optionValue="value"
              class="w-48" size="small" />
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('cls.settings.basic.theme') }}</label>
            <Select v-model="settings.theme" :options="themeOptions" optionLabel="label" optionValue="value"
              class="w-48" size="small" />
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('cls.settings.basic.minimize_tray') }}</label>
            <ToggleSwitch v-model="settings.minimize_to_tray_on_close" />
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('cls.settings.basic.context_menu') }}</label>
            <ToggleSwitch v-model="settings.context_menu" />
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('cls.settings.basic.auto_startup') }}</label>
            <ToggleSwitch v-model="settings.auto_startup" />
          </div>
        </div>
      </Panel>

      <!-- App Install Settings -->
      <Panel class="w-full">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir-install_desktop text-xl"></span>
            <h3 class="text-base font-medium">{{ t('ui.settings.install.app') }}</h3>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label>{{ t('cls.install.modes.self') }}</label>
            <ToggleSwitch v-model="settings.app_install.current_user_only" />
          </div>

          <div class="flex min-w-0 flex-col gap-4 lg:flex-row">
            <!-- Current User Settings -->
            <Panel class="min-w-0 flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir-person text-lg"></span>
                  <h4 class="text-sm font-medium">{{ t('cls.install.modes.current_user') }}</h4>
                </div>
              </template>
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.desktop_shortcut') }}</label>
                  <ToggleSwitch v-model="settings.app_install.current_user.create_desktop_shortcut" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.registry_key') }}</label>
                  <ToggleSwitch v-model="settings.app_install.current_user.create_registry_key" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.start_menu_shortcut') }}</label>
                  <ToggleSwitch v-model="settings.app_install.current_user.create_start_menu_shortcut" />
                </div>
                <div class="space-y-2">
                  <label>{{ t('cls.install.config.install_path') }}</label>
                  <div class="flex min-w-0 items-center gap-2">
                    <InputText v-model="settings.app_install.current_user.install_path"
                      :placeholder="t('cls.install.config.install_path')" class="h-9 min-w-0 flex-1 text-sm" />
                    <Button @click="selectAppInstallPath('current_user')" severity="secondary" class="h-9 px-4"
                      icon="mir-folder_open" :label="t('g.browse')" />
                  </div>
                </div>
              </div>
            </Panel>

            <!-- All Users Settings -->
            <Panel class="min-w-0 flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir-group text-lg"></span>
                  <h4 class="text-sm font-medium">{{ t('cls.install.modes.all_users') }}</h4>
                </div>
              </template>
              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.desktop_shortcut') }}</label>
                  <ToggleSwitch v-model="settings.app_install.all_users.create_desktop_shortcut" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.registry_key') }}</label>
                  <ToggleSwitch v-model="settings.app_install.all_users.create_registry_key" />
                </div>
                <div class="flex items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.start_menu_shortcut') }}</label>
                  <ToggleSwitch v-model="settings.app_install.all_users.create_start_menu_shortcut" />
                </div>
                <div class="space-y-2">
                  <label>{{ t('cls.install.config.install_path') }}</label>
                  <div class="flex min-w-0 items-center gap-2">
                    <InputText v-model="settings.app_install.all_users.install_path"
                      :placeholder="t('cls.install.config.install_path')" class="h-9 min-w-0 flex-1 text-sm" />
                    <Button @click="selectAppInstallPath('all_users')" severity="secondary" class="h-9 px-4"
                      icon="mir-folder_open" :label="t('g.browse')" />
                  </div>
                </div>
              </div>
            </Panel>
          </div>
        </div>
      </Panel>

      <!-- Lib Install Settings -->
      <Panel class="w-full">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir-folder_copy text-xl"></span>
            <h3 class="text-base font-medium">{{ t('ui.settings.install.lib') }}</h3>
          </div>
        </template>
        <div class="space-y-4">
          <div class="space-y-2">
            <label>{{ t('cls.install.config.install_path') }}</label>
            <div class="flex min-w-0 items-center gap-2">
              <InputText v-model="settings.lib_install.install_path" :placeholder="t('cls.install.config.install_path')"
                class="h-9 min-w-0 flex-1 text-sm" />
              <Button @click="selectLibInstallPath" severity="secondary" class="h-9 px-4" icon="mir-folder_open"
                :label="t('g.browse')" />
            </div>
          </div>
          <div class="flex items-center justify-between">
            <label>{{ t('cls.install.shortcuts.add_to_path') }}</label>
            <ToggleSwitch v-model="settings.lib_install.add_to_path" />
          </div>
        </div>
      </Panel>

      <div class="flex flex-col items-center justify-center space-y-1">
        <img src="@/assets/appporter.svg" class="size-24" alt="AppPorter" />
        <h1 class="whitespace-nowrap px-6 text-2xl font-semibold">AppPorter</h1>
        <p class="text-lg">v0.2.0</p>
        <a href="https://github.com/AppPorter/AppPorter" target="_blank">
          <Chip label="GitHub" class="flex items-center">
            <template #icon>
              <img :src="githubIcon" class="mr-1 size-4" alt="GitHub" />
            </template>
          </Chip>
        </a>
      </div>
    </div>
  </div>

  <!-- Fixed position reload button -->
  <div class="fixed bottom-4 right-10 z-40">
    <Button v-if="env.isBasicSettingsChanged" @click="reloadApp" class="h-8 w-28 text-sm transition-all duration-200"
      severity="primary" :label="t('g.reload')" icon="mir-refresh" :disabled="isReloadDisabled"
      v-tooltip.top="isReloadDisabled ? t('ui.settings.reload_disabled') : ''" />
  </div>
</template>
