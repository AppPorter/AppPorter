import { createApp } from 'vue'
import App from './App.vue'
import { TrayIcon, TrayIconEvent } from '@tauri-apps/api/tray'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { Menu } from '@tauri-apps/api/menu'
import { exit } from '@tauri-apps/plugin-process'
import { getCurrentWindow } from '@tauri-apps/api/window'

const window = await getCurrentWindow()
const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'
const menu = await Menu.new({
  items: [
    {
      id: 'open',
      text: 'Open',
      action: () => {
        window.unminimize()
        window.setFocus()
      },
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
  action: (event: TrayIconEvent) => {
    switch (event.type) {
      case 'Click':
        if (event.button == 'Left' && event.buttonState == 'Down') {
          console.log(
            `mouse ${event.button} button pressed, state: ${event.buttonState}`
          )
          window.unminimize()
          window.setFocus()
        }
        break
    }
  },
}

await TrayIcon.new(options)

createApp(App).mount('#app')
