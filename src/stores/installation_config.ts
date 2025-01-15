import { defineStore } from "pinia";

// Core interface
interface InstallationConfig {
  // Input path
  zip_path: string;

  // Installation options
  current_user_only: boolean;
  create_desktop_shortcut: boolean;
  create_registry_key: boolean;
  create_start_menu_shortcut: boolean;
  install_path: string;
  executable_path: string;

  // Application metadata
  app_icon: string;
  app_name: string;
  app_publisher: string;
  app_version: string;
}

// Store definition
export const useInstallationConfigStore = defineStore("installation_config", {
  state: (): InstallationConfig => ({
    zip_path: "",

    current_user_only: false,
    create_desktop_shortcut: false,
    create_registry_key: true,
    create_start_menu_shortcut: true,
    install_path: "",
    executable_path: "",

    app_icon: "",
    app_name: "",
    app_publisher: "",
    app_version: "",
  }),

  actions: {
    resetConfig() {
      const zipPath = this.zip_path;
      this.$reset();
      this.zip_path = zipPath;
    },
  },
});
