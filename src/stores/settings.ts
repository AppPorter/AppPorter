import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

export interface Settings {
  language: LanguageType
  theme: ThemeType
  minimize_to_tray_on_close: boolean
  context_menu: boolean
  auto_startup: boolean
  color: string
  run_as_admin: boolean
  app_install: AppInstall
  tool_install: ToolInstall
}

export type LanguageType = 'en' | 'zh' | 'fr' | 'de' | 'es' | 'ja' | 'ko' | 'ru'
export type ThemeType = 'system' | 'light' | 'dark'

interface AppInstall {
  current_user_only: boolean
  all_users: InstallSettings
  current_user: InstallSettings
}

interface InstallSettings {
  create_desktop_shortcut: boolean
  create_registry_key: boolean
  create_start_menu_shortcut: boolean
  install_path: string
  add_to_path: boolean
}

interface ToolInstall {
  install_path: string
  add_to_path: boolean
}

// Store definition
export const SettingsStore = defineStore('settings', {
  state: (): Settings => ({
    language: 'en',
    theme: 'system',
    minimize_to_tray_on_close: false,
    context_menu: false,
    auto_startup: false,
    color: '',
    run_as_admin: false,

    app_install: {
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
    tool_install: {
      install_path: '',
      add_to_path: false,
    },
  }),

  actions: {
    async loadSettings() {
      const result = await invoke<string>('execute_command', {
        command: { name: 'LoadSettings' },
      })
      const settings = JSON.parse(result)
      this.$patch(settings)
    },

    async saveSettings() {
      await invoke('execute_command', {
        command: {
          name: 'SaveSettings',
          settings: this.$state,
        },
      })
    },

    async updateBasicSettingsChanged() {
      const { EnvStore } = await import('./env')
      const env = EnvStore()

      const currentBasicSettings = {
        language: this.language,
        theme: this.theme,
        minimize_to_tray_on_close: this.minimize_to_tray_on_close,
      }

      if (env.initialSettings) {
        const initialBasicSettings = {
          language: env.initialSettings.language,
          theme: env.initialSettings.theme,
          minimize_to_tray_on_close: env.initialSettings.minimize_to_tray_on_close,
        }
        env.isBasicSettingsChanged =
          JSON.stringify(currentBasicSettings) !== JSON.stringify(initialBasicSettings)
      }
    },

    // Update theme mode and apply changes to DOM based on current theme setting
    async updateThemeMode() {
      const { EnvStore } = await import('./env')
      const env = EnvStore()

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

      env.isDarkMode = isDarkMode

      // Setup system theme change listener
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

      // Remove any existing listener first to prevent duplicates
      mediaQuery.removeEventListener('change', this.handleSystemThemeChange)

      // Only add listener when theme is set to 'system'
      if (this.theme === 'system') {
        mediaQuery.addEventListener('change', (event: MediaQueryListEvent) => {
          // Update isDarkMode and DOM when system theme changes
          env.isDarkMode = event.matches

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
