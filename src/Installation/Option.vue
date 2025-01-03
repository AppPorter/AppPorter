<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { useSettingsStore } from "@/stores/settings";
import { useInstallationConfigStore } from "@/stores/installation_config";

const settingsStore = useSettingsStore();
const installationConfig = useInstallationConfigStore();
</script>

<template>
  <h1 class="text-2xl relative">Installation Option</h1>
  <h2>Install Mode</h2>
  <Button class="relative" @click="">Install</Button>
</template>
