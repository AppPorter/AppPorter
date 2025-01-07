import { defineStore } from "pinia";

interface InstallationConfig {
  zip_path: string;

  current_user_only: boolean; //configurable
  create_desktop_shortcut: boolean; //configurable
  create_registry_key: boolean; //configurable
  create_start_menu_shortcut: boolean; //configurable
  install_path: string; //configurable

  executable_path: string; //auto + configurable

  app_icon: string; //auto
  app_name: string; //auto + configurable
  app_publisher: string; //auto + configurable
  app_version: string; //auto + configurable
}

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
    // Custom reset method if needed
    customReset() {
      // Reset specific fields while keeping others
      this.$patch({
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
      });
    },
  },
});
