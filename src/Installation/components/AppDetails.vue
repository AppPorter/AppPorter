<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import { ref } from 'vue'

defineProps<{
  nameError: string
  detailsLoading: boolean
  detailsLoadingProgress: number
  progressMode: 'indeterminate' | 'determinate'
}>()

const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
const { name, icon, publisher, version, executable_path } = storeToRefs(installationConfig)

const autoConfirmed = ref(false)

async function confirmSelection() {
  emit('loading', true)
  autoConfirmed.value = false

  try {
    const result = await invoke('execute_command', {
      command: {
        name: 'GetDetails',
        path: {
          zip_path,
          executable_path: executable_path.value,
        },
      },
    })

    if (typeof result !== 'string') {
      throw new Error('Unexpected response type')
    }

    const parsedResult = JSON.parse(result)

    if ('error' in parsedResult) {
      throw new Error(parsedResult.error)
    }

    const details = Array.isArray(parsedResult) ? parsedResult : null
    if (!details) {
      throw new Error('Invalid response format')
    }

    // Small delay for UI
    await new Promise((resolve) => setTimeout(resolve, 100))

    // Update app details with received data
    ;[name.value, version.value, publisher.value, icon.value] = details

    autoConfirmed.value = true
  } catch (error) {
    console.error('Failed to get details:', error)
    autoConfirmed.value = false
  } finally {
    emit('loading', false)
  }
}

function clearIcon() {
  icon.value = ''
}
</script>

<template>
  <Panel
    class="shadow-sm border border-surface-200 dark:border-surface-700 relative overflow-hidden"
  >
    <template #header>
      <div class="flex justify-between items-center w-full py-0">
        <div class="flex items-center gap-1.5">
          <span class="mir apps text-lg text-surface-600 dark:text-surface-400"></span>
          <h2 class="text-base font-medium text-surface-900 dark:text-surface-0">App Details</h2>
        </div>
        <Button
          :severity="autoConfirmed ? 'success' : 'secondary'"
          class="h-8 text-sm min-w-[6.5rem] transition-all duration-200"
          :disabled="detailsLoading || executable_path === ''"
          @click="confirmSelection"
          :icon="autoConfirmed ? 'mir verified' : 'mir auto_awesome'"
          :label="autoConfirmed ? 'Auto Filled' : 'Auto Fill'"
        />
      </div>
    </template>

    <div class="space-y-3 p-3">
      <!-- App Icon -->
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium text-surface-900 dark:text-surface-0">Icon</span>
          <p class="text-xs text-surface-500 dark:text-surface-400">(Optional)</p>
        </div>
        <div class="w-full">
          <div class="flex items-center gap-2">
            <div class="relative group">
              <div
                class="w-12 h-12 border border-surface-200 dark:border-surface-700 rounded-lg flex items-center justify-center overflow-hidden bg-surface-50 dark:bg-surface-800"
              >
                <img v-if="icon" :src="icon" class="w-12 h-12 object-contain" alt="App Icon" />
                <span v-else class="mir apps text-2xl text-surface-300 dark:text-surface-600" />
              </div>
              <Button
                v-if="icon"
                type="button"
                severity="danger"
                text
                raised
                class="!absolute !-top-1.5 !-right-1.5 !min-w-0 !w-5 !h-5 !p-0 invisible opacity-0 scale-75 group-hover:opacity-100 group-hover:visible group-hover:scale-100 transition-all duration-200 ease-out hover:scale-110"
                @click="clearIcon"
              >
                <span class="mir close !text-xs" />
              </Button>
            </div>
            <span v-if="!icon" class="text-xs text-surface-500 dark:text-surface-400">
              Icon will be automatically extracted from the application
            </span>
          </div>
        </div>
      </div>

      <!-- App Name -->
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium text-surface-900 dark:text-surface-0">Name</span>
        </div>
        <div class="w-full">
          <InputText
            v-model="name"
            placeholder="Application Name"
            class="w-full text-sm h-8"
            :invalid="!!nameError"
            :title="nameError"
          />
        </div>
      </div>

      <!-- Publisher -->
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium text-surface-900 dark:text-surface-0">Publisher</span>
          <p class="text-xs text-surface-500 dark:text-surface-400">(Optional)</p>
        </div>
        <div class="w-full">
          <InputText v-model="publisher" placeholder="Publisher Name" class="w-full text-sm h-8" />
        </div>
      </div>

      <!-- Version -->
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium text-surface-900 dark:text-surface-0">Version</span>
          <p class="text-xs text-surface-500 dark:text-surface-400">(Optional)</p>
        </div>
        <div class="w-full">
          <InputText v-model="version" placeholder="1.0.0" class="w-full text-sm h-8" />
        </div>
      </div>
    </div>

    <!-- Loading Overlay -->
    <div
      v-if="detailsLoading"
      class="absolute inset-0 backdrop-blur-[0.125rem] bg-surface-0/60 dark:bg-surface-900/60 flex flex-col items-center justify-center gap-[0.5rem] transition-all duration-300"
    >
      <h3 class="text-base font-semibold text-surface-900 dark:text-surface-0">
        Loading App Details
      </h3>
      <ProgressBar :mode="progressMode" :value="detailsLoadingProgress" class="w-40" />
      <p class="text-sm text-surface-600 dark:text-surface-400">
        {{
          ['Preparing', 'Extracting', 'Reading', 'Processing'][
            Math.floor(detailsLoadingProgress / 25) - 1
          ] || 'Loading...'
        }}
      </p>
    </div>
  </Panel>
</template>
