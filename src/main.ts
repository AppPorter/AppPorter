import '@/assets/styles/main.css'
import { setupI18n } from '@/locales/i18n'
import Main from '@/Main.vue'
import router, { setupRouterGuards } from '@/router'
import { SettingsStore } from '@/stores/settings'
import { definePreset } from '@primeuix/themes'
import Aura from '@primeuix/themes/aura'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { Menu } from '@tauri-apps/api/menu'
import { TrayIcon, type TrayIconEvent } from '@tauri-apps/api/tray'
import { getCurrentWindow } from '@tauri-apps/api/window'
import Color from 'color'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import ConfirmationService from 'primevue/confirmationservice'
import ToastService from 'primevue/toastservice'
import { createApp } from 'vue'
import { EnvStore } from './stores/env'

document.addEventListener('contextmenu', (event) => event.preventDefault())

// Initialize window and app core components
export const window = await getCurrentWindow()
const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'

// Configure tray menu with basic actions
const createTrayMenu = (t: (key: string) => string) => {
  return Menu.new({
    items: [
      {
        id: 'open',
        text: t('basic.open'),
        action: () => {
          window.show()
          window.unminimize()
          window.setFocus()
        },
      },
      {
        id: 'quit',
        text: t('basic.quit'),
        action: () => invoke('exit'),
      },
    ],
  })
}

// Hide window instead of closing
window.onCloseRequested(async () => {
  window.hide()
})

const pinia = createPinia()
const app = createApp(Main)

app.use(pinia)

// Initialize settings first
const envStore = EnvStore()
await envStore.loadEnv()

const settingsStore = SettingsStore()
await settingsStore.loadSettings()

// Then initialize i18n with the loaded language
const i18n = setupI18n(settingsStore.language)
app.use(router).use(ToastService).use(ConfirmationService).use(i18n)
setupRouterGuards(router)

// Initialize theme
const UserColor = definePreset(Aura, {
  semantic: {
    primary: {
      50: Color(settingsStore.color).lighten(0.5).hex(),
      100: Color(settingsStore.color).lighten(0.4).hex(),
      200: Color(settingsStore.color).lighten(0.3).hex(),
      300: Color(settingsStore.color).lighten(0.2).hex(),
      400: Color(settingsStore.color).lighten(0.1).hex(),
      500: Color(settingsStore.color).hex(),
      600: Color(settingsStore.color).darken(0.1).hex(),
      700: Color(settingsStore.color).darken(0.2).hex(),
      800: Color(settingsStore.color).darken(0.3).hex(),
      900: Color(settingsStore.color).darken(0.4).hex(),
      950: Color(settingsStore.color).darken(0.5).hex(),
    },
  },
})

// Initial theme setup - using the settings store's functionality
await settingsStore.updateThemeMode()

app.use(PrimeVue, {
  theme: {
    preset: UserColor,
    options: {
      prefix: 'p',
      darkModeSelector: '.dark',
      cssLayer: false,
    },
  },
})

// Initialize tray if needed
if (settingsStore.minimize_to_tray_on_close) {
  const { t } = i18n.global
  const menu = await createTrayMenu(t)
  const trayOptions = {
    icon,
    menu,
    menuOnLeftClick: false,
    action: (event: TrayIconEvent) => {
      if (event.type === 'Click' && event.button === 'Left' && event.buttonState === 'Down') {
        window.show()
        window.unminimize()
        window.setFocus()
      }
    },
  }
  await TrayIcon.new(trayOptions).catch(console.error)
}

app.mount('#app')
