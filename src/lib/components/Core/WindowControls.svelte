<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { get } from 'svelte/store';
	import { SettingsStore } from '../../stores/settings';

	const window = getCurrentWindow();

	// Window control handlers
	function handleClose() {
		const settings = get(SettingsStore);
		if (settings.minimize_to_tray_on_close) {
			window.hide();
		} else {
			invoke('execute_command', {
				command: { name: 'Exit' }
			});
		}
	}

	function handleMinimize() {
		window.minimize();
	}
</script>

<div class="fixed right-0 top-0 z-50 flex h-auto">
	<button
		class="flex h-8 w-12 items-center justify-center hover:bg-[#e9e9e9] dark:hover:bg-[#2d2d2d]"
		on:click={handleMinimize}
	>
		<span class="mir-remove" />
	</button>
	<button
		class="group flex h-8 w-12 items-center justify-center hover:bg-[#c42b1c]"
		on:click={handleClose}
	>
		<span class="mir-close group-hover:text-white" />
	</button>
</div>
