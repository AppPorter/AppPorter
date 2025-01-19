<script setup lang="ts">
import { useInstallationConfigStore } from "@/stores/installation_config";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

const progress_mode = ref<"indeterminate" | "determinate">("indeterminate");
const extract_progress = ref(0);
const is_finished = ref(false);

const installationConfig = useInstallationConfigStore();
onMounted(() => {
  try {
    listen("installation", (event) => {
      if (event.payload == 0) {
        progress_mode.value = "indeterminate";
      }
      if (event.payload == 101) {
        progress_mode.value = "indeterminate";
        is_finished.value = true;
      }
    });
    listen("installation_extract", (event) => {
      progress_mode.value = "determinate";
      extract_progress.value = event.payload as number;
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
    listen("installation", () => {});
    listen("installation_extract", () => {});
  } catch (error) {
    console.error("Failed to install:", error);
  }
});
</script>

<template>
  <ProgressBar
    v-if="!is_finished"
    :mode="progress_mode"
    :value="extract_progress"
  />
</template>
