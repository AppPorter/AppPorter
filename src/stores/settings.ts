import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

interface InstallSettings {
  create_desktop_shortcut: boolean
  create_registry_key: boolean
  create_start_menu_shortcut: boolean
  install_path: string
  add_to_path: boolean
}

interface Installation {
  current_user_only: boolean
  all_users: InstallSettings
  current_user: InstallSettings
}

export type ThemeType = 'system' | 'light' | 'dark'
export type LanguageType = 'en' | 'zh' | 'fr' | 'de' | 'es' | 'ja' | 'ko' | 'ru'

interface Settings {
  // App preferences
  language: LanguageType
  theme: ThemeType
  minimize_to_tray_on_close: boolean
  context_menu: boolean
  first_run: boolean
  isBasicSettingsChanged: boolean

  // System info
  color: string
  debug: boolean
  elevated: boolean
  run_as_admin: boolean
  system_drive_letter: string
  username: string

  installation: Installation
}

// Store definition
export const useSettingsStore = defineStore('settings', {
  state: (): Settings & { initialSettings: Settings | null } => ({
    language: 'en',
    theme: 'system',
    minimize_to_tray_on_close: false,
    context_menu: false,
    first_run: true,
    isBasicSettingsChanged: false,

    color: '',
    debug: false,
    elevated: false,
    run_as_admin: false,
    system_drive_letter: '',
    username: '',

    installation: {
      current_user_only: false,
      all_users: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: '',
        add_to_path: false,
      },
      current_user: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: '',
        add_to_path: false,
      },
    },

    initialSettings: null,
  }),

  actions: {
    async loadSettings() {
      const result = await invoke<string>('execute_command', {
        command: { name: 'LoadSettings' },
      })
      const settings = JSON.parse(result)
      this.$patch(settings)
      this.initialSettings = JSON.parse(JSON.stringify(settings))
    },

    async saveSettings() {
      await invoke('execute_command', {
        command: {
          name: 'SaveSettings',
          settings: this.$state,
        },
      })
    },

    async acknowledgeFirstRun() {
      this.first_run = false
      await this.saveSettings()
    },

    updateBasicSettingsChanged() {
      const currentBasicSettings = {
        language: this.language,
        theme: this.theme,
        minimize_to_tray_on_close: this.minimize_to_tray_on_close,
      }

      if (this.initialSettings) {
        const initialBasicSettings = {
          language: this.initialSettings.language,
          theme: this.initialSettings.theme,
          minimize_to_tray_on_close: this.initialSettings.minimize_to_tray_on_close,
        }
        this.isBasicSettingsChanged =
          JSON.stringify(currentBasicSettings) !== JSON.stringify(initialBasicSettings)
      }
    },
  },
})
