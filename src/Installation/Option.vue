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
  <!-- 减小整体页面边距和间距 -->
  <div class="h-[calc(100vh-144px)] p-1.5 pb-12 flex gap-2">
    <!-- Left Column -->
    <div class="flex-1 min-w-[400px] space-y-2">
      <!-- Installation Options Panel -->
      <Panel
        class="shadow-sm border border-surface-200 dark:border-surface-700"
      >
        <template #header>
          <div class="flex flex-col py-0">
            <div class="flex items-center gap-1.5">
              <span
                class="material-symbols-rounded text-lg text-surface-600 dark:text-surface-400"
                >settings</span
              >
              <h2
                class="text-base font-medium text-surface-900 dark:text-surface-0"
              >
                Installation Options
              </h2>
            </div>
            <p
              class="text-xs text-surface-500 dark:text-surface-400 mt-0.5 ml-6"
            >
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
              <InputSwitch
                v-model="current_user_only"
                @change="handleInstallModeChange"
                class="mx-1"
              />
              <span class="text-sm">Current User</span>
            </div>
          </div>

          <!-- Install Path -->
          <div class="flex items-center gap-2">
            <span class="w-24 text-sm font-medium">Install Path</span>
            <div class="flex-1 flex gap-1">
              <InputText
                v-model="install_path"
                placeholder="Choose installation directory"
                class="w-full text-sm h-8"
              />
              <Button
                class="p-button-secondary h-8"
                @click="select_install_path"
              >
                <span class="material-symbols-rounded">folder_open</span>
                Browse
              </Button>
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

      <!-- App Details Panel -->
      <Panel
        class="shadow-sm border border-surface-200 dark:border-surface-700 relative overflow-hidden"
      >
        <template #header>
          <div class="flex justify-between items-center w-full py-0">
            <div class="flex items-center gap-1.5">
              <span
                class="material-symbols-rounded text-lg text-surface-600 dark:text-surface-400"
                >apps</span
              >
              <h2
                class="text-base font-medium text-surface-900 dark:text-surface-0"
              >
                App Details
              </h2>
            </div>
            <div
              class="w-8 h-8 rounded-lg overflow-hidden shrink-0 border-2"
              :class="[
                !app_icon
                  ? 'border-dashed border-surface-300 dark:border-surface-600 bg-surface-50 dark:bg-surface-800'
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
                class="material-symbols-rounded w-full h-full flex items-center justify-center text-surface-400 dark:text-surface-500"
              >
                image
              </span>
            </div>
          </div>
        </template>

        <div
          class="space-y-3 p-3 bg-gradient-to-b from-surface-0 dark:from-surface-900 to-surface-50/50 dark:to-surface-800/50"
        >
          <!-- App Name -->
          <div class="flex items-center gap-2">
            <span class="w-24 text-sm font-medium">App Name</span>
            <InputText
              v-model="app_name"
              placeholder="Application Name"
              class="w-full text-sm h-8"
            />
          </div>

          <!-- Publisher -->
          <div class="flex items-center gap-2">
            <div class="w-24">
              <span
                class="text-sm font-medium text-surface-900 dark:text-surface-0"
                >Publisher</span
              >
              <p class="text-xs text-surface-500 dark:text-surface-400">
                (Optional)
              </p>
            </div>
            <InputText
              v-model="app_publisher"
              placeholder="Publisher Name"
              class="w-full text-sm h-8"
            />
          </div>

          <!-- Version -->
          <div class="flex items-center gap-2">
            <div class="w-24">
              <span
                class="text-sm font-medium text-surface-900 dark:text-surface-0"
                >Version</span
              >
              <p class="text-xs text-surface-500 dark:text-surface-400">
                (Optional)
              </p>
            </div>
            <InputText
              v-model="app_version"
              placeholder="1.0.0"
              class="w-full text-sm h-8"
            />
          </div>
        </div>

        <!-- Loading Overlay -->
        <div
          v-if="detailsLoading"
          class="absolute inset-0 backdrop-blur-[2px] bg-surface-0/60 dark:bg-surface-900/60 flex flex-col items-center justify-center gap-2 transition-all duration-300"
        >
          <h3
            class="text-base font-semibold text-surface-900 dark:text-surface-0"
          >
            App Details
          </h3>
          <ProgressBar
            v-if="detailsLoadingProgress > 0"
            :value="(detailsLoadingProgress / 4) * 100"
            class="w-40"
          />
          <p class="text-sm text-surface-600 dark:text-surface-400">
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
    <div class="min-w-[350px] w-[40%]">
      <ZipPreview
        :zip-path="zip_path"
        :details-loading="detailsLoading"
        @loading="(value) => (detailsLoading = value)"
        @progress="handleDetailsProgress"
      />
    </div>
  </div>

  <!-- Install Button -->
  <div class="fixed bottom-6 right-6 z-40">
    <Button
      severity="primary"
      size="large"
      class="w-28 h-8 text-sm shadow-lg hover:shadow-xl transition-all duration-300"
      @click="start_installation"
    >
      <span class="material-symbols-rounded text-lg mr-1">download</span>
      Install
    </Button>
  </div>
</template>
