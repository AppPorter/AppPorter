<script setup lang="ts">
import { goTo } from "@/plugins/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { invoke } from "@tauri-apps/api/core";
import Button from "primevue/button";
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
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

const toast = useToast();
const confirm = useConfirm();

function handleDetailsProgress(value: number) {
  progressMode.value = "indeterminate";
  detailsLoadingProgress.value = value;
}

async function handleInstallClick() {
  // Reset errors
  nameError.value = "";
  pathError.value = "";

  const is_legal = ref(true);

  // Validate executable selection
  if (!installationConfig.executable_path) {
    toast.add({
      severity: "error",
      summary: "Executable Missing",
      detail: "Please select an executable file from the archive",
      life: 3000,
    });
    is_legal.value = false;
  }

  // Check app name
  if (!installationConfig.app_name) {
    nameError.value = "Application name is required";
    toast.add({
      severity: "error",
      summary: "Missing App Name",
      detail: "Please enter an application name",
      life: 3000,
    });
    is_legal.value = false;
  }

  // Check installation path
  if (!installationConfig.install_path) {
    pathError.value = "Installation path is required";
    toast.add({
      severity: "error",
      summary: "Missing Install Path",
      detail: "Please select an installation path",
      life: 3000,
    });
    is_legal.value = false;
  }

  if (!is_legal.value) return;

  try {
    await new Promise((resolve, reject) => {
      confirm.require({
        message: "Do you want to start the installation process now?",
        group: "dialog",
        header: "Start Installation",
        rejectProps: {
          label: "Cancel",
          severity: "secondary",
          outlined: true,
        },
        acceptProps: {
          label: "Install",
        },
        accept: () => resolve(true),
        reject: () => reject(),
      });
    });

    await invoke("execute_command", {
      command: {
        name: "ValidatePath",
        path: installationConfig.install_path,
      },
    });
    goTo("/Installation/Progress");
  } catch (error) {
    if (error instanceof Error) {
      pathError.value = error.message;
      toast.add({
        severity: "error",
        summary: "Invalid Install Path",
        detail: error.message,
        life: 3000,
      });
    }
  }
}
</script>

<template>
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
  <div class="min-w-[350px] w-[40%] h-[calc(100vh-170px)]">
    <ZipPreview
      :zip-path="zip_path"
      :details-loading="detailsLoading"
      @loading="(value) => (detailsLoading = value)"
      @progress="handleDetailsProgress"
    />
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
