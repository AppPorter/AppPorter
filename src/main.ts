// Style imports
import "@/assets/index.css";

// Vue core imports
import { createPinia } from "pinia";
import { createApp, ref, type Ref } from "vue";

// App components and plugins
import Main from "@/Main.vue";
import i18n from "@/plugin/i18n";
import router, { setupRouterGuards } from "@/plugin/router";
import { useSettingsStore } from "@/stores/settings";

// PrimeVue imports
import Aura from "@primevue/themes/aura";
import PrimeVue from "primevue/config";
import ConfirmationService from "primevue/confirmationservice";

// Tauri imports
import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu } from "@tauri-apps/api/menu";
import { TrayIcon, type TrayIconEvent } from "@tauri-apps/api/tray";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { exit } from "@tauri-apps/plugin-process";

// Window initialization
export const window = await getCurrentWindow();
const icon = (await defaultWindowIcon()) || "src-tauri\\icons\\icon.ico";

// Tray menu configuration
const menu = await Menu.new({
  items: [
    {
      id: "open",
      text: "Open",
      action: () => {
        window.show();
        window.unminimize();
        window.setFocus();
      },
    },
    {
      id: "quit",
      text: "Quit",
      action: () => exit(0),
    },
  ],
});

// Tray icon configuration
const trayOptions = {
  icon,
  menu,
  menuOnLeftClick: false,
  action: (event: TrayIconEvent) => {
    if (
      event.type === "Click" &&
      event.button === "Left" &&
      event.buttonState === "Down"
    ) {
      window.show();
      window.unminimize();
      window.setFocus();
    }
  },
};

// Window event handlers
window.onCloseRequested(async () => {
  window.hide();
});

// App initialization
const pinia = createPinia();
const app = createApp(Main);

// SVG loader utility
async function loadSvg(name: string): Promise<string> {
  try {
    const response = await fetch(`/src/assets/icons/${name}.svg`);
    return await response.text();
  } catch {
    return "";
  }
}

// App configuration
app
  .use(pinia)
  .use(router)
  .use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        prefix: "p",
        darkModeSelector: "system",
        cssLayer: false,
      },
    },
  })
  .use(ConfirmationService);

// Router guards setup
setupRouterGuards(router);

// SVG directive registration
app.use(i18n).directive("svg", {
  async mounted(el, binding) {
    el.innerHTML = await loadSvg(binding.value);
  },
  async updated(el, binding) {
    el.innerHTML = await loadSvg(binding.value);
  },
});

// Error handling
export const error: Ref<string[]> = ref([]);

// Settings and tray initialization
const settingsStore = useSettingsStore();
await settingsStore.loadSettings();

if (settingsStore.minimize_to_tray_on_close) {
  await TrayIcon.new(trayOptions).catch(console.error);
}

// Mount application
app.mount("#app");
