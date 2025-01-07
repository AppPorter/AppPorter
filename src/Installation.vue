<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { goTo } from "@/plugin/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";

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
  <CardHeader class="pb-3">
    <CardTitle class="text-lg">Select Installation Package</CardTitle>
    <p class="text-xs text-muted-foreground">Choose a zip file to install</p>
  </CardHeader>

  <CardContent class="space-y-4">
    <div class="flex items-center gap-2">
      <Input
        v-model="zip_path"
        type="text"
        placeholder="Select a zip file to install"
        class="flex-1 text-sm"
      >
        <span v-svg="'zip'" class="w-4 h-4 mr-2 opacity-70"></span>
      </Input>
      <Button variant="secondary" @click="select_zip_file">
        <span v-svg="'folder_open'" class="w-4 h-4"></span>
        Browse
      </Button>
    </div>
  </CardContent>

  <CardFooter class="flex justify-end">
    <Button
      class="w-28"
      @click="goTo('/Installation/Option')"
      :disabled="!installationConfig.zip_path"
    >
      Next
    </Button>
  </CardFooter>
</template>
