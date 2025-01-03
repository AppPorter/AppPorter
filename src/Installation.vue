<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { goTo } from "@/router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { open } from "@tauri-apps/plugin-dialog";

const installationConfig = useInstallationConfigStore();
let path;
async function select_file() {
  path = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "Zip", extensions: ["zip"] }],
  });
  installationConfig.zip_path = path;
}
</script>

<template>
  <CardHeader class="pb-3">
    <CardTitle class="text-lg">Select Installation Package</CardTitle>
    <p class="text-xs text-muted-foreground">Choose a ZIP file to install</p>
  </CardHeader>

  <CardContent class="space-y-4">
    <div class="flex items-center gap-2">
      <Input
        :value="installationConfig.zip_path"
        type="text"
        placeholder="Select a ZIP file to install"
        readonly
        class="flex-1 text-sm"
      />
      <Button variant="secondary" @click="select_file">Browse</Button>
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
