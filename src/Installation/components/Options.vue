<script setup lang="ts">
import { useInstallationConfigStore } from "@/stores/installation_config";
import { useSettingsStore } from "@/stores/settings";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";
import { watchEffect } from "vue";

// PrimeVue components
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import InputText from "primevue/inputtext";
import Panel from "primevue/panel";
import ToggleSwitch from "primevue/toggleswitch";

defineProps<{
  pathError: string;
}>();

// Store initialization and destructuring
const settingsStore = useSettingsStore();
const {
  installation: {
    current_user_only: settings_current_user_only,
    all_users: {
      create_desktop_shortcut: all_create_desktop_shortcut,
      create_registry_key: all_create_registry_key,
      create_start_menu_shortcut: all_create_start_menu_shortcut,
      install_path: all_install_path,
    },
    current_user: {
      create_desktop_shortcut: current_create_desktop_shortcut,
      create_registry_key: current_create_registry_key,
      create_start_menu_shortcut: current_create_start_menu_shortcut,
      install_path: current_install_path,
    },
  },
} = settingsStore;

// Installation config store setup
const installationConfig = useInstallationConfigStore();
const { zip_path } = installationConfig;
const {
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
} = storeToRefs(installationConfig);

// Config update handler based on installation mode
function updateConfig(isCurrentUser: boolean) {
  if (isCurrentUser) {
    create_desktop_shortcut.value = current_create_desktop_shortcut;
    create_registry_key.value = current_create_registry_key;
    create_start_menu_shortcut.value = current_create_start_menu_shortcut;
    install_path.value = current_install_path;
  } else {
    create_desktop_shortcut.value = all_create_desktop_shortcut;
    create_registry_key.value = all_create_registry_key;
    create_start_menu_shortcut.value = all_create_start_menu_shortcut;
    install_path.value = all_install_path;
  }
}

// Initialize with settings
current_user_only.value = settings_current_user_only;
updateConfig(current_user_only.value);

// Event handlers
function handleInstallModeChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  current_user_only.value = checked;
  updateConfig(checked);
}

async function select_install_path() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  if (selected) {
    install_path.value = String(selected);
  }
}

watchEffect(() => {
  updateConfig(current_user_only.value);
});
</script>

<template>
  <Panel class="shadow-sm border border-surface-200 dark:border-surface-700">
    <template #header>
      <div class="flex flex-col py-0">
        <div class="flex items-center gap-1.5">
          <span
            class="mir settings text-lg text-surface-600 dark:text-surface-400"
          ></span>
          <h2
            class="text-base font-medium text-surface-900 dark:text-surface-0"
          >
            Installation Options
          </h2>
        </div>
        <p class="text-xs text-surface-500 dark:text-surface-400 mt-0.5 ml-6">
          Selected file: <span class="font-medium">{{ zip_path }}</span>
        </p>
      </div>
    </template>

    <div
      class="space-y-2 p-2 bg-gradient-to-b from-transparent to-surface-50/5 dark:to-surface-900/5"
    >
      <!-- Install Mode -->
      <div class="flex items-center gap-2">
        <span class="w-24 text-sm font-medium">Install Mode</span>
        <div
          class="flex items-center gap-2 bg-surface-50 dark:bg-surface-800 px-2 py-1 rounded-lg"
        >
          <span class="text-sm">All Users</span>
          <ToggleSwitch
            v-model="current_user_only"
            @change="handleInstallModeChange"
            class="mx-1"
          />
          <span class="text-sm">Current User</span>
        </div>
      </div>

      <!-- Install Path -->
      <div class="flex items-center gap-2 w-full">
        <span class="w-24 text-sm font-medium">Install Path</span>
        <div class="flex-1 flex gap-2">
          <InputText
            v-model="install_path"
            placeholder="Choose installation directory"
            class="w-full text-sm h-8"
            :invalid="!!pathError"
            :title="pathError"
          />
          <Button
            class="h-8 w-36"
            severity="secondary"
            @click="select_install_path"
            icon="mir folder_open"
            label="Browse"
          />
        </div>
      </div>

      <!-- Shortcuts Section -->
      <div class="flex items-start gap-2">
        <span class="w-24 mt-1 text-sm font-medium">Shortcuts</span>
        <div
          class="space-y-1 bg-surface-50 dark:bg-surface-800 p-1.5 rounded-lg flex-1"
        >
          <div class="flex items-center gap-2">
            <Checkbox
              v-model="create_desktop_shortcut"
              :binary="true"
              inputId="desktop_shortcut"
            />
            <label for="desktop_shortcut" class="text-sm"
              >Create Desktop Shortcut</label
            >
          </div>
          <div class="flex items-center gap-2">
            <Checkbox
              v-model="create_start_menu_shortcut"
              :binary="true"
              inputId="start_menu_shortcut"
            />
            <label for="start_menu_shortcut" class="text-sm"
              >Create Start Menu Shortcut</label
            >
          </div>
          <div class="flex items-center gap-2">
            <Checkbox
              v-model="create_registry_key"
              :binary="true"
              inputId="registry_key"
            />
            <label for="registry_key" class="text-sm"
              >Create Registry Entry</label
            >
          </div>
        </div>
      </div>
    </div>
  </Panel>
</template>
