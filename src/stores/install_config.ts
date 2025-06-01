import { defineStore } from 'pinia'
import type { AppDetails, LibDetails } from './app_list'

type Pages =
  | 'Home'
  | 'Preview'
  | 'Install_App_Config'
  | 'Install_App_Progress'
  | 'Install_Lib_Config'
  | 'Install_Lib_Progress'

interface InstallConfig {
  zip_path: string
  page: Pages
  archive_content: string[] | null
  timestamp: number
  url: string
  archive_password: string
  app_details: AppDetails
  lib_details: LibDetails
}

export const InstallConfigStore = defineStore('install_config', {
  state: (): InstallConfig => ({
    zip_path: '',
    page: 'Home',
    archive_content: null,
    timestamp: 0,
    url: '',
    archive_password: '',
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
    lib_details: {
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
      this.$reset()
      this.zip_path = zipPath
      this.archive_content = null
    },
  },
})
