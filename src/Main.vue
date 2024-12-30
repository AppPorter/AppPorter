<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router';

const router = useRouter();
const goToSettings = () => {
    router.push('/Settings');
};
</script>

<template>
    <p class='absolute bottom-2 left-2'>{{ $route.fullPath }}</p>
    <button v-if="$route.fullPath != '/Settings'" class="btn btn-square absolute top-2 right-2"
        @click="goToSettings"><span v-svg="'setting'"></span></button>
    <RouterView></RouterView>
</template>