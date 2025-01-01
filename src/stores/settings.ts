import { Channel, invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { Ref, ref } from "vue";

interface Settings {
  download_dir: string;
  language: string;
  minimize_to_tray_on_close: boolean;
  system_drive_letter: string;
  theme: string;
  username: string;
  installation: Installation;
}

interface Installation {
  install_mode: InstallMode;
  start_path: string;
  all_users: InstallSettings;
  current_user: InstallSettings;
}

interface InstallSettings {
  create_desktop_shortcut: boolean;
  create_registry_key: boolean;
  create_start_menu_shortcut: boolean;
  install_path: string;
  registry: RegistrySettings;
}

enum InstallMode {
  AllUsers = "AllUsers",
  CurrentUser = "CurrentUser",
}

interface RegistrySettings {
  create_comments: boolean;
  create_display_icon: boolean;
  create_display_name: boolean;
  create_display_version: boolean;
  create_estimated_size: boolean;
  create_install_location: boolean;
  create_no_modify: boolean;
  create_no_remove: boolean;
  create_no_repair: boolean;
  create_publisher: boolean;
  create_uninstall_string: boolean;
}

export const useSettingsStore = defineStore("settings", {
  state: (): Settings => ({
    download_dir: "",
    language: "",
    minimize_to_tray_on_close: false,
    system_drive_letter: "",
    theme: "",
    username: "",
    installation: {
      install_mode: InstallMode.CurrentUser,
      start_path: "",
      all_users: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: "",
        registry: {
          create_comments: false,
          create_display_icon: false,
          create_display_name: false,
          create_display_version: false,
          create_estimated_size: false,
          create_install_location: false,
          create_no_modify: false,
          create_no_remove: false,
          create_no_repair: false,
          create_publisher: false,
          create_uninstall_string: false,
        },
      },
      current_user: {
        create_desktop_shortcut: false,
        create_registry_key: false,
        create_start_menu_shortcut: false,
        install_path: "",
        registry: {
          create_comments: false,
          create_display_icon: false,
          create_display_name: false,
          create_display_version: false,
          create_estimated_size: false,
          create_install_location: false,
          create_no_modify: false,
          create_no_remove: false,
          create_no_repair: false,
          create_publisher: false,
          create_uninstall_string: false,
        },
      },
    },
  }),

  actions: {
    async loadSettings() {
      try {
        const settings_channel = new Channel<Settings>();
        let settings: Ref<Settings> = ref({} as Settings);

        settings_channel.onmessage = (message) => {
          settings.value = message;
          console.log(settings.value);
        };

        invoke("execute_command", {
          command: "LoadSettings",
          arg: null,
          channel: settings_channel,
        }).catch((e) => {
          e.value.push(e as string);
          console.error(e);
        });

        this.$patch(settings.value);
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
