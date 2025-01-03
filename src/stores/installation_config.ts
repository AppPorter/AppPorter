import { defineStore } from "pinia";

interface Installation_Config {
  app_icon: string; //auto
  app_name: string; //auto + configurable
  app_version: string; //auto + configurable
  current_user_only: boolean; //configurable
  create_desktop_shortcut: boolean; //configurable
  create_registry_key: boolean; //configurable
  create_start_menu_shortcut: boolean; //configurable
  install_path: string; //configurable
  publisher: String; //auto + configurable
  zip_path: string;
}

export const useInstallationConfigStore = defineStore("installation_config", {
  state: (): Installation_Config => ({
    app_icon: "",
    app_name: "",
    app_version: "",
    current_user_only: false,
    create_desktop_shortcut: false,
    create_registry_key: true,
    create_start_menu_shortcut: true,
    install_path: "",
    publisher: "",
    zip_path: "",
  }),

  actions: {},
});
