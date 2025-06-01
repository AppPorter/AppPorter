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
    app_details: {
      info: {
        name: '',
        icon: '',
        publisher: '',
        version: '',
      },
      config: {
        archive_exe_path: '',
        archive_password: '',
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
      validation_status: {
        file_exists: false,
        registry_valid: false,
        path_exists: false,
      },
    },
    lib_details: {
      name: '',
      config: {
        archive_password: '',
        add_to_path: false,
        path_directory: '',
      },
      paths: {
        parent_install_path: '',
        install_path: '',
      },
      validation_status: {
        file_exists: false,
        path_exists: false,
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
