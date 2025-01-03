import { defineStore } from "pinia";

interface Installation_Config {
  app_icon: string;
  app_name: string;
  app_version: string;
  install_mode: InstallMode;
  create_desktop_shortcut: boolean;
  create_registry_key: boolean;
  create_start_menu_shortcut: boolean;
  install_path: string;
  publisher: String;
  zip_path: string;
}
enum InstallMode {
  AllUsers = "AllUsers",
  CurrentUser = "CurrentUser",
}

export const useInstallationConfigStore = defineStore("installation_config", {
  state: (): Installation_Config => ({
    app_icon: "",
    app_name: "",
    app_version: "",
    install_mode: InstallMode.AllUsers,
    create_desktop_shortcut: false,
    create_registry_key: true,
    create_start_menu_shortcut: true,
    install_path: "",
    publisher: "",
    zip_path: "",
  }),

  actions: {},
});
