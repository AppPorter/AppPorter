import { FileTreeNode } from '#/FileTreeNode'
import { defineStore } from 'pinia'
import type { AppDetails } from '#/AppDetails'
import type { ToolDetails } from '#/ToolDetails'

type Pages =
  | 'Home'
  | 'Install_App_Config'
  | 'Install_App_Progress'
  | 'Install_Tool_Config'
  | 'Install_Tool_Progress'

interface InstallConfig {
  page: Pages
  show_preview_drawer: boolean

  zip_path: string
  timestamp: number
  url: string
  archive_password: string
  file_tree: FileTreeNode[]
  temp: {
    zip_path: string
    timestamp: number
    url: string
    archive_password: string
    file_tree: FileTreeNode[]
  }
  archive_exe_path?: string
  archive_path_dir?: string

  app_details: AppDetails
  tool_details: ToolDetails
}

export const InstallConfigStore = defineStore('install_config', {
  state: (): InstallConfig => ({
    page: 'Home',
    show_preview_drawer: false,

    zip_path: '',
    timestamp: 0,
    url: '',
    archive_password: '',
    file_tree: [],
    temp: {
      zip_path: '',
      timestamp: 0,
      url: '',
      archive_password: '',
      file_tree: [],
    },

    app_details: {
      current_user_only: false,
      info: {
        name: '',
        icon: '',
        publisher: '',
        version: '',
      },
      config: {
        custom_icon: false,
        create_desktop_shortcut: false,
        create_start_menu_shortcut: true,
        create_registry_key: false,
        add_to_path: [false, ''],
      },
      install_path: '',
      full_path: '',
    },
    tool_details: {
      name: '',
      add_to_path: [false, ''],
      install_path: '',
    },
  }),

  actions: {
    setTempData(data: {
      zip_path?: string
      timestamp?: number
      url?: string
      archive_password?: string
      file_tree?: FileTreeNode[]
    }) {
      if (data.zip_path !== undefined) this.temp.zip_path = data.zip_path
      if (data.timestamp !== undefined) this.temp.timestamp = data.timestamp
      if (data.url !== undefined) this.temp.url = data.url
      if (data.archive_password !== undefined) this.temp.archive_password = data.archive_password
      if (data.file_tree !== undefined) this.temp.file_tree = data.file_tree
    },

    clearTempData() {
      this.temp = {
        zip_path: '',
        timestamp: 0,
        url: '',
        archive_password: '',
        file_tree: [],
      }
    },

    confirmTempData() {
      this.zip_path = this.temp.zip_path
      this.timestamp = this.temp.timestamp
      this.url = this.temp.url
      this.archive_password = this.temp.archive_password
      this.file_tree = this.temp.file_tree
      this.clearTempData()
    },
  },
})
