<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

const install_mode = ref("AllUser");
</script>

<template>
  <h1 class="text-2xl relative">Installation Option</h1>

  <Button class="relative" @click="">Install</Button>
</template>
