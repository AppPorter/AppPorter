import { exec } from '@/exec'
import { defineStore } from 'pinia'
import type { Settings } from '#/Settings'
import Color from 'color'
import { listen } from '@tauri-apps/api/event'

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
      this.$patch(await exec<Settings>('LoadSettings'))
    },

    async saveSettings() {
      await exec('SaveSettings', {
        settings: this.$state,
      })
    },

    async initializeTheme() {
      await this.updateThemeMode()
      await this.startThemeColorMonitoring()
    },

    generateColorPalette(): Record<string, string> {
      const baseColor = Color(this.color)
      return {
        50: baseColor.lighten(0.5).hex(),
        100: baseColor.lighten(0.4).hex(),
        200: baseColor.lighten(0.3).hex(),
        300: baseColor.lighten(0.2).hex(),
        400: baseColor.lighten(0.1).hex(),
        500: baseColor.hex(),
        600: baseColor.darken(0.1).hex(),
        700: baseColor.darken(0.2).hex(),
        800: baseColor.darken(0.3).hex(),
        900: baseColor.darken(0.4).hex(),
        950: baseColor.darken(0.5).hex(),
      }
    },

    async updateBasicSettingsChanged() {
      const { GeneralStore } = await import('./general')
      const generalStore = GeneralStore()

      const currentBasicSettings = {
        language: this.language,
        theme: this.theme,
        minimize_to_tray_on_close: this.minimize_to_tray_on_close,
      }

      if (generalStore.initialSettings) {
        const initialBasicSettings = {
          language: generalStore.initialSettings.language,
          theme: generalStore.initialSettings.theme,
          minimize_to_tray_on_close: generalStore.initialSettings.minimize_to_tray_on_close,
        }
        generalStore.isBasicSettingsChanged =
          JSON.stringify(currentBasicSettings) !== JSON.stringify(initialBasicSettings)
      }
    },

    async updateThemeMode() {
      const { GeneralStore } = await import('./general')
      const generalStore = GeneralStore()

      const isDarkMode =
        this.theme === 'dark' ||
        (this.theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)

      if (isDarkMode) {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }

      generalStore.isDarkMode = isDarkMode

      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

      if (this.theme === 'system') {
        mediaQuery.addEventListener('change', (event: MediaQueryListEvent) => {
          generalStore.isDarkMode = event.matches

          if (event.matches) {
            document.documentElement.classList.add('dark')
          } else {
            document.documentElement.classList.remove('dark')
          }
        })
      }
    },

    async startThemeColorMonitoring() {
      await exec('StartThemeMonitoring')

      const unlisten = await listen<string>('theme-color-changed', (event) => {
        this.color = event.payload
        const colorPalette = this.generateColorPalette()
        const root = document.documentElement
        Object.entries(colorPalette).forEach(([shade, colorValue]) => {
          root.style.setProperty(`--p-primary-${shade}`, colorValue as string)
        })
      })

      this.unlistenThemeColor = unlisten
    },

    async stopThemeColorMonitoring() {
      await exec('StopThemeMonitoring')

      if (this.unlistenThemeColor) {
        this.unlistenThemeColor()
        this.unlistenThemeColor = null
      }
    },
  },
})
