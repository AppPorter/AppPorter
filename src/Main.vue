<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
</script>

<template>
  <main class="container">Main</main>
  <RouterLink to="/Settings">Settings</RouterLink>
  <RouterLink to="/Installation/Config">Config</RouterLink>
  <RouterView></RouterView>
</template>

<style scoped></style>

<style></style>
