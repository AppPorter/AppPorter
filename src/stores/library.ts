import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

export type AppTypes = 'app' | 'tool'

interface Library {
  apps: App[]
  tools: Tool[]
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
  custom_icon: boolean
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

export interface Tool {
  timestamp: number
  installed: boolean
  url: string
  details: ToolDetails
}

export interface ToolDetails {
  name: string
  config: ToolConfig
  paths: ToolPaths
  validation_status?: ToolValidationStatus
}

export interface ToolConfig {
  archive_password?: string
  add_to_path: boolean
  path_directory: string
}

export interface ToolPaths {
  parent_install_path: string
  install_path: string
}

export interface ToolValidationStatus {
  file_exists: boolean
  path_exists: boolean
}

export const LibraryStore = defineStore('library', {
  state: (): Library => ({
    apps: [],
    tools: [],
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
      const result = await invoke<string>('execute_command', {
        command: { name: 'LoadLibrary' },
      })
      this.$patch(JSON.parse(result))
    },

    async saveLibrary() {
      await invoke('execute_command', {
        command: {
          name: 'SaveLibrary',
          library: this.$state,
        },
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

    async executeUninstall(apptype: AppTypes, timestamp: number) {
      switch (apptype) {
        case 'app':
          await invoke('execute_command', {
            command: {
              name: 'UninstallApp',
              timestamp,
            },
          })
          break
        case 'tool':
          await invoke('execute_command', {
            command: {
              name: 'UninstallTool',
              timestamp,
            },
          })
          break
      }

      await this.loadLibrary()
    },

    async removeApp(timestamp: number) {
      // Remove the app from the list (without uninstalling)
      this.apps = this.apps.filter((app) => app.timestamp !== timestamp)
      await this.saveLibrary()
    },

    async removeTool(timestamp: number) {
      // Remove the tool from the list (without uninstalling)
      this.tools = this.tools.filter((tool) => tool.timestamp !== timestamp)
      await this.saveLibrary()
    },
  },
})
