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

interface CopyOnly {
  install_path: string
  add_to_path: boolean
}

export type ThemeType = 'system' | 'light' | 'dark'
export type LanguageType = 'en' | 'zh' | 'fr' | 'de' | 'es' | 'ja' | 'ko' | 'ru'

interface Settings {
  // App preferences
  language: LanguageType
  theme: ThemeType
  minimize_to_tray_on_close: boolean
  context_menu: boolean
  auto_startup: boolean
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
  copy_only: CopyOnly
}

// Store definition
export const useSettingsStore = defineStore('settings', {
  state: (): Settings & { initialSettings: Settings | null; isDarkMode: boolean } => ({
    language: 'en',
    theme: 'system',
    minimize_to_tray_on_close: false,
    context_menu: false,
    auto_startup: false,
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
    copy_only: {
      install_path: '',
      add_to_path: false,
    },
    initialSettings: null,
    isDarkMode: window.matchMedia('(prefers-color-scheme: dark)').matches,
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

    // Update theme mode and apply changes to DOM based on current theme setting
    updateThemeMode() {
      // Get dark mode status based on current theme setting
      const isDarkMode =
        this.theme === 'dark' ||
        (this.theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)

      // Update DOM
      if (isDarkMode) {
        console.log('Dark mode enabled')
        document.documentElement.classList.add('dark')
      } else {
        console.log('Light mode enabled')
        document.documentElement.classList.remove('dark')
      }

      // Update store state
      this.isDarkMode = isDarkMode

      // Setup system theme change listener
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

      // Remove any existing listener first to prevent duplicates
      mediaQuery.removeEventListener('change', this.handleSystemThemeChange)

      // Only add listener when theme is set to 'system'
      if (this.theme === 'system') {
        mediaQuery.addEventListener('change', (event: MediaQueryListEvent) => {
          // Update isDarkMode and DOM when system theme changes
          this.isDarkMode = event.matches

          if (event.matches) {
            document.documentElement.classList.add('dark')
          } else {
            document.documentElement.classList.remove('dark')
          }
        })
      }
    },
  },
})
