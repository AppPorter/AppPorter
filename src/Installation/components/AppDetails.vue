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

// Store setup
const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
const { name, publisher, version, executable_path } =
  storeToRefs(installationConfig)

// Auto fill states
const autoConfirmed = ref(false)

async function confirmSelection() {
  emit('loading', true)
  autoConfirmed.value = false

  try {
    const result = await invoke('execute_command', {
      command: {
        name: 'GetDetails',
        path: {
          zip_path: zip_path,
          executable_path: executable_path.value,
        },
      },
    })

    if (typeof result === 'string') {
      const parsedResult = JSON.parse(result)

      if ('error' in parsedResult) {
        throw new Error(parsedResult.error)
      }

      const details = Array.isArray(parsedResult) ? parsedResult : null
      if (!details) {
        throw new Error('Invalid response format')
      }

      await new Promise((resolve) => setTimeout(resolve, 100))

      name.value = details[0]
      version.value = details[1]
      publisher.value = details[2] || ''

      autoConfirmed.value = true
    } else {
      throw new Error('Unexpected response type')
    }
  } catch (error) {
    console.error('Failed to get details:', error)
    autoConfirmed.value = false
  } finally {
    emit('loading', false)
  }
}
</script>

<template>
  <Panel
    class="shadow-sm border border-surface-200 dark:border-surface-700 relative overflow-hidden"
  >
    <template #header>
      <div class="flex justify-between items-center w-full py-0">
        <div class="flex items-center gap-1.5">
          <span
            class="mir apps text-lg text-surface-600 dark:text-surface-400"
          ></span>
          <h2
            class="text-base font-medium text-surface-900 dark:text-surface-0"
          >
            App Details
          </h2>
        </div>
        <Button
          :severity="autoConfirmed ? 'success' : 'secondary'"
          class="h-8 text-sm min-w-[6.5rem] transition-all duration-200"
          :disabled="detailsLoading"
          @click="confirmSelection"
          :icon="autoConfirmed ? 'mir verified' : 'mir auto_awesome'"
          :label="autoConfirmed ? 'Auto Filled' : 'Auto Fill'"
        />
      </div>
    </template>

    <div class="space-y-3 p-3">
      <!-- App Name -->
      <div class="flex items-center gap-2">
        <span class="w-24 text-sm font-medium">App Name</span>
        <InputText
          v-model="name"
          placeholder="Application Name"
          class="w-full text-sm h-8"
          :invalid="!!nameError"
          :title="nameError"
        />
      </div>

      <!-- Publisher -->
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium text-surface-900 dark:text-surface-0"
            >Publisher</span
          >
          <p class="text-xs text-surface-500 dark:text-surface-400">
            (Optional)
          </p>
        </div>
        <InputText
          v-model="publisher"
          placeholder="Publisher Name"
          class="w-full text-sm h-8"
        />
      </div>

      <!-- Version -->
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium text-surface-900 dark:text-surface-0"
            >Version</span
          >
          <p class="text-xs text-surface-500 dark:text-surface-400">
            (Optional)
          </p>
        </div>
        <InputText
          v-model="version"
          placeholder="1.0.0"
          class="w-full text-sm h-8"
        />
      </div>
    </div>

    <!-- Loading Overlay -->
    <div
      v-if="detailsLoading"
      class="absolute inset-0 backdrop-blur-[2px] bg-surface-0/60 dark:bg-surface-900/60 flex flex-col items-center justify-center gap-2 transition-all duration-300"
    >
      <h3 class="text-base font-semibold text-surface-900 dark:text-surface-0">
        Loading App Details
      </h3>
      <ProgressBar
        :mode="progressMode"
        :value="detailsLoadingProgress"
        class="w-40"
      />
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
