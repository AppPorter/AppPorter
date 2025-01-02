import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";

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
      let result: any;
      try {
        result = await invoke("execute_command", {
          command: "LoadSettings",
          arg: null,
        });

        // Convert Rust debug format to JSON
        const processString = (str: string) => {
          // Extract and process paths first
          const paths: string[] = [];
          let json = str
            // Extract paths with proper escaping
            .replace(
              /install_path:\s*"([A-Z]):\\([^"]+)"/g,
              (_, drive, path) => {
                const processed = `"${drive}:\\\\${path.replace(/\\/g, "\\\\")}"`;
                paths.push(processed);
                return `"install_path":__PATH${paths.length - 1}__`;
              },
            )
            // Remove Rust type names
            .replace(
              /(?:Settings|Installation|InstallSettings|RegistrySettings)\s*{/g,
              "{",
            )
            .replace(/:\s*(?:Install|Registry)\s*{/g, ": {")
            // Handle enum values
            .replace(/:\s*(AllUsers|CurrentUser)(?=[,}\s])/g, ':"$1"')
            // Format JSON structure
            .replace(/([a-zA-Z_]+)(?=\s*:)/g, '"$1"')
            .replace(/[\n\s]+/g, "")
            .replace(/,\s*}/g, "}");

          // Restore paths and return
          return json.replace(/__PATH(\d+)__/g, (_, i) => paths[parseInt(i)]);
        };

        // Parse and validate
        const jsonStr = processString(result as string);
        const settings = JSON.parse(jsonStr) as Settings;

        // Handle enum
        settings.installation.install_mode =
          settings.installation.install_mode === "CurrentUser"
            ? InstallMode.CurrentUser
            : InstallMode.AllUsers;

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
