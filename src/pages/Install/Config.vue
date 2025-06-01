<script setup lang="ts">
import { InstallConfigStore } from '@/stores/install_config'
import SelectButton from 'primevue/selectbutton'
import { computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import AppConfig from './App/Config.vue'
import LibConfig from './Lib/Config.vue'

const { t } = useI18n()
const installConfig = InstallConfigStore()

// Mode options for SelectButton
const modeOptions = [
    { label: t('install.mode.app'), value: 'app', icon: 'mir-install_desktop' },
    { label: t('install.mode.library'), value: 'lib', icon: 'mir-folder_copy' }
]

// Current mode based on install_config page
const currentMode = computed({
    get: () => {
        return installConfig.page === 'Install_App_Config' ? 'app' : 'lib'
    },
    set: (value: string) => {
        installConfig.page = value === 'app' ? 'Install_App_Config' : 'Install_Lib_Config'
    }
})

onMounted(() => {
    // Set default mode if not already set
    if (!installConfig.page || (installConfig.page !== 'Install_App_Config' && installConfig.page !== 'Install_Lib_Config')) {
        installConfig.page = 'Install_App_Config'
    }
})
</script>

<template>
    <div class="flex size-full flex-col overflow-hidden">
        <!-- Mode selector header -->
        <div class="flex justify-center border-b border-surface-200 p-3 dark:border-surface-700">
            <SelectButton v-model="currentMode" :options="modeOptions" option-label="label" option-value="value"
                class="flex gap-1">
                <template #option="{ option }">
                    <div class="flex items-center gap-2 px-4 py-2">
                        <span :class="option.icon" class="text-base" />
                        <span class="text-sm font-medium">{{ option.label }}</span>
                    </div>
                </template>
            </SelectButton>
        </div>

        <!-- Dynamic content based on selected mode -->
        <div class="flex-1 overflow-hidden">
            <AppConfig v-if="currentMode === 'app'" />
            <LibConfig v-else />
        </div>
    </div>
</template>