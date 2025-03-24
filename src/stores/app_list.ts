import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

export interface InstalledApp {
  name: string
  icon: string
  publisher: string
  version: string
  install_path: string
  executable_path: string
  full_path: string
  current_user_only: boolean
  create_desktop_shortcut: boolean
  create_start_menu_shortcut: boolean
  create_registry_key: boolean
}

interface App {
  timestamp: number
  installed: boolean
  details: InstalledApp
  url: string
}

interface AppList {
  links: App[]
}

export const useAppListStore = defineStore('app_list', {
  state: (): AppList => ({
    links: [],
  }),

  actions: {
    async loadAppList() {
      const result = await invoke<string>('execute_command', {
        command: { name: 'LoadAppList' },
      })
      this.$patch(JSON.parse(result))
    },

    async saveAppList() {
      await invoke('execute_command', {
        command: {
          name: 'SaveAppList',
          app_list: this.$state,
        },
      })
    },

    hasLink(url: string): boolean {
      return this.links.some((link) => link.url === url)
    },

    addLink() {
      this.links.push()
    },
  },
})
