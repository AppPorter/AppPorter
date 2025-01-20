<script setup lang="ts">
// Core imports
import { goTo } from "@/plugin/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";

// PrimeVue components
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Panel from "primevue/panel";

// Store initialization
const installationConfig = useInstallationConfigStore();
const { zip_path } = storeToRefs(installationConfig);

// File selection handler
async function select_zip_file() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "Zip", extensions: ["zip"] }],
  });
  if (selected) {
    zip_path.value = selected;
  }
}
</script>

<template>
  <div class="mt-32 flex items-center justify-center">
    <!-- Main Selection Panel -->
    <Panel
      class="w-full max-w-3xl mx-auto shadow-sm border border-surface-200 dark:border-surface-700"
    >
      <!-- Header Section -->
      <template #header>
        <div class="flex items-center gap-2">
          <div class="p-1 rounded-md bg-primary-50">
            <span class="material-symbols-rounded text-xl text-primary-600">
              folder_zip
            </span>
          </div>
          <div>
            <h2
              class="text-lg font-medium text-surface-900 dark:text-surface-0"
            >
              Select Installation Package
            </h2>
            <p class="text-xs text-surface-600 dark:text-surface-400 mt-0.5">
              Choose a zip file to install
            </p>
          </div>
        </div>
      </template>

      <!-- Content Section -->
      <div class="space-y-6">
        <!-- File Selection Input -->
        <div class="flex items-center gap-2">
          <InputText
            v-model="zip_path"
            placeholder="Select a zip file to install"
            class="flex-1 text-sm h-9"
          />
          <Button
            @click="select_zip_file"
            severity="secondary"
            class="h-9 px-4"
          >
            <span class="material-symbols-rounded mr-1.5">folder_open</span>
            Browse
          </Button>
        </div>

        <!-- Navigation Button -->
        <div class="flex justify-end">
          <Button
            @click="goTo('/Installation/Config')"
            :disabled="!zip_path"
            severity="primary"
            class="h-9 px-6"
          >
            <span class="material-symbols-rounded mr-1.5">navigate_next</span>
            Next
          </Button>
        </div>
      </div>
    </Panel>
  </div>
</template>
