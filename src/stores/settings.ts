import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import Color from 'color'
import { defineStore } from 'pinia'

export interface Settings {
  first_run: boolean
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

export type LanguageType = 'en' | 'zh-hans' | 'zh-hant' | 'fr' | 'de' | 'es' | 'ja' | 'ko' | 'ru'
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
  state: (): Settings & { unlistenThemeColor?: (() => void) | null } => ({
    first_run: true,
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
    unlistenThemeColor: null,
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

    // Initialize theme system with color palette and monitoring
    async initializeTheme() {
      await this.updateThemeMode()
      await this.startThemeColorMonitoring()
    },

    // Generate color palette for PrimeVue theme
    generateColorPalette(): Record<string, string> {
      const baseColor = Color(this.color)
      return {
        50: baseColor.lighten(0.4).hex(),
        100: baseColor.lighten(0.3).hex(),
        200: baseColor.lighten(0.2).hex(),
        300: baseColor.lighten(0.1).hex(),
        400: baseColor.lighten(0.05).hex(),
        500: baseColor.hex(),
        600: baseColor.darken(0.05).hex(),
        700: baseColor.darken(0.1).hex(),
        800: baseColor.darken(0.2).hex(),
        900: baseColor.darken(0.3).hex(),
        950: baseColor.darken(0.4).hex(),
      }
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
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }

      env.isDarkMode = isDarkMode

      // Setup system theme change listener
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

      // Only add listener when theme is set to 'system'
      if (this.theme === 'system') {
        mediaQuery.addEventListener('change', (event: MediaQueryListEvent) => {
          env.isDarkMode = event.matches

          if (event.matches) {
            document.documentElement.classList.add('dark')
          } else {
            document.documentElement.classList.remove('dark')
          }
        })
      }
    },

    // Start listening for system color changes
    async startThemeColorMonitoring() {
      await invoke('execute_command', {
        command: { name: 'StartThemeMonitoring' },
      })

      const unlisten = await listen<string>('theme-color-changed', (event) => {
        this.color = event.payload
        // Update CSS custom properties when color changes
        const colorPalette = this.generateColorPalette()
        const root = document.documentElement
        Object.entries(colorPalette).forEach(([shade, colorValue]) => {
          root.style.setProperty(`--p-primary-${shade}`, colorValue as string)
        })
      })

      this.unlistenThemeColor = unlisten
    },

    // Stop listening for system color changes
    async stopThemeColorMonitoring() {
      await invoke('execute_command', {
        command: { name: 'StopThemeMonitoring' },
      })

      if (this.unlistenThemeColor) {
        this.unlistenThemeColor()
        this.unlistenThemeColor = null
      }
    },

    async acknowledgeFirstRun() {
      this.first_run = false
      await this.saveSettings()
    },
  },
})
