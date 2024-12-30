<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router';
import { Button } from '@/components/ui/button'

const router = useRouter();
const goToSettings = () => {
    router.push('/Settings');
};

</script>

<template>
    <p class='absolute bottom-2 left-2'>{{ $route.fullPath }}</p>
    <Button v-if="$route.fullPath != '/Settings'" class="absolute top-2 right-2" @click="goToSettings"><span
            v-svg="'setting'"></span></Button>
    <Button>
        <RouterLink class="absolute bottom-2 right-2" to="/Installation/Option">Go to Option</RouterLink>
    </Button>
    <RouterView></RouterView>
</template>