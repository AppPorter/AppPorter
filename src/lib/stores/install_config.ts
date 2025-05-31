import { derived, get, writable } from 'svelte/store';
import type { AppDetails } from './app_list';

type Pages =
	| 'Home'
	| 'Install_App_Config'
	| 'Install_App_Progress'
	| 'Install_Lib_Config'
	| 'Install_Lib_Progress';

interface InstallConfig {
	zip_path: string;
	details: AppDetails;
	page: Pages;
	archive_content: string[] | null;
	timestamp: number;
}

// Create writable store
const installConfigStore = writable<InstallConfig>({
	zip_path: '',
	details: {
		info: {
			name: '',
			icon: '',
			publisher: '',
			version: ''
		},
		config: {
			archive_exe_path: '',
			archive_password: '',
			current_user_only: false,
			create_desktop_shortcut: false,
			create_start_menu_shortcut: true,
			create_registry_key: true,
			add_to_path: false,
			path_directory: ''
		},
		paths: {
			parent_install_path: '',
			install_path: '',
			full_path: ''
		},
		validation_status: {
			file_exists: false,
			registry_valid: false,
			path_exists: false
		}
	},
	page: 'Home',
	archive_content: null,
	timestamp: 0
});

// Derived stores for convenience getters
export const name = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.info.name
);
export const icon = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.info.icon
);
export const publisher = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.info.publisher
);
export const version = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.info.version
);
export const executable_path = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.archive_exe_path
);
export const archive_password = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.archive_password
);
export const current_user_only = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.current_user_only
);
export const create_desktop_shortcut = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.create_desktop_shortcut
);
export const create_start_menu_shortcut = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.create_start_menu_shortcut
);
export const create_registry_key = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.create_registry_key
);
export const add_to_path = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.add_to_path
);
export const path_directory = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.config.path_directory
);
export const install_path = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.paths.parent_install_path
);
export const full_path = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.paths.full_path
);
export const validation_status = derived(
	installConfigStore,
	($state: InstallConfig) => $state.details.validation_status
);
export const toAppDetails = derived(installConfigStore, ($state: InstallConfig) => $state.details);

// Export the store
export const InstallConfigStore = {
	subscribe: installConfigStore.subscribe,
	update: installConfigStore.update,
	set: installConfigStore.set,

	resetConfig() {
		const currentState = get(installConfigStore);
		const zipPath = currentState.zip_path;

		installConfigStore.set({
			zip_path: zipPath,
			details: {
				info: {
					name: '',
					icon: '',
					publisher: '',
					version: ''
				},
				config: {
					archive_exe_path: '',
					archive_password: '',
					current_user_only: false,
					create_desktop_shortcut: false,
					create_start_menu_shortcut: true,
					create_registry_key: true,
					add_to_path: false,
					path_directory: ''
				},
				paths: {
					parent_install_path: '',
					install_path: '',
					full_path: ''
				},
				validation_status: {
					file_exists: false,
					registry_valid: false,
					path_exists: false
				}
			},
			page: 'Home',
			archive_content: null,
			timestamp: 0
		});
	}
};
