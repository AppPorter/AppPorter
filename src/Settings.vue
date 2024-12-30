<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router';

const router = useRouter();
const goBack = () => {
    router.back();
};
</script>

<template>
    <button class="btn btn-square absolute top-2 left-2" @click="goBack"><span v-svg="'left_arrow'"></span></button>
</template>