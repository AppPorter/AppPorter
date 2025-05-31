import { invoke } from '@tauri-apps/api/core';
import { get, writable } from 'svelte/store';
import type { Settings } from './settings';

interface Env {
	first_run: boolean;
	debug: boolean;
	elevated: boolean;
	system_drive_letter: string;
	user_sid: string;
	username: string;
	initialSettings: Settings | null;
	isBasicSettingsChanged: boolean;
	isDarkMode: boolean;
}

// Create writable store
const envStore = writable<Env>({
	first_run: true,
	debug: false,
	elevated: false,
	system_drive_letter: '',
	user_sid: '',
	username: '',
	initialSettings: null,
	isBasicSettingsChanged: false,
	isDarkMode: window.matchMedia('(prefers-color-scheme: dark)').matches
});

export const EnvStore = {
	subscribe: envStore.subscribe,
	update: envStore.update,
	set: envStore.set,

	loadEnv(): Promise<void> {
		return invoke<string>('execute_command', {
			command: { name: 'LoadEnv' }
		}).then((result) => {
			const env = JSON.parse(result);
			envStore.update((state) => ({ ...state, ...env }));
		});
	},

	saveEnv(): Promise<void> {
		const currentState = get(envStore);

		return invoke('execute_command', {
			command: {
				name: 'SaveEnv',
				env: currentState
			}
		});
	},

	setInitialSettings(settings: Settings) {
		envStore.update((state) => ({
			...state,
			initialSettings: settings
		}));
	},

	acknowledgeFirstRun(): Promise<void> {
		envStore.update((state) => ({
			...state,
			first_run: false
		}));
		return this.saveEnv();
	}
};
