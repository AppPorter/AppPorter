<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog'
import Button from 'primevue/button'
import Chip from 'primevue/chip'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import Select from 'primevue/select'
import SelectButton from 'primevue/selectbutton'
import ToggleSwitch from 'primevue/toggleswitch'
import { computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { exec } from '@/exec'
import { generalStore, settingsStore } from '@/main'

const isSettingsChanged = computed(() => {
  if (!generalStore.initialSettings) return false
  return JSON.stringify(settingsStore.$state) !== JSON.stringify(generalStore.initialSettings)
})
const { t } = useI18n()

const languageOptions = [
  { label: 'English', value: 'en' },
  { label: '简体中文', value: 'zh-hans' },
  { label: '繁体中文', value: 'zh-hant' },
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

const installModeOptions = [
  { label: t('cls.install.modes.current_user'), value: true },
  { label: t('cls.install.modes.all_users'), value: false },
]

watch(
  () => settingsStore.$state,
  async () => {
    settingsStore.updateBasicSettingsChanged()
    if (isSettingsChanged.value) {
      await settingsStore.saveSettings()
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
      settingsStore.app_install.current_user.install_path = selected as string
    } else {
      settingsStore.app_install.all_users.install_path = selected as string
    }
  }
}

async function selectToolInstallPath() {
  const selected = await open({
    directory: true,
  })
  if (selected) {
    settingsStore.tool_install.install_path = selected as string
  }
}

watch(
  () => settingsStore.context_menu,
  async (newValue, oldValue) => {
    if (newValue === oldValue) return

    if (newValue) {
      await exec('RegisterContextMenu')
    } else {
      await exec('UnregisterContextMenu')
    }
  }
)

watch(
  () => settingsStore.auto_startup,
  async (newValue, oldValue) => {
    if (newValue === oldValue) return

    if (newValue) {
      await exec('SetStartup')
    } else {
      await exec('RemoveStartup')
    }
  }
)

const isReloadDisabled = computed(() => generalStore.page === 'Install_App_Progress' || generalStore.page === 'Install_Tool_Progress')

const githubIcon = computed(() => {
  return generalStore.isDarkMode
    ? new URL('../assets/github-white.svg', import.meta.url).href
    : new URL('../assets/github-black.svg', import.meta.url).href
})
</script>

<template>
  <div class="flex size-full flex-col">
    <div class="mx-auto w-full max-w-5xl space-y-2 overflow-y-auto px-4 pt-2">
      <Panel class="w-full">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir-tune text-xl"></span>
            <h3 class="font-medium">{{ t('ui.settingsStore.basic') }}</h3>
          </div>
        </template>
        <div class="space-y-2">
          <div class="flex h-9 items-center justify-between">
            <label>{{ t('cls.settingsStore.basic.language') }}</label>
            <Select v-model="settingsStore.language" :options="languageOptions" optionLabel="label" optionValue="value"
              class="w-48" size="small" />
          </div>
          <div class="flex h-9 items-center justify-between">
            <label>{{ t('cls.settingsStore.basic.theme') }}</label>
            <Select v-model="settingsStore.theme" :options="themeOptions" optionLabel="label" optionValue="value"
              class="w-48" size="small" />
          </div>
          <div class="flex h-9 items-center justify-between">
            <label>{{ t('cls.settingsStore.basic.minimize_tray') }}</label>
            <ToggleSwitch v-model="settingsStore.minimize_to_tray_on_close" />
          </div>
          <div class="flex h-9 items-center justify-between pl-4"
            :class="{ 'opacity-60': !settingsStore.minimize_to_tray_on_close }">
            <div class="flex items-center gap-2">
              <div class="h-4 w-px bg-gray-300 dark:bg-gray-600"></div>
              <label class="text-sm">{{ t('cls.settingsStore.basic.auto_startup') }}</label>
            </div>
            <ToggleSwitch v-model="settingsStore.auto_startup" :disabled="!settingsStore.minimize_to_tray_on_close" />
          </div>
          <div class="flex h-9 items-center justify-between">
            <label>{{ t('cls.settingsStore.basic.context_menu') }}</label>
            <ToggleSwitch v-model="settingsStore.context_menu" />
          </div>
        </div>
      </Panel>

      <Panel class="w-full">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir-install_desktop text-xl"></span>
            <h3 class="font-medium">{{ t('ui.settingsStore.install.app') }}</h3>
          </div>
        </template>
        <div class="space-y-2">
          <div class="flex h-9 items-center justify-between">
            <label>{{ t('ui.settingsStore.default_install_mode') }}</label>
            <SelectButton v-model="settingsStore.app_install.current_user_only" :options="installModeOptions"
              optionLabel="label" optionValue="value" :allowEmpty="false" size="small" />
          </div>

          <div class="flex min-w-0 flex-col gap-4 lg:flex-row">
            <Panel class="min-w-0 flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir-person text-lg"></span>
                  <h4 class="font-medium">{{ t('cls.install.modes.current_user') }}</h4>
                </div>
              </template>
              <div class="space-y-2">
                <div class="flex h-9 items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.desktop') }}</label>
                  <ToggleSwitch v-model="settingsStore.app_install.current_user.create_desktop_shortcut" />
                </div>
                <div class="flex h-9 items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.registry_key') }}</label>
                  <ToggleSwitch v-model="settingsStore.app_install.current_user.create_registry_key" />
                </div>
                <div class="flex h-9 items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.start_menu') }}</label>
                  <ToggleSwitch v-model="settingsStore.app_install.current_user.create_start_menu_shortcut" />
                </div>
                <div>
                  <label class="flex h-9 items-center justify-between">{{ t('cls.install.config.install_path')
                  }}</label>
                  <div class="flex min-w-0 items-center gap-2">
                    <InputText v-model="settingsStore.app_install.current_user.install_path"
                      :placeholder="t('cls.install.config.install_path')" class="h-9 min-w-0 flex-1 text-sm" />
                    <Button @click="selectAppInstallPath('current_user')" severity="secondary" class="h-9 px-4"
                      icon="mir-folder_open" :label="t('g.browse')" />
                  </div>
                </div>
              </div>
            </Panel>

            <Panel class="min-w-0 flex-1">
              <template #header>
                <div class="flex items-center gap-2">
                  <span class="mir-group text-lg"></span>
                  <h4 class="font-medium">{{ t('cls.install.modes.all_users') }}</h4>
                </div>
              </template>
              <div class="space-y-2">
                <div class="flex h-9 items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.desktop') }}</label>
                  <ToggleSwitch v-model="settingsStore.app_install.all_users.create_desktop_shortcut" />
                </div>
                <div class="flex h-9 items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.registry_key') }}</label>
                  <ToggleSwitch v-model="settingsStore.app_install.all_users.create_registry_key" />
                </div>
                <div class="flex h-9 items-center justify-between">
                  <label>{{ t('cls.install.shortcuts.start_menu') }}</label>
                  <ToggleSwitch v-model="settingsStore.app_install.all_users.create_start_menu_shortcut" />
                </div>
                <div>
                  <label class="flex h-9 items-center justify-between">{{ t('cls.install.config.install_path')
                    }}</label>
                  <div class="flex min-w-0 items-center gap-2">
                    <InputText v-model="settingsStore.app_install.all_users.install_path"
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

      <Panel class="w-full">
        <template #header>
          <div class="flex items-center gap-2">
            <span class="mir-folder_copy text-xl"></span>
            <h3 class="font-medium">{{ t('ui.settingsStore.install.tool') }}</h3>
          </div>
        </template>
        <div class="space-y-2">
          <div class="">
            <label class="flex h-9 items-center justify-between">{{ t('cls.install.config.install_path') }}</label>
            <div class="flex min-w-0 items-center gap-2">
              <InputText v-model="settingsStore.tool_install.install_path"
                :placeholder="t('cls.install.config.install_path')" class="h-9 min-w-0 flex-1 text-sm" />
              <Button @click="selectToolInstallPath" severity="secondary" class="h-9 px-4" icon="mir-folder_open"
                :label="t('g.browse')" />
            </div>
          </div>
          <div class="flex h-9 items-center justify-between">
            <label>{{ t('cls.install.shortcuts.add_to_path') }}</label>
            <ToggleSwitch v-model="settingsStore.tool_install.add_to_path" />
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

  <div class="fixed bottom-4 right-10 z-40">
    <Button v-if="generalStore.isBasicSettingsChanged" @click="reloadApp"
      class="h-8 w-28 text-sm transition-all duration-200" severity="primary" :label="t('g.reload')" icon="mir-refresh"
      :disabled="isReloadDisabled" v-tooltip.top="isReloadDisabled ? t('ui.settingsStore.reload_disabled') : ''" />
  </div>
</template>
