import '@/assets/main.css'

import { createPinia } from 'pinia'
import { createApp } from 'vue'

import Main from '@/Main.vue'
import router, { setupRouterGuards } from '@/plugins/router'
import { useSettingsStore } from '@/stores/settings'

import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config'
import ConfirmationService from 'primevue/confirmationservice'
import ToastService from 'primevue/toastservice'

import { definePreset } from '@primevue/themes'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { Menu } from '@tauri-apps/api/menu'
import { TrayIcon, type TrayIconEvent } from '@tauri-apps/api/tray'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { exit } from '@tauri-apps/plugin-process'

import Color from 'color'

document.addEventListener('contextmenu', (event) => event.preventDefault())

// Initialize window and app core components
export const window = await getCurrentWindow()
const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'

// Configure tray menu with basic actions
const menu = await Menu.new({
  items: [
    {
      id: 'open',
      text: 'Open',
      action: () => {
        window.show()
        window.unminimize()
        window.setFocus()
      },
    },
    {
      id: 'quit',
      text: 'Quit',
      action: () => exit(0),
    },
  ],
})

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

// Hide window instead of closing
window.onCloseRequested(async () => {
  window.hide()
})

// App setup
const pinia = createPinia()
const app = createApp(Main)

app.use(pinia).use(router).use(ToastService).use(ConfirmationService)
setupRouterGuards(router)

// Initialize settings and theme
const settingsStore = useSettingsStore()
await settingsStore.loadSettings()

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

app.use(PrimeVue, {
  theme: {
    preset: UserColor,
    options: {
      prefix: 'p',
      darkModeSelector: 'system',
      cssLayer: false,
    },
  },
})

// Initialize tray if needed
if (settingsStore.minimize_to_tray_on_close) {
  await TrayIcon.new(trayOptions).catch(console.error)
}

app.mount('#app')
