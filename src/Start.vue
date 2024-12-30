<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog';

function select_file(): Promise<string | null> {
    return open({
        multiple: false,
        directory: false,
        filters: [
            { name: 'Zip', extensions: ['zip'] },
        ]
    })
}

</script>

<template>
    <button class="" @click="select_file">{{ $t('select_file') }}</button>
</template>