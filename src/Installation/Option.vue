<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { useSettingsStore } from "../stores/settings";

let settingsStore = useSettingsStore();
</script>

<template>
  <h1 class="text-2xl relative">Installation Option</h1>

  <Button class="relative" @click="">Install</Button>
</template>
