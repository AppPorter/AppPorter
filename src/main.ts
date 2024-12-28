import { createApp } from 'vue'
import App from './App.vue'
import { TrayIcon } from '@tauri-apps/api/tray'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { Menu } from '@tauri-apps/api/menu'
import { exit } from '@tauri-apps/plugin-process'
import { Window } from '@tauri-apps/api/window'

const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'
const menu = await Menu.new({
  items: [
    {
      id: 'open',
      text: 'Open',
      action: () => {},
    },
    {
      id: 'quit',
      text: 'Quit',
      action: () => {
        exit(0)
      },
    },
  ],
})
const options = {
  icon,
  menu,
  menuOnLeftClick: false,
}

await TrayIcon.new(options)

createApp(App).mount('#app')
