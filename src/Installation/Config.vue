<script setup lang="ts">
import { goTo } from "@/plugin/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { invoke } from "@tauri-apps/api/core";
import Button from "primevue/button";
import { ref } from "vue";

// Components
import AppDetails from "./components/AppDetails.vue";
import Options from "./components/Options.vue";
import ZipPreview from "./components/ZipPreview.vue";

// Store setup
const installationConfig = useInstallationConfigStore();
const { zip_path } = installationConfig;

// Shared states
const detailsLoading = ref(false);
const detailsLoadingProgress = ref(0);
const progressMode = ref<"indeterminate" | "determinate">("indeterminate");

// Path validation states
const pathError = ref("");
const nameError = ref("");

function handleDetailsProgress(value: number) {
  progressMode.value = "indeterminate";
  detailsLoadingProgress.value = value;
}

async function handleInstallClick() {
  // Reset errors
  nameError.value = "";
  pathError.value = "";

  // Check app name
  if (!installationConfig.app_name) {
    nameError.value = "Application name is required";
    return;
  }

  // Check installation path
  if (!installationConfig.install_path) {
    pathError.value = "Installation path is required";
    return;
  }

  try {
    await invoke("execute_command", {
      command: {
        name: "ValidatePath",
        path: installationConfig.install_path,
      },
    });
    goTo("/installation/progress");
  } catch (error) {
    pathError.value = String(error);
  }
}
</script>

<template>
  <div class="h-[calc(100vh-130px)] p-1.5 pb-12 flex gap-2">
    <!-- Left Column: Options & App Details -->
    <div class="flex-1 min-w-[400px] space-y-2">
      <Options
        :path-error="pathError"
        @update:path-error="(val) => (pathError = val)"
      />
      <AppDetails
        :name-error="nameError"
        :details-loading="detailsLoading"
        :details-loading-progress="detailsLoadingProgress"
        :progress-mode="progressMode"
      />
    </div>

    <!-- Right Column: ZIP Preview -->
    <div class="min-w-[350px] w-[40%]">
      <ZipPreview
        :zip-path="zip_path"
        :details-loading="detailsLoading"
        @loading="(value) => (detailsLoading = value)"
        @progress="handleDetailsProgress"
      />
    </div>
  </div>

  <!-- Installation Button -->
  <div class="fixed bottom-6 right-6 z-40">
    <Button
      severity="primary"
      size="large"
      class="w-28 h-8 text-sm shadow-lg hover:shadow-xl transition-all duration-300"
      @click="handleInstallClick"
    >
      <span class="material-symbols-rounded text-lg mr-1">download</span>
      Install
    </Button>
  </div>
</template>
