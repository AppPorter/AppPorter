import { createApp } from 'vue'
import Main from './Main.vue'
import { TrayIcon, TrayIconEvent } from '@tauri-apps/api/tray'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { Menu } from '@tauri-apps/api/menu'
import { exit } from '@tauri-apps/plugin-process'
import { getCurrentWindow } from '@tauri-apps/api/window'
import router from './router.ts'

const window = await getCurrentWindow()
const icon = (await defaultWindowIcon()) || 'src-tauri\\icons\\icon.ico'
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
          window.show()
          window.unminimize()
          window.setFocus()
        }
        break
    }
  },
}

await TrayIcon.new(options)

await window.onCloseRequested(async () => {
  window.hide()
})

createApp(Main).use(router).mount('#app')
