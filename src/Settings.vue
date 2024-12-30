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
    <Button class="absolute top-2 left-2" @click="goBack"><span v-svg="'left_arrow'"></span></Button>
</template>