<script setup lang="ts">
import { useConfirm } from 'primevue/useconfirm'
import { useI18n } from 'vue-i18n'
import { SettingsStore } from '@/stores/settings.ts'
import { exec } from '@/exec.ts'
import { onMounted } from 'vue'

const confirm = useConfirm()
const { t } = useI18n()
const settings = SettingsStore()

onMounted(async () => {
    if (settings.first_run) {
        confirm.require({
            group: 'disclaimer',
            header: t('ui.disclaimer.header'),
            message: t('ui.disclaimer.msg'),
            icon: 'mir-info',
            acceptProps: {
                label: t('g.accept'),
                icon: 'mir-check',
                severity: 'primary',
            },
            rejectProps: {
                label: t('g.exit'),
                icon: 'mir-close',
                severity: 'secondary',
                outlined: true,
            },
            accept: async () => {
                await settings.acknowledgeFirstRun()
            },
            reject: () => {
                exec('Exit', {
                    code: 0
                })
            },
        })
    }
})
</script>

<template>
    <ConfirmDialog group="disclaimer" class="w-[32rem] max-w-[90vw]" :closable="false" />
</template>
