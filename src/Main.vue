<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { loadSvg } from './main.ts'

const svgContent = ref('')

onMounted(async () => {
    svgContent.value = await loadSvg('setting.svg')
})
</script>

<template>
    Main
    <button class="btn btn-square"><img class='fill-neutral-50' src="./assets/icons/setting.svg"></img></button>
    <div v-html="svgContent"></div>
</template>