<script setup lang="ts">
import { goTo } from "@/plugin/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";
import Button from "primevue/button";

const installationConfig = useInstallationConfigStore();
const { zip_path } = storeToRefs(installationConfig);
async function select_zip_file() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "Zip", extensions: ["zip"] }],
  });
  // Only update if a file was selected
  if (selected) {
    zip_path.value = selected;
  }
}
</script>

<template>
  <h1 class="text-lg">Select Installation Package</h1>
  <p class="text-xs text-muted-foreground">Choose a zip file to install</p>

  <div class="flex items-center gap-2">
    <InputText
      type="text"
      v-model="zip_path"
      placeholder="Select a zip file to install"
      class="flex-1 text-sm"
    />
    <Button label="Browse" severity="secondary" @click="select_zip_file" />
  </div>

  <CardFooter class="flex justify-end">
    <Button
      @click="goTo('/Installation/Option')"
      :disabled="!installationConfig.zip_path"
      label="Next"
    >
    </Button>
  </CardFooter>
</template>
