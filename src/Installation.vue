<script setup lang="ts">
import { Button } from "@/components/ui/button";
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
  <Button class="relative" @click="select_file">{{ $t("select_file") }}</Button>
  <Button class="relative" @click="goTo('/Installation/Option')"
    >Install</Button
  >
</template>
