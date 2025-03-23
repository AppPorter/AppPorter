import '@/assets/styles/main.css'
import { setupI18n } from '@/locales/i18n'
import Main from '@/Main.vue'
import router, { goTo, setupRouterGuards } from '@/router'
import { useSettingsStore } from '@/stores/settings'
import { definePreset } from '@primeuix/themes'
import Aura from '@primeuix/themes/aura'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Menu } from '@tauri-apps/api/menu'
import { TrayIcon, type TrayIconEvent } from '@tauri-apps/api/tray'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { exit } from '@tauri-apps/plugin-process'
import Color from 'color'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import ConfirmationService from 'primevue/confirmationservice'
import ToastService from 'primevue/toastservice'
import { createApp } from 'vue'
import { useInstallationConfigStore } from './stores/installation_config'

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
        text: t('system.menu.open'),
        action: () => {
          window.show()
          window.unminimize()
          window.setFocus()
        },
      },
      {
        id: 'quit',
        text: t('system.menu.quit'),
        action: () => exit(0),
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
const settingsStore = useSettingsStore()
await settingsStore.loadSettings()

// Then initialize i18n with the loaded language
const i18n = setupI18n(settingsStore.language)
app.use(router).use(ToastService).use(ConfirmationService).use(i18n)
setupRouterGuards(router)

// Initialize theme
const UserColor = definePreset(Aura, {
  semantic: {
    primary: {
      50: Color(settingsStore.color).lightness(95).hex(),
      100: Color(settingsStore.color).lightness(90).hex(),
      200: Color(settingsStore.color).lightness(80).hex(),
      300: Color(settingsStore.color).lightness(70).hex(),
      400: Color(settingsStore.color).lightness(60).hex(),
      500: Color(settingsStore.color).lightness(50).hex(),
      600: Color(settingsStore.color).lightness(40).hex(),
      700: Color(settingsStore.color).lightness(30).hex(),
      800: Color(settingsStore.color).lightness(20).hex(),
      900: Color(settingsStore.color).lightness(10).hex(),
      950: Color(settingsStore.color).lightness(5).hex(),
    },
  },
})

// Update theme handler
const updateTheme = (theme: 'system' | 'light' | 'dark') => {
  if (
    theme === 'dark' ||
    (theme === 'system' && globalThis.window.matchMedia('(prefers-color-scheme: dark)').matches)
  ) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

// Initial theme setup
updateTheme(settingsStore.theme)

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

await listen('install', (event) => {
  useInstallationConfigStore().zip_path = event.payload as string
  goTo('/Installation/Config')
})

invoke('execute_command', {
  command: {
    name: 'Cli',
  },
})

app.mount('#app')
