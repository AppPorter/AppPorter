import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

interface InstallSettings {
  create_desktop_shortcut: boolean
  create_registry_key: boolean
  create_start_menu_shortcut: boolean
  install_path: string
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
  first_run: boolean // Flag to check if this is the first run

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
  state: (): Settings => ({
    language: 'en',
    theme: 'system',
    minimize_to_tray_on_close: false,
    first_run: true, // Default to true

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
      },
      current_user: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: '',
      },
    },
  }),

  actions: {
    async loadSettings() {
      const result = await invoke<string>('execute_command', {
        command: { name: 'LoadSettings' },
      })
      this.$patch(JSON.parse(result))
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
  },
})
