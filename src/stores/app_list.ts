import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

export interface AppBasicInformation {
  name: string
  icon: string
  publisher: string
  version: string
}

export interface AppConfig {
  archive_exe_path: string
  current_user_only: boolean
  create_desktop_shortcut: boolean
  create_start_menu_shortcut: boolean
  create_registry_key: boolean
  add_to_path: boolean
  path_directory: string
}

export interface AppPaths {
  parent_install_path: string
  install_path: string
  full_path: string
}

export interface AppValidationStatus {
  file_exists: boolean
  registry_valid: boolean
}

export interface AppDetails {
  info: AppBasicInformation
  config: AppConfig
  paths: AppPaths
  validation_status: AppValidationStatus
}

export interface App {
  timestamp: number
  installed: boolean
  copy_only: boolean
  url: string
  details: AppDetails
}

interface AppList {
  links: App[]
}

export const useAppListStore = defineStore('app_list', {
  state: (): AppList => ({
    links: [],
  }),

  getters: {
    installedApps: (state) => {
      return state.links.filter((app) => app.installed)
    },
  },

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

    getAppByTimestamp(timestamp: number) {
      return this.installedApps.find((app) => app.timestamp === timestamp)
    },

    async executeUninstall(timestamp: number) {
      await invoke('execute_command', {
        command: {
          name: 'Uninstallation',
          timestamp,
        },
      })
      await this.loadAppList()
    },

    async removeApp(timestamp: number) {
      // Remove the app from the list (without uninstalling)
      this.links = this.links.filter((app) => app.timestamp !== timestamp)
      await this.saveAppList()
    },
  },
})
