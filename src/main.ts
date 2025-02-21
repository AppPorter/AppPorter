// Style imports
import '@/assets/index.css'

// Vue core imports
import { createPinia } from 'pinia'
import { createApp, ref, type Ref } from 'vue'

// App components and plugins
import Main from '@/Main.vue'
import i18n from '@/plugins/i18n'
import router, { setupRouterGuards } from '@/plugins/router'
import { useSettingsStore } from '@/stores/settings'

// PrimeVue imports
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config'
import ConfirmationService from 'primevue/confirmationservice'
import ToastService from 'primevue/toastservice'

// Tauri imports
import { definePreset } from '@primevue/themes'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { Menu } from '@tauri-apps/api/menu'
import { TrayIcon, type TrayIconEvent } from '@tauri-apps/api/tray'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { exit } from '@tauri-apps/plugin-process'

// Other imports
import Color from 'color'

document.addEventListener('contextmenu', (event) => event.preventDefault())

// Window initialization
export const window = await getCurrentWindow()
const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'

// Tray menu configuration
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

// Tray icon configuration
const trayOptions = {
  icon,
  menu,
  menuOnLeftClick: false,
  action: (event: TrayIconEvent) => {
    if (
      event.type === 'Click' &&
      event.button === 'Left' &&
      event.buttonState === 'Down'
    ) {
      window.show()
      window.unminimize()
      window.setFocus()
    }
  },
}

// Window event handlers
window.onCloseRequested(async () => {
  window.hide()
})

// App initialization
const pinia = createPinia()
const app = createApp(Main)

// SVG loader utility
async function loadSvg(name: string): Promise<string> {
  try {
    const response = await fetch(`/src/assets/icons/${name}.svg`)
    return await response.text()
  } catch {
    return ''
  }
}

// App configuration
app.use(pinia).use(router).use(ToastService).use(ConfirmationService)

// Router guards setup
setupRouterGuards(router)

// SVG directive registration
app.use(i18n).directive('svg', {
  async mounted(el, binding) {
    el.innerHTML = await loadSvg(binding.value)
  },
  async updated(el, binding) {
    el.innerHTML = await loadSvg(binding.value)
  },
})

// Error handling
export const error: Ref<string[]> = ref([])

// Settings, PrimeVue, and tray initialization
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

if (settingsStore.minimize_to_tray_on_close) {
  await TrayIcon.new(trayOptions).catch(console.error)
}

// Mount application
app.mount('#app')
