<script setup lang="ts">
import { goTo } from "@/plugin/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Button from "primevue/button";
import Card from "primevue/card";
import Panel from "primevue/panel";
import ProgressBar from "primevue/progressbar";
import Tooltip from "primevue/tooltip";
import { useToast } from "primevue/usetoast";
import { onMounted, ref } from "vue";

const progress_mode = ref<"indeterminate" | "determinate">("indeterminate");
const extract_progress = ref(0);
const is_finished = ref(false);
const current_status = ref("Preparing installation...");
const can_close = ref(false);

const installationConfig = useInstallationConfigStore();

const getInstallMode = (isCurrentUser: boolean) =>
  isCurrentUser ? "Current User Only" : "All Users";

interface ShortcutsConfig {
  create_desktop_shortcut: boolean;
  create_start_menu_shortcut: boolean;
  create_registry_key: boolean;
}

const getShortcutsList = (config: ShortcutsConfig) => {
  const shortcuts = [];
  if (config.create_desktop_shortcut) shortcuts.push("Desktop");
  if (config.create_start_menu_shortcut) shortcuts.push("Start Menu");
  if (config.create_registry_key) shortcuts.push("Registry Entry");
  return shortcuts.length ? shortcuts.join(", ") : "None";
};

const toast = useToast();

// Get final executable path
const getFinalExecutablePath = () => {
  const appNameForPath = installationConfig.app_name.replace(/ /g, "-");
  const exePath = installationConfig.executable_path.replace(/\//g, "\\");
  return `${installationConfig.install_path}\\${appNameForPath}\\${exePath}`;
};

// Copy handler
const handleCopy = async (text: string, type: string) => {
  try {
    await navigator.clipboard.writeText(text);
    toast.add({
      severity: "info",
      summary: "Copied",
      detail: `${type} copied to clipboard`,
      life: 2000,
    });
  } catch (error) {
    console.error("Copy failed:", error);
  }
};

onMounted(() => {
  try {
    listen("installation", (event) => {
      if (event.payload === 0) {
        progress_mode.value = "indeterminate";
        current_status.value = "Preparing to extract files...";
      }
      if (event.payload === 101) {
        progress_mode.value = "indeterminate";
        current_status.value = "Installation completed successfully!";
        is_finished.value = true;
        can_close.value = true;
      }
    });

    listen("installation_extract", (event) => {
      progress_mode.value = "determinate";
      extract_progress.value = event.payload as number;
      current_status.value = `Extracting files (${extract_progress.value}%)...`;
    });

    invoke("execute_command", {
      command: {
        name: "Installation",
        config: {
          app_icon: installationConfig.app_icon,
          app_name: installationConfig.app_name,
          app_publisher: installationConfig.app_publisher,
          app_version: installationConfig.app_version,
          current_user_only: installationConfig.current_user_only,
          create_desktop_shortcut: installationConfig.create_desktop_shortcut,
          create_registry_key: installationConfig.create_registry_key,
          create_start_menu_shortcut:
            installationConfig.create_start_menu_shortcut,
          install_path: installationConfig.install_path,
          executable_path: installationConfig.executable_path,
          zip_path: installationConfig.zip_path,
        },
      },
    });
  } catch (error) {
    console.error("Installation failed:", error);
    current_status.value = "Installation failed. Please try again.";
    can_close.value = true;
  }
});

const handleClose = () => {
  goTo("/");
};

// Register the tooltip directive
defineOptions({
  directives: {
    tooltip: Tooltip,
  },
});
</script>

<template>
  <div class="p-1.5 pb-12 flex items-center">
    <Panel
      class="border border-surface-200 dark:border-surface-700 shadow-sm max-w-5xl w-full mx-auto"
    >
      <template #header>
        <div class="flex items-center justify-between py-1 w-full min-w-0">
          <!-- Left: Progress Title -->
          <div class="flex items-center gap-2 min-w-0 flex-shrink">
            <div
              class="p-1.5 rounded-md shrink-0"
              :class="[
                is_finished
                  ? 'bg-green-50 dark:bg-green-900/20'
                  : 'bg-primary-50 dark:bg-primary-900/20',
              ]"
            >
              <span
                class="material-symbols-rounded text-xl"
                :class="[
                  is_finished
                    ? 'text-green-600 dark:text-green-400'
                    : 'text-primary-600 dark:text-primary-400',
                ]"
              >
                {{ is_finished ? "task_alt" : "install_desktop" }}
              </span>
            </div>
            <div class="min-w-0 flex-shrink">
              <h2
                class="text-lg font-medium text-surface-900 dark:text-surface-0 leading-none truncate"
              >
                Installation Progress
              </h2>
              <p
                class="text-xs text-surface-600 dark:text-surface-400 mt-1 truncate"
              >
                Installing application to your system
              </p>
            </div>
          </div>

          <!-- Right: App Details -->
          <div class="flex items-center gap-3 shrink-0 ml-4 select-text">
            <div class="text-right">
              <h3
                class="text-base font-medium text-surface-900 dark:text-surface-0 leading-none"
              >
                {{ installationConfig.app_name }}
              </h3>
              <p class="text-xs text-surface-600 dark:text-surface-400 mt-1">
                {{ installationConfig.app_version || "Version N/A" }}
              </p>
              <p class="text-xs text-surface-500 dark:text-surface-400 mt-0.5">
                {{ installationConfig.app_publisher || "Publisher N/A" }}
              </p>
            </div>
            <div
              class="w-10 h-10 rounded-lg overflow-hidden shrink-0 flex items-center justify-center bg-surface-50 dark:bg-surface-800 border border-surface-200 dark:border-surface-700"
            >
              <img
                v-if="installationConfig.app_icon"
                :src="installationConfig.app_icon"
                class="w-8 h-8 object-contain"
                alt="App Icon"
              />
              <span
                v-else
                class="material-symbols-rounded text-2xl text-surface-400 dark:text-surface-600"
              >
                apps
              </span>
            </div>
          </div>
        </div>
      </template>

      <div class="space-y-4">
        <!-- Progress Section -->
        <div class="space-y-2">
          <!-- Status Text -->
          <p
            class="text-sm"
            :class="[
              is_finished
                ? 'text-green-600 dark:text-green-400'
                : 'text-surface-600 dark:text-surface-400',
            ]"
          >
            {{ current_status }}
          </p>

          <!-- Progress Bar -->
          <ProgressBar
            :mode="progress_mode"
            :value="extract_progress"
            class="h-1.5"
          />
        </div>

        <!-- Final Executable Path Card -->
        <Card
          v-if="is_finished"
          class="shadow-none border border-surface-200 dark:border-surface-700"
        >
          <template #title>
            <div class="flex items-center justify-between w-full py-1">
              <div class="flex items-center gap-2">
                <span
                  class="material-symbols-rounded text-surface-600 dark:text-surface-400"
                >
                  terminal
                </span>
                <span class="text-sm font-medium">Installed Location</span>
              </div>
              <Button
                severity="secondary"
                outlined
                v-tooltip.top="'Copy path'"
                class="w-8 h-7"
                @click="handleCopy(getFinalExecutablePath(), 'Executable path')"
              >
                <i class="material-symbols-rounded">content_copy</i>
              </Button>
            </div>
          </template>
          <template #content>
            <p class="text-sm font-medium break-all select-text">
              {{ getFinalExecutablePath() }}
            </p>
          </template>
        </Card>

        <!-- Installation Details Grid -->
        <div class="grid grid-cols-2 gap-3">
          <!-- Installation Settings Card -->
          <Card
            class="shadow-none border border-surface-200 dark:border-surface-700"
          >
            <template #title>
              <div class="flex items-center justify-between w-full py-1">
                <div class="flex items-center gap-2">
                  <span
                    class="material-symbols-rounded text-surface-600 dark:text-surface-400"
                  >
                    settings
                  </span>
                  <span class="text-sm font-medium">Installation Settings</span>
                </div>
                <Button
                  severity="secondary"
                  outlined
                  v-tooltip.top="'Copy settings'"
                  class="w-8 h-7"
                  @click="
                    handleCopy(
                      `Install Mode: ${getInstallMode(installationConfig.current_user_only)}\nShortcuts: ${getShortcutsList(installationConfig)}\nInstall Path: ${installationConfig.install_path}`,
                      'Settings',
                    )
                  "
                >
                  <i class="material-symbols-rounded">content_copy</i>
                </Button>
              </div>
            </template>
            <template #content>
              <div class="space-y-3 select-text">
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400"
                    >Install Mode</span
                  >
                  <p class="text-sm font-medium">
                    {{ getInstallMode(installationConfig.current_user_only) }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400"
                    >Shortcuts</span
                  >
                  <p class="text-sm font-medium">
                    {{ getShortcutsList(installationConfig) }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400"
                    >Install Path</span
                  >
                  <p class="text-sm font-medium break-all">
                    {{ installationConfig.install_path }}
                  </p>
                </div>
              </div>
            </template>
          </Card>

          <!-- Package Information Card -->
          <Card
            class="shadow-none border border-surface-200 dark:border-surface-700"
          >
            <template #title>
              <div class="flex items-center justify-between w-full py-1">
                <div class="flex items-center gap-2">
                  <span
                    class="material-symbols-rounded text-surface-600 dark:text-surface-400"
                  >
                    folder_zip
                  </span>
                  <span class="text-sm font-medium">Package Information</span>
                </div>
                <Button
                  severity="secondary"
                  outlined
                  v-tooltip.top="'Copy package info'"
                  class="w-8 h-7"
                  @click="
                    handleCopy(
                      `Source Archive: ${installationConfig.zip_path}\nSelected Executable: ${installationConfig.executable_path}`,
                      'Package info',
                    )
                  "
                >
                  <i class="material-symbols-rounded">content_copy</i>
                </Button>
              </div>
            </template>
            <template #content>
              <div class="space-y-3 select-text">
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400"
                    >Source Archive</span
                  >
                  <p class="text-sm font-medium break-all">
                    {{ installationConfig.zip_path }}
                  </p>
                </div>
                <div class="space-y-1">
                  <span class="text-sm text-surface-600 dark:text-surface-400"
                    >Selected Executable</span
                  >
                  <p class="text-sm font-medium break-all">
                    {{ installationConfig.executable_path }}
                  </p>
                </div>
              </div>
            </template>
          </Card>
        </div>

        <!-- Action Buttons -->
        <div class="flex justify-end">
          <Button
            v-if="can_close"
            @click="handleClose"
            :severity="is_finished ? 'success' : 'danger'"
            class="w-24 h-8"
          >
            <span class="material-symbols-rounded mr-1">
              {{ is_finished ? "home" : "close" }}
            </span>
            {{ is_finished ? "Finish" : "Close" }}
          </Button>
        </div>
      </div>
    </Panel>
  </div>
</template>
