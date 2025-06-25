import { setupI18n } from '@/locales/i18n'
import Main from '@/Main.vue'
import router, { setupRouterGuards } from '@/router'
import { SettingsStore } from '@/stores/settings'
import '@/styles/main.css'
import { definePreset } from '@primeuix/themes'
import Aura from '@primeuix/themes/aura'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { Menu } from '@tauri-apps/api/menu'
import { TrayIcon, type TrayIconEvent } from '@tauri-apps/api/tray'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import ConfirmationService from 'primevue/confirmationservice'
import ToastService from 'primevue/toastservice'
import { createApp } from 'vue'
import { EnvStore } from './stores/env'

document.addEventListener('contextmenu', (event) => event.preventDefault())

export const window = await getCurrentWindow()
const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'

export let trayIcon: void | TrayIcon

const createTrayMenu = (t: (key: string) => string) => {
  return Menu.new({
    items: [
      {
        id: 'open',
        text: t('g.open'),
        action: () => {
          window.show()
          window.unminimize()
          window.setFocus()
        },
      },
      {
        id: 'quit',
        text: t('g.exit'),
        action: () =>
          invoke('execute_command', {
            command: { name: 'Exit', code: 0 },
          }),
      },
    ],
  })
}

const pinia = createPinia()
const app = createApp(Main)

app.use(pinia)

const envStore = EnvStore()
await envStore.loadEnv()

const settingsStore = SettingsStore()
await settingsStore.loadSettings()

await envStore.setInitialSettings()

const i18n = setupI18n(settingsStore.language)
app.use(router).use(ToastService).use(ConfirmationService).use(i18n)
setupRouterGuards(router)

const UserColor = definePreset(Aura, {
  semantic: {
    primary: settingsStore.generateColorPalette(),
  },
})

await settingsStore.initializeTheme()

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
  trayIcon = await TrayIcon.new(trayOptions).catch(console.error)
}

app.mount('#app')
