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
import ToastService from "primevue/toastservice";

// Tauri imports
import { definePreset } from "@primevue/themes";
import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu } from "@tauri-apps/api/menu";
import { TrayIcon, type TrayIconEvent } from "@tauri-apps/api/tray";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { exit } from "@tauri-apps/plugin-process";

document.addEventListener("contextmenu", (event) => event.preventDefault());

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
app.use(pinia).use(router).use(ToastService).use(ConfirmationService);

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

// Color adjustment utility
function adjustColor(hex: string, lighten: number = 0): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);

  const getRGB = (value: number) => {
    const adjusted = value + lighten * 255;
    return Math.min(255, Math.max(0, Math.round(adjusted)));
  };

  const rr = getRGB(r).toString(16).padStart(2, "0");
  const gg = getRGB(g).toString(16).padStart(2, "0");
  const bb = getRGB(b).toString(16).padStart(2, "0");

  return `#${rr}${gg}${bb}`;
}

// Settings, PrimeVue, and tray initialization
const settingsStore = useSettingsStore();
await settingsStore.loadSettings();

const UserColor = definePreset(Aura, {
  semantic: {
    primary: {
      50: adjustColor(settingsStore.color, 0.5),
      100: adjustColor(settingsStore.color, 0.4),
      200: adjustColor(settingsStore.color, 0.3),
      300: adjustColor(settingsStore.color, 0.2),
      400: adjustColor(settingsStore.color, 0.1),
      500: settingsStore.color,
      600: adjustColor(settingsStore.color, -0.1),
      700: adjustColor(settingsStore.color, -0.2),
      800: adjustColor(settingsStore.color, -0.3),
      900: adjustColor(settingsStore.color, -0.4),
      950: adjustColor(settingsStore.color, -0.5),
    },
  },
});

app.use(PrimeVue, {
  theme: {
    preset: UserColor,
    options: {
      prefix: "p",
      darkModeSelector: "system",
      cssLayer: false,
    },
  },
});

if (settingsStore.minimize_to_tray_on_close) {
  await TrayIcon.new(trayOptions).catch(console.error);
}

// Mount application
app.mount("#app");
