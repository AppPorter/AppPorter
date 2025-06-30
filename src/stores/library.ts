import { exec } from '@/exec'
import { defineStore } from 'pinia'

export type InstallTypes = 'app' | 'tool' | 'url'

interface Library {
  apps: App[]
  tools: Tool[]
  urls: Url[]
}

export interface App {
  timestamp: number
  installed: boolean
  url: string
  archive_password: string
  details: AppDetails
  validation_status: AppValidationStatus
}

export interface AppDetails {
  current_user_only: boolean
  info: AppInfo
  config: AppConfig
  install_path: string
  full_path: string
}

export interface AppInfo {
  name: string
  icon: string
  publisher: string
  version: string
}

export interface AppConfig {
  custom_icon: boolean
  create_desktop_shortcut: boolean
  create_start_menu_shortcut: boolean
  create_registry_key: boolean
  add_to_path: [boolean, string]
}

export interface AppValidationStatus {
  file_exists: boolean
  registry_valid: boolean
  path_exists: boolean
}

export interface Tool {
  timestamp: number
  installed: boolean
  url: string
  archive_password: string
  details: ToolDetails
  validation_status: ToolValidationStatus
}

export interface ToolDetails {
  name: string
  add_to_path: [boolean, string]
  install_path: string
}

export interface ToolValidationStatus {
  file_exists: boolean
  path_exists: boolean
}

export interface Url {
  url: string
  timestamp: number
}

export const LibraryStore = defineStore('library', {
  state: (): Library => ({
    apps: [],
    tools: [],
    urls: [],
  }),

  getters: {
    installedApps: (state) => {
      return state.apps.filter((app) => app.installed)
    },
    installedTools: (state) => {
      return state.tools.filter((tool) => tool.installed)
    },
  },

  actions: {
    async loadLibrary() {
      const result = await exec('LoadLibrary')
      this.$patch(result)
    },

    async saveLibrary() {
      await exec('SaveLibrary', {
        library: this.$state,
      })
    },

    hasLink(url: string): boolean {
      return this.apps.some((app) => app.url === url) || this.tools.some((tool) => tool.url === url)
    },

    hasApp(url: string): boolean {
      return this.apps.some((app) => app.url === url)
    },

    hasTool(url: string): boolean {
      return this.tools.some((tool) => tool.url === url)
    },

    getAppByTimestamp(timestamp: number) {
      return this.installedApps.find((app) => app.timestamp === timestamp)
    },

    getToolByTimestamp(timestamp: number) {
      return this.installedTools.find((tool) => tool.timestamp === timestamp)
    },

    async executeUninstall(apptype: InstallTypes, timestamp: number) {
      switch (apptype) {
        case 'app':
          await exec('UninstallApp', { timestamp })
          break
        case 'tool':
          await exec('UninstallTool', { timestamp })
          break
      }

      await this.loadLibrary()
    },

    async removeApp(timestamp: number) {
      this.apps = this.apps.filter((app) => app.timestamp !== timestamp)
      await this.saveLibrary()
    },

    async removeTool(timestamp: number) {
      this.tools = this.tools.filter((tool) => tool.timestamp !== timestamp)
      await this.saveLibrary()
    },
  },
})
