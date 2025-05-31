import { invoke } from '@tauri-apps/api/core';
import { derived, get, writable } from 'svelte/store';

interface AppList {
	apps: App[];
	libs: Lib[];
}

export interface App {
	timestamp: number;
	installed: boolean;
	url: string;
	details: AppDetails;
}

export interface AppDetails {
	info: AppBasicInformation;
	config: AppConfig;
	paths: AppPaths;
	validation_status: AppValidationStatus;
}

export interface AppBasicInformation {
	name: string;
	icon: string;
	publisher: string;
	version: string;
}

export interface AppConfig {
	archive_exe_path: string;
	archive_password: string;
	current_user_only: boolean;
	create_desktop_shortcut: boolean;
	create_start_menu_shortcut: boolean;
	create_registry_key: boolean;
	add_to_path: boolean;
	path_directory: string;
}

export interface AppPaths {
	parent_install_path: string;
	install_path: string;
	full_path: string;
}

export interface AppValidationStatus {
	file_exists: boolean;
	registry_valid: boolean;
	path_exists: boolean;
}

export interface Lib {
	timestamp: number;
	installed: boolean;
	url: string;
	details: LibDetails;
}

export interface LibDetails {
	name: string;
	config: LibConfig;
	paths: LibPaths;
	validation_status: LibValidationStatus;
}

export interface LibConfig {
	archive_password: string;
	add_to_path: boolean;
	path_directory: string;
}

export interface LibPaths {
	parent_install_path: string;
	install_path: string;
}

export interface LibValidationStatus {
	file_exists: boolean;
	path_exists: boolean;
}

// Create writable store
const appListStore = writable<AppList>({
	apps: [],
	libs: []
});

// Derived stores for computed values
export const installedApps = derived(appListStore, ($appList: AppList) => {
	return $appList.apps.filter((app: App) => app.installed);
});

export const installedLibs = derived(appListStore, ($appList: AppList) => {
	return $appList.libs.filter((lib: Lib) => lib.installed);
});

// Export the store actions
export const AppListStore = {
	subscribe: appListStore.subscribe,
	update: appListStore.update,
	set: appListStore.set,

	loadAppList(): Promise<void> {
		return invoke<string>('execute_command', {
			command: { name: 'LoadAppList' }
		}).then((result) => {
			appListStore.set(JSON.parse(result));
		});
	},

	saveAppList(): Promise<void> {
		const currentState = get(appListStore);
		return invoke('execute_command', {
			command: {
				name: 'SaveAppList',
				app_list: currentState
			}
		});
	},

	hasLink(url: string): boolean {
		const currentState = get(appListStore);
		return (
			currentState.apps.some((app: App) => app.url === url) ||
			currentState.libs.some((lib: Lib) => lib.url === url)
		);
	},

	hasApp(url: string): boolean {
		const currentState = get(appListStore);
		return currentState.apps.some((app: App) => app.url === url);
	},

	hasLib(url: string): boolean {
		const currentState = get(appListStore);
		return currentState.libs.some((lib: Lib) => lib.url === url);
	},

	getAppByTimestamp(timestamp: number): App | undefined {
		const currentInstalledApps = get(installedApps);
		const filtered = currentInstalledApps.filter((app: App) => app.timestamp === timestamp);
		return filtered.length > 0 ? filtered[0] : undefined;
	},

	getLibByTimestamp(timestamp: number): Lib | undefined {
		const currentInstalledLibs = get(installedLibs);
		const filtered = currentInstalledLibs.filter((lib: Lib) => lib.timestamp === timestamp);
		return filtered.length > 0 ? filtered[0] : undefined;
	},

	executeUninstall(timestamp: number): Promise<void> {
		return invoke('execute_command', {
			command: {
				name: 'Uninstall',
				timestamp
			}
		}).then(() => {
			return this.loadAppList();
		});
	},

	removeApp(timestamp: number): Promise<void> {
		// Remove the app from the list (without uninstalling)
		appListStore.update((state) => ({
			...state,
			apps: state.apps.filter((app: App) => app.timestamp !== timestamp)
		}));
		return this.saveAppList();
	},

	removeLib(timestamp: number): Promise<void> {
		// Remove the lib from the list (without uninstalling)
		appListStore.update((state) => ({
			...state,
			libs: state.libs.filter((lib: Lib) => lib.timestamp !== timestamp)
		}));
		return this.saveAppList();
	}
};
