<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { goTo } from '@/main'


</script>

<template>
    <div class="absolute top-8 left-8">
        <h1 class="text-3xl" style="-webkit-app-region: drag;">AppPorter</h1>
        <div class="absolute top-1 right-1">a</div>
        <div class="relative top-2">
            <Tabs default-value="installation" class="w-[400px]">
                <TabsList>
                    <TabsTrigger value="installation" @click="goTo('/Installation')">
                        Installation
                    </TabsTrigger>
                    <TabsTrigger value="option" @click="goTo('/Installation/Option')">
                        Installation/Option
                    </TabsTrigger>
                    <TabsTrigger value="settings" @click="goTo('/Settings')">
                        Settings
                    </TabsTrigger>
                </TabsList>
            </Tabs>
        </div>
        <div class="relative top-2">
            <RouterView class=""></RouterView>
        </div>
    </div>

    <p class='absolute bottom-2 left-2'>{{ $route.fullPath }}</p>
</template>