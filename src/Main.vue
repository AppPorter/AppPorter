<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router';

const router = useRouter();
const goToSettings = () => {
    router.push('/Settings');
};

import { Button } from '/@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '/@/components/ui/dropdown-menu'
import { Icon } from '@iconify/vue'
import { useColorMode } from '@vueuse/core'
</script>

<template>
    <p class='absolute bottom-2 left-2'>{{ $route.fullPath }}</p>
    <button v-if="$route.fullPath != '/Settings'" class="absolute top-2 right-2" @click="goToSettings"><span
            v-svg="'setting'"></span></button>
    <button>
        <RouterLink class="absolute bottom-2 right-2" to="/Installation/Option">Go to Option</RouterLink>
    </button>
    <RouterView></RouterView>
    <DropdownMenu>
        <DropdownMenuTrigger as-child>
            <Button variant="outline">
                <Icon icon="radix-icons:moon"
                    class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
                <Icon icon="radix-icons:sun"
                    class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
                <span class="sr-only">Toggle theme</span>
            </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
            <DropdownMenuItem @click="mode = 'light'">
                Light
            </DropdownMenuItem>
            <DropdownMenuItem @click="mode = 'dark'">
                Dark
            </DropdownMenuItem>
            <DropdownMenuItem @click="mode = 'auto'">
                System
            </DropdownMenuItem>
        </DropdownMenuContent>
    </DropdownMenu>
</template>