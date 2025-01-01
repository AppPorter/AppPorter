export interface Settings {
  download_dir: string;
  language: string;
  minimize_to_tray_on_close: boolean;
  system_drive_letter: string;
  theme: string;
  username: string;
  installation: Installation;
}

export interface Installation {
  install_mode: InstallMode;
  start_path: string;
  all_users: InstallSettings;
  current_user: InstallSettings;
}

export interface InstallSettings {
  create_desktop_shortcut: boolean;
  create_registry_key: boolean;
  create_start_menu_shortcut: boolean;
  install_path: string;
  registry: RegistrySettings;
}

export enum InstallMode {
  AllUsers = "AllUsers",
  CurrentUser = "CurrentUser",
}

export interface RegistrySettings {
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
