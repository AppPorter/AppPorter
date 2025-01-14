<script setup lang="ts">
import { useInstallationConfigStore } from "@/stores/installation_config";
import { useSettingsStore } from "@/stores/settings";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import InputSwitch from "primevue/inputswitch";
import InputText from "primevue/inputtext";
import Panel from "primevue/panel";
import ProgressBar from "primevue/progressbar";
import { ref } from "vue";
import ZipPreview from "./components/ZipPreview.vue";

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

const installationConfig = useInstallationConfigStore();
const { zip_path } = installationConfig;
const {
  app_icon,
  app_name,
  app_publisher,
  app_version,
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
} = storeToRefs(installationConfig);

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

current_user_only.value = settings_current_user_only;
updateConfig(current_user_only.value);

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
  // Only update if a directory was selected
  if (selected) {
    install_path.value = String(selected);
  }
}

async function start_installation() {
  try {
    const {
      app_icon,
      app_name,
      app_publisher,
      app_version,
      current_user_only,
      create_desktop_shortcut,
      create_registry_key,
      create_start_menu_shortcut,
      install_path,
      executable_path,
      zip_path,
    } = installationConfig;
    const arg = JSON.stringify({
      app_icon,
      app_name,
      app_publisher,
      app_version,
      current_user_only,
      create_desktop_shortcut,
      create_registry_key,
      create_start_menu_shortcut,
      install_path,
      executable_path,
      zip_path,
    });
    console.log(arg);
    const result = await invoke("execute_command", {
      command: "Installation",
      arg: arg,
    });
    console.log(result);
  } catch (error) {
    console.error("Failed to install:", error);
  }
}

const detailsLoading = ref(false);
const detailsLoadingProgress = ref(0);

function handleDetailsProgress(value: number) {
  detailsLoadingProgress.value = value;
  console.log("Progress update:", value); // Add logging for debugging
}
</script>

<template>
  <div class="h-[calc(100vh-144px)] p-4 pb-16 flex gap-6">
    <!-- Left Column -->
    <div class="flex-1 min-w-[400px] space-y-6">
      <!-- Installation Options Panel -->
      <Panel
        class="shadow-sm border border-gray-200/80 dark:border-gray-800/80"
      >
        <template #header>
          <div class="flex flex-col py-1">
            <div class="flex items-center gap-3">
              <span class="material-symbols-rounded text-xl opacity-80"
                >settings</span
              >
              <h2 class="text-lg font-medium">Installation Options</h2>
            </div>
            <p class="text-xs opacity-60 mt-1 ml-9">
              Selected file: <span class="font-medium">{{ zip_path }}</span>
            </p>
          </div>
        </template>

        <div
          class="space-y-6 p-6 bg-gradient-to-b from-transparent to-gray-50/5"
        >
          <!-- Install Mode -->
          <div class="flex items-center gap-3">
            <span class="w-28 text-sm font-medium">Install Mode</span>
            <div
              class="flex items-center gap-3 bg-gray-50/50 dark:bg-gray-900/50 px-4 py-2 rounded-lg"
            >
              <span class="text-sm">All Users</span>
              <InputSwitch
                v-model="current_user_only"
                @change="handleInstallModeChange"
                class="mx-1"
              />
              <span class="text-sm">Current User</span>
            </div>
          </div>

          <!-- Install Path -->
          <div class="flex items-center gap-3">
            <span class="w-24 text-sm font-medium">Install Path</span>
            <div class="flex-1 flex gap-2">
              <InputText
                v-model="install_path"
                placeholder="Choose installation directory"
                class="w-full text-sm"
              />
              <Button class="p-button-secondary" @click="select_install_path">
                <span class="material-symbols-rounded mr-2">folder_open</span>
                Browse
              </Button>
            </div>
          </div>

          <!-- Shortcuts Section -->
          <div class="flex items-start gap-3">
            <span class="w-28 mt-1 text-sm font-medium">Shortcuts</span>
            <div
              class="space-y-3 bg-gray-50/50 dark:bg-gray-900/50 p-3 rounded-lg flex-1"
            >
              <div class="flex items-center gap-2">
                <Checkbox
                  v-model="create_desktop_shortcut"
                  :binary="true"
                  inputId="desktop_shortcut"
                />
                <label for="desktop_shortcut">Create Desktop Shortcut</label>
              </div>
              <div class="flex items-center gap-2">
                <Checkbox
                  v-model="create_start_menu_shortcut"
                  :binary="true"
                  inputId="start_menu_shortcut"
                />
                <label for="start_menu_shortcut"
                  >Create Start Menu Shortcut</label
                >
              </div>
              <div class="flex items-center gap-2">
                <Checkbox
                  v-model="create_registry_key"
                  :binary="true"
                  inputId="registry_key"
                />
                <label for="registry_key">Create Registry Entry</label>
              </div>
            </div>
          </div>
        </div>
      </Panel>

      <!-- App Details Panel -->
      <Panel class="shadow-sm border border-gray-200 relative overflow-hidden">
        <template #header>
          <div class="flex justify-between items-center w-full py-1">
            <div class="flex items-center gap-3">
              <span class="material-symbols-rounded text-xl text-gray-600"
                >apps</span
              >
              <h2 class="text-lg font-medium">App Details</h2>
            </div>
            <div
              class="w-12 h-12 rounded-lg overflow-hidden shrink-0 border-2"
              :class="[
                !app_icon
                  ? 'border-dashed border-gray-300 bg-gray-50'
                  : 'border-transparent',
              ]"
            >
              <img
                v-if="app_icon"
                :src="app_icon"
                class="w-full h-full object-contain"
                alt="App Icon"
              />
              <span
                v-else
                class="material-symbols-rounded w-full h-full flex items-center justify-center text-gray-400"
              >
                image
              </span>
            </div>
          </div>
        </template>

        <div class="space-y-6 p-6 bg-gradient-to-b from-white to-gray-50/50">
          <!-- App Name -->
          <div class="flex items-center gap-3">
            <span class="w-24 text-sm font-medium">App Name</span>
            <InputText
              v-model="app_name"
              placeholder="Application Name"
              class="w-full text-sm"
            />
          </div>

          <!-- Publisher -->
          <div class="flex items-center gap-3">
            <div class="w-24">
              <span class="text-sm font-medium">Publisher</span>
              <p class="text-xs text-gray-500">(Optional)</p>
            </div>
            <InputText
              v-model="app_publisher"
              placeholder="Publisher Name"
              class="w-full text-sm"
            />
          </div>

          <!-- Version -->
          <div class="flex items-center gap-3">
            <div class="w-24">
              <span class="text-sm font-medium">Version</span>
              <p class="text-xs text-gray-500">(Optional)</p>
            </div>
            <InputText
              v-model="app_version"
              placeholder="1.0.0"
              class="w-full text-sm"
            />
          </div>
        </div>

        <!-- Loading Overlay with improved visuals -->
        <div
          v-if="detailsLoading"
          class="absolute inset-0 backdrop-blur-[2px] bg-white/60 flex flex-col items-center justify-center gap-3 transition-all duration-300"
        >
          <h3 class="text-lg font-semibold">App Details</h3>
          <ProgressBar
            v-if="detailsLoadingProgress > 0"
            :value="(detailsLoadingProgress / 4) * 100"
            class="w-48"
          />
          <p class="text-sm text-gray-600">
            {{
              detailsLoadingProgress > 0
                ? ["Preparing", "Extracting", "Reading", "Processing"][
                    detailsLoadingProgress - 1
                  ]
                : "Loading..."
            }}
          </p>
        </div>
      </Panel>
    </div>

    <!-- Right Column -->
    <div class="min-w-[340px] w-[32%]">
      <ZipPreview
        :zip-path="zip_path"
        :details-loading="detailsLoading"
        @loading="(value) => (detailsLoading = value)"
        @progress="handleDetailsProgress"
      />
    </div>
  </div>

  <!-- Install Button -->
  <div class="fixed bottom-12 right-14 z-40">
    <Button
      severity="primary"
      size="large"
      class="w-40 h-12 text-base shadow-lg hover:shadow-xl transition-all duration-300"
      @click="start_installation"
    >
      <span class="material-symbols-rounded text-xl mr-2">download</span>
      Install
    </Button>
  </div>
</template>
