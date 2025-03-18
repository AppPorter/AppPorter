import { defineStore } from 'pinia'
import type { InstalledApp } from './app_list'

type Pages = 'Home' | 'Config' | 'Progress'

interface InstallationConfig extends InstalledApp {
  zip_path: string
  page: Pages
}

export const useInstallationConfigStore = defineStore('installation_config', {
  state: (): InstallationConfig => ({
    zip_path: '',
    name: '',
    icon: '',
    publisher: '',
    version: '',
    install_path: '',
    executable_path: '',
    full_path: '',
    current_user_only: false,
    create_desktop_shortcut: false,
    create_start_menu_shortcut: true,
    create_registry_key: true,
    page: 'Home',
  }),

  actions: {
    resetConfig() {
      const zipPath = this.zip_path
      this.$reset()
      this.zip_path = zipPath
    },
  },
})
