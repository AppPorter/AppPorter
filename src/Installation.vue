<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { goTo } from "@/router";
import { open } from "@tauri-apps/plugin-dialog";
import { useSettingsStore } from "./stores/settings";

const settingsStore = useSettingsStore();
let path;
async function select_file() {
  path = await open({
    multiple: false,
    directory: false,
    filters: [{ name: "Zip", extensions: ["zip"] }],
  });
  settingsStore.installation.zip_path = path;
}
</script>

<template>
  <Button class="relative" @click="select_file">{{ $t("select_file") }}</Button>
  <Button class="relative" @click="goTo('/Installation/Option')"
    >Install</Button
  >
</template>
