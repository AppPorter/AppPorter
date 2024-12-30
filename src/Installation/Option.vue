<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const install_mode = ref('AllUser');

</script>

<template>
    <h1 class="text-2xl">Installation Option</h1>

    <Button class="" @click="install">Install</Button>
</template>