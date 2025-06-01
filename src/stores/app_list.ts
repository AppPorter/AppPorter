import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

interface AppList {
  apps: App[]
  libs: Lib[]
}

export interface App {
  timestamp: number
  installed: boolean
  url: string
  details: AppDetails
}

export interface AppDetails {
  info: AppBasicInformation
  config: AppConfig
  paths: AppPaths
  validation_status?: AppValidationStatus
}

export interface AppBasicInformation {
  name: string
  icon: string
  publisher: string
  version: string
}

export interface AppConfig {
  archive_exe_path: string
  archive_password?: string
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
  path_exists: boolean
}

export interface Lib {
  timestamp: number
  installed: boolean
  url: string
  details: LibDetails
}

export interface LibDetails {
  name: string
  config: LibConfig
  paths: LibPaths
  validation_status?: LibValidationStatus
}

export interface LibConfig {
  archive_password?: string
  add_to_path: boolean
  path_directory: string
}

export interface LibPaths {
  parent_install_path: string
  install_path: string
}

export interface LibValidationStatus {
  file_exists: boolean
  path_exists: boolean
}

export const AppListStore = defineStore('app_list', {
  state: (): AppList => ({
    apps: [],
    libs: [],
  }),

  getters: {
    installedApps: (state) => {
      return state.apps.filter((app) => app.installed)
    },
    installedLibs: (state) => {
      return state.libs.filter((lib) => lib.installed)
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
      return this.apps.some((app) => app.url === url) || this.libs.some((lib) => lib.url === url)
    },

    hasApp(url: string): boolean {
      return this.apps.some((app) => app.url === url)
    },

    hasLib(url: string): boolean {
      return this.libs.some((lib) => lib.url === url)
    },

    getAppByTimestamp(timestamp: number) {
      return this.installedApps.find((app) => app.timestamp === timestamp)
    },

    getLibByTimestamp(timestamp: number) {
      return this.installedLibs.find((lib) => lib.timestamp === timestamp)
    },

    async executeUninstall(timestamp: number) {
      await invoke('execute_command', {
        command: {
          name: 'Uninstall',
          timestamp,
        },
      })
      await this.loadAppList()
    },

    async removeApp(timestamp: number) {
      // Remove the app from the list (without uninstalling)
      this.apps = this.apps.filter((app) => app.timestamp !== timestamp)
      await this.saveAppList()
    },

    async removeLib(timestamp: number) {
      // Remove the lib from the list (without uninstalling)
      this.libs = this.libs.filter((lib) => lib.timestamp !== timestamp)
      await this.saveAppList()
    },
  },
})
