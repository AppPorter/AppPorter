import { FileTreeNode } from '@/components/ZipPreview/ZipPreview.vue'
import { defineStore } from 'pinia'
import type { AppDetails, ToolDetails } from './library'

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
      info: {
        name: '',
        icon: '',
        publisher: '',
        version: '',
      },
      config: {
        archive_exe_path: '',
        current_user_only: false,
        create_desktop_shortcut: false,
        create_start_menu_shortcut: true,
        create_registry_key: false,
        custom_icon: false,
        add_to_path: false,
        path_directory: '',
      },
      paths: {
        parent_install_path: '',
        install_path: '',
        full_path: '',
      },
    },
    tool_details: {
      name: '',
      config: {
        add_to_path: false,
        path_directory: '',
      },
      paths: {
        parent_install_path: '',
        install_path: '',
      },
    },
  }),

  actions: {
    // Set temp data for new incoming app
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

    // Clear temp data
    clearTempData() {
      this.temp = {
        zip_path: '',
        timestamp: 0,
        url: '',
        archive_password: '',
        file_tree: [],
      }
    },

    // Move temp data to main data (confirm installation)
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
