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
    resetConfig() {
      const zipPath = this.zip_path
      const fileTree = this.file_tree
      this.$reset()
      this.zip_path = zipPath
      this.file_tree = fileTree
    },
  },
})
