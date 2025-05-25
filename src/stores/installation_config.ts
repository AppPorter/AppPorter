import { defineStore } from 'pinia'
import type { AppDetails } from './app_list'

type Pages =
  | 'Home'
  | 'InstallationConfig'
  | 'InstallationProgress'
  | 'CopyOnlyConfig'
  | 'CopyOnlyProgress'

interface InstallationConfig {
  zip_path: string
  details: AppDetails
  page: Pages
  archive_content: string[] | null
  timestamp: number
  archive_password: string
}

export const useInstallationConfigStore = defineStore('installation_config', {
  state: (): InstallationConfig => ({
    zip_path: '',
    details: {
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
        create_registry_key: true,
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
      },
    },
    page: 'Home',
    archive_content: null,
    timestamp: 0,
    archive_password: '',
  }),

  getters: {
    // Convenience getters for accessing nested properties
    name: (state) => state.details.info.name,
    icon: (state) => state.details.info.icon,
    publisher: (state) => state.details.info.publisher,
    version: (state) => state.details.info.version,
    executable_path: (state) => state.details.config.archive_exe_path,
    archive_password: (state) => state.details.config.archive_password,
    current_user_only: (state) => state.details.config.current_user_only,
    create_desktop_shortcut: (state) => state.details.config.create_desktop_shortcut,
    create_start_menu_shortcut: (state) => state.details.config.create_start_menu_shortcut,
    create_registry_key: (state) => state.details.config.create_registry_key,
    add_to_path: (state) => state.details.config.add_to_path,
    path_directory: (state) => state.details.config.path_directory,
    install_path: (state) => state.details.paths.parent_install_path,
    full_path: (state) => state.details.paths.full_path,
    validation_status: (state) => state.details.validation_status,

    // Convert to AppDetails for Rust backend
    toAppDetails: (state) => state.details,
  },

  actions: {
    resetConfig() {
      const zipPath = this.zip_path
      this.$reset()
      this.zip_path = zipPath
      this.archive_content = null
    },
  },
})
