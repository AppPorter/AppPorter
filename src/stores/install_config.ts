import type { AppDetails } from '#/AppDetails'
import { FileTreeNode } from '#/FileTreeNode'
import type { ToolDetails } from '#/ToolDetails'
import { defineStore } from 'pinia'

interface InstallConfig {
  id: string
  zip_path: string
  timestamp_add: string
  timestamp_update: string
  url: string
  archive_password: string
  file_tree: FileTreeNode[]
  temp: {
    id: string
    zip_path: string
    timestamp_add: string
    timestamp_update: string
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
    id: '',
    zip_path: '',
    timestamp_add: '',
    timestamp_update: '',
    url: '',
    archive_password: '',
    file_tree: [],
    temp: {
      id: '',
      zip_path: '',
      timestamp_add: '',
      timestamp_update: '',
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
      id?: string
      zip_path?: string
      timestamp_add?: string
      timestamp_update?: string
      url?: string
      archive_password?: string
      file_tree?: FileTreeNode[]
    }) {
      if (data.id !== undefined) this.temp.id = data.id
      if (data.zip_path !== undefined) this.temp.zip_path = data.zip_path
      if (data.timestamp_add !== undefined) this.temp.timestamp_add = data.timestamp_add
      if (data.timestamp_update !== undefined) this.temp.timestamp_update = data.timestamp_update
      if (data.url !== undefined) this.temp.url = data.url
      if (data.archive_password !== undefined) this.temp.archive_password = data.archive_password
      if (data.file_tree !== undefined) this.temp.file_tree = data.file_tree
    },

    clearTempData() {
      this.temp = {
        id: '',
        zip_path: '',
        timestamp_add: '',
        timestamp_update: '',
        url: '',
        archive_password: '',
        file_tree: [],
      }
    },

    confirmTempData() {
      this.id = this.temp.id
      this.zip_path = this.temp.zip_path
      this.timestamp_add = this.temp.timestamp_add
      this.timestamp_update = this.temp.timestamp_update
      this.url = this.temp.url
      this.archive_password = this.temp.archive_password
      this.file_tree = this.temp.file_tree
      this.clearTempData()
    },
  },
})
