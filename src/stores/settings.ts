import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";

interface Settings {
  language: string;
  minimize_to_tray_on_close: boolean;
  system_drive_letter: string;
  theme: string;
  username: string;
  installation: Installation;
}

interface Installation {
  current_user_only: boolean;
  all_users: InstallSettings;
  current_user: InstallSettings;
}

interface InstallSettings {
  create_desktop_shortcut: boolean;
  create_registry_key: boolean;
  create_start_menu_shortcut: boolean;
  install_path: string;
}

export const useSettingsStore = defineStore("settings", {
  state: (): Settings => ({
    language: "",
    minimize_to_tray_on_close: false,
    system_drive_letter: "",
    theme: "",
    username: "",
    installation: {
      current_user_only: false,
      all_users: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: "",
      },
      current_user: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: "",
      },
    },
  }),

  actions: {
    async loadSettings() {
      try {
        const result = await invoke("execute_command", {
          command: "LoadSettings",
          arg: null,
        });

        const settings = JSON.parse(result as string) as Settings;
        this.$patch(settings);
      } catch (error) {
        console.error("Failed to load settings:", error);
      }
    },

    async saveSettings() {
      try {
        await invoke("save_settings", { settings: this.$state });
      } catch (error) {
        console.error("Failed to save settings:", error);
      }
    },
  },
});
