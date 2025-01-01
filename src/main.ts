import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Channel, invoke } from "@tauri-apps/api/core";
import { Menu } from "@tauri-apps/api/menu";
import { TrayIconEvent } from "@tauri-apps/api/tray";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { exit } from "@tauri-apps/plugin-process";
import { createApp, Ref, ref } from "vue";
import "./assets/index.css";
import i18n from "./i18n";
import Main from "./Main.vue";
import router from "./router.ts";

let channel = new Channel<String>();

channel.onmessage = (message) => {
  console.log(message);
};

let error: Ref<string[]> = ref([]);

invoke("execute_command", {
  command: "ReadSettings",
  arg: null,
  channel: channel,
}).catch((e) => {
  error.value.push(e as string);
  console.error(e);
});

const window = await getCurrentWindow();
const icon = (await defaultWindowIcon()) || "src-tauri\\icons\\icon.ico";
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
      action: () => {
        exit(0);
      },
    },
  ],
});
const options = {
  icon,
  menu,
  menuOnLeftClick: false,
  action: (event: TrayIconEvent) => {
    switch (event.type) {
      case "Click":
        if (event.button == "Left" && event.buttonState == "Down") {
          window.show();
          window.unminimize();
          window.setFocus();
        }
        break;
    }
  },
};

//await TrayIcon.new(options)

await window.onCloseRequested(async () => {
  window.hide();
});

async function loadSvg(name: string): Promise<string> {
  const path = `/src/assets/icons/${name}.svg`;
  try {
    const response = await fetch(path);
    return await response.text();
  } catch (error) {
    console.error("Failed to load SVG:", error);
    return "";
  }
}

createApp(Main)
  .use(router)
  .use(i18n)
  .directive("svg", {
    async mounted(el, binding) {
      const svgContent = await loadSvg(binding.value);
      el.innerHTML = svgContent;
    },
    async updated(el, binding) {
      const svgContent = await loadSvg(binding.value);
      el.innerHTML = svgContent;
    },
  })
  .mount("#app");
