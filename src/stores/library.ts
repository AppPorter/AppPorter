import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

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

    async executeUninstall(timestamp: number) {
      await invoke('execute_command', {
        command: {
          name: 'Uninstall',
          timestamp,
        },
      })
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

    async confirmAndExecuteUninstall(timestamp: number) {
      const { useConfirm } = await import('primevue/useconfirm')
      const { useI18n } = await import('vue-i18n')

      const confirm = useConfirm()
      const { t } = useI18n()

      const app = this.getAppByTimestamp(timestamp)
      if (!app) return

      return new Promise<void>((resolve, reject) => {
        confirm.require({
          message: t('ui.library.confirm_uninstall_message', {
            name: app.details.info.name,
          }),
          group: 'dialog',
          header: t('ui.library.confirm_uninstall_header', {
            name: app.details.info.name,
          }),
          icon: 'mir-warning',
          rejectProps: {
            label: t('g.cancel'),
            severity: 'secondary',
            outlined: true,
            icon: 'mir-close',
          },
          acceptProps: {
            label: t('cls.uninstall.self'),
            severity: 'danger',
            icon: 'mir-warning',
          },
          accept: async () => {
            await this.executeUninstall(timestamp)
            resolve()
          },
          reject: () => reject(),
        })
      })
    },
  },
})
