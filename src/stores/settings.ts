import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

// Core interfaces
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

interface Settings {
  // App preferences
  language: string
  theme: string
  minimize_to_tray_on_close: boolean

  // System info
  color: string
  debug: boolean
  elevated: boolean
  run_as_admin: boolean
  system_drive_letter: string
  username: string

  // Installation settings
  installation: Installation
}

// Store definition
export const useSettingsStore = defineStore('settings', {
  state: (): Settings => ({
    language: '',
    theme: '',
    minimize_to_tray_on_close: false,

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
      const result = await invoke('execute_command', {
        command: {
          name: 'LoadSettings',
        },
      })
      this.$patch(JSON.parse(result as string))
    },

    async saveSettings() {
      await invoke('execute_command', {
        command: {
          name: 'SaveSettings',
          settings: this.$state,
        },
      })
    },
  },
})
