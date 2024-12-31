<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";

const router = useRouter();
const goBack = () => {
  router.back();
};
</script>

<template></template>
