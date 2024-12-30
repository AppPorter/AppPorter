<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const install_mode = ref('AllUser');

</script>

<template>
    <h1 class="text-4xl">Installation Option</h1>

    <form>
        <div class="join">
            <input type="radio" name="install_mode" class="btn join-item" aria-label="All User" value="AllUsers"
                v-model="install_mode" />
            <input type="radio" name="install_mode" class="btn join-item" aria-label="Current User" value="CurrentUser"
                v-model="install_mode" />
        </div>
        <div class="form-control">
            <label class="label cursor-pointer">
                <span class="label-text">{{ $t('create_start_menu_shortcut') }}</span>
                <input type="checkbox" checked="checked" class="checkbox" value=true
                    v-model="create_start_menu_shortcut" />
            </label>
            <label class="label cursor-pointer">
                <span class="label-text">{{ $t('create_desktop_shortcut') }}</span>
                <input type="checkbox" checked="checked" class="checkbox" value=false
                    v-model="create_desktop_shortcut" />
            </label>
            <label class="label cursor-pointer">
                <span class="label-text">{{ $t('create_registry_key') }}</span>
                <input type="checkbox" checked="checked" class="checkbox" value=true v-model="create_registry_key" />
            </label>
        </div>
    </form>

    <button class="btn" @click="install">Install</button>
</template>