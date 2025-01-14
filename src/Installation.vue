<script setup lang="ts">
import { goTo } from "@/plugin/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";
import Button from "primevue/button";
import Card from "primevue/card";
import InputText from "primevue/inputtext";

const installationConfig = useInstallationConfigStore();
const { zip_path } = storeToRefs(installationConfig);

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
  <Card class="border shadow-sm">
    <template #title>
      <div class="flex items-center gap-2">
        <!-- 使用 surface 和 primary 语义化颜色 -->
        <div class="p-1 rounded-md bg-primary-50">
          <span class="material-symbols-rounded text-xl text-primary-600">
            folder_zip
          </span>
        </div>
        <div>
          <h2
            class="text-lg font-medium leading-tight text-surface-900 dark:text-surface-0"
          >
            Select Installation Package
          </h2>
          <p class="text-xs text-surface-600 dark:text-surface-400 mt-0.5">
            Choose a zip file to install
          </p>
        </div>
      </div>
    </template>

    <template #content>
      <div class="flex flex-col gap-3 py-2">
        <div class="flex items-center gap-2">
          <InputText
            v-model="zip_path"
            placeholder="Select a zip file to install"
            class="flex-1 text-sm h-9"
            readonly
          />
          <Button
            @click="select_zip_file"
            severity="secondary"
            class="min-w-24 h-9 bg-white hover:bg-gray-50"
          >
            <span class="material-symbols-rounded mr-1 text-lg"
              >folder_open</span
            >
            Browse
          </Button>
        </div>
      </div>
    </template>

    <template #footer>
      <div
        class="flex justify-end border-t border-surface-200 dark:border-surface-700 -mx-3 -mb-3 px-3 pt-3 bg-surface-50 dark:bg-surface-800"
      >
        <Button
          @click="goTo('/Installation/Option')"
          :disabled="!zip_path"
          severity="primary"
          class="min-w-24 h-9"
        >
          <span class="material-symbols-rounded mr-1 text-lg"
            >navigate_next</span
          >
          Next
        </Button>
      </div>
    </template>
  </Card>
</template>
