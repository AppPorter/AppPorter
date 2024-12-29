<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

</script>

<template>
    Main
    <button class="btn btn-square absolute top-2 right-2"><span v-svg="'setting'"></span></button>
    <RouterView></RouterView>
</template>