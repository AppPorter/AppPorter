import { invoke } from '@tauri-apps/api/core';
import { get, writable } from 'svelte/store';
import { EnvStore } from './env';

export interface Settings {
	language: LanguageType;
	theme: ThemeType;
	minimize_to_tray_on_close: boolean;
	context_menu: boolean;
	auto_startup: boolean;
	color: string;
	run_as_admin: boolean;
	app_install: AppInstall;
	lib_install: LibInstall;
}

export type LanguageType = 'en' | 'zh' | 'fr' | 'de' | 'es' | 'ja' | 'ko' | 'ru';
export type ThemeType = 'system' | 'light' | 'dark';

interface AppInstall {
	current_user_only: boolean;
	all_users: InstallSettings;
	current_user: InstallSettings;
}

interface InstallSettings {
	create_desktop_shortcut: boolean;
	create_registry_key: boolean;
	create_start_menu_shortcut: boolean;
	install_path: string;
	add_to_path: boolean;
}

interface LibInstall {
	install_path: string;
	add_to_path: boolean;
}

// Create writable store
const settingsStore = writable<Settings>({
	language: 'en',
	theme: 'system',
	minimize_to_tray_on_close: false,
	context_menu: false,
	auto_startup: false,
	color: '',
	run_as_admin: false,

	app_install: {
		current_user_only: false,
		all_users: {
			create_desktop_shortcut: false,
			create_registry_key: false,
			create_start_menu_shortcut: false,
			install_path: '',
			add_to_path: false
		},
		current_user: {
			create_desktop_shortcut: false,
			create_registry_key: false,
			create_start_menu_shortcut: false,
			install_path: '',
			add_to_path: false
		}
	},
	lib_install: {
		install_path: '',
		add_to_path: false
	}
});

// Export the store
export const SettingsStore = {
	subscribe: settingsStore.subscribe,
	update: settingsStore.update,
	set: settingsStore.set,

	loadSettings(): Promise<void> {
		return invoke<string>('execute_command', {
			command: { name: 'LoadSettings' }
		}).then((result) => {
			const settings = JSON.parse(result);
			settingsStore.set(settings);
		});
	},

	saveSettings(): Promise<void> {
		const currentState = get(settingsStore);
		return invoke('execute_command', {
			command: {
				name: 'SaveSettings',
				settings: currentState
			}
		});
	},

	updateBasicSettingsChanged(): void {
		const envState = get(EnvStore);
		const currentState = get(settingsStore);

		const currentBasicSettings = {
			language: currentState.language,
			theme: currentState.theme,
			minimize_to_tray_on_close: currentState.minimize_to_tray_on_close
		};

		if (envState.initialSettings) {
			const initialBasicSettings = {
				language: envState.initialSettings.language,
				theme: envState.initialSettings.theme,
				minimize_to_tray_on_close: envState.initialSettings.minimize_to_tray_on_close
			};

			EnvStore.update((state) => ({
				...state,
				isBasicSettingsChanged:
					JSON.stringify(currentBasicSettings) !== JSON.stringify(initialBasicSettings)
			}));
		}
	},

	// Update theme mode and apply changes to DOM based on current theme setting
	updateThemeMode(): void {
		const currentState = get(settingsStore);

		// Get dark mode status based on current theme setting
		const isDarkMode =
			currentState.theme === 'dark' ||
			(currentState.theme === 'system' &&
				window.matchMedia('(prefers-color-scheme: dark)').matches);

		// Update DOM
		if (isDarkMode) {
			console.log('Dark mode enabled');
			document.documentElement.classList.add('dark');
		} else {
			console.log('Light mode enabled');
			document.documentElement.classList.remove('dark');
		}

		EnvStore.update((state) => ({
			...state,
			isDarkMode
		}));

		// Setup system theme change listener
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

		// Remove any existing listener first to prevent duplicates
		mediaQuery.removeEventListener('change', this.handleSystemThemeChange);

		// Only add listener when theme is set to 'system'
		if (currentState.theme === 'system') {
			mediaQuery.addEventListener('change', (event: MediaQueryListEvent) => {
				// Update isDarkMode and DOM when system theme changes
				EnvStore.update((state) => ({
					...state,
					isDarkMode: event.matches
				}));

				if (event.matches) {
					document.documentElement.classList.add('dark');
				} else {
					document.documentElement.classList.remove('dark');
				}
			});
		}
	},

	handleSystemThemeChange: () => {
		// Placeholder for the system theme change handler
	}
};
