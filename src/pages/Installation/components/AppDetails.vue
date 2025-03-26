<script setup lang="ts">
import { useInstallationConfigStore } from '@/stores/installation_config'
import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import ProgressBar from 'primevue/progressbar'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  nameError: string
  detailsLoading: boolean
  detailsLoadingProgress: number
  progressMode: 'indeterminate' | 'determinate'
}>()

const installationConfig = useInstallationConfigStore()
const { zip_path } = installationConfig
const { name, icon, publisher, version, executable_path } = storeToRefs(installationConfig)
const { t } = useI18n()

const autoConfirmed = ref(false)

// Automatically fetch and update app details from the selected executable
async function confirmSelection() {
  emit('loading', true)
  autoConfirmed.value = false

  const result = await invoke('execute_command', {
    command: {
      name: 'GetDetails',
      path: {
        zip_path,
        executable_path: executable_path.value,
      },
    },
  })

  const details = JSON.parse(result as string)
  ;[name.value, version.value, publisher.value, icon.value] = details

  autoConfirmed.value = true
  emit('loading', false)
}

function clearIcon() {
  icon.value = ''
}
</script>

<template>
  <Panel class="shadow-sm">
    <template #header>
      <div class="flex w-full items-center justify-between">
        <div class="flex items-center gap-1.5">
          <span class="mir-apps"></span>
          <h2 class="text-base font-medium">
            {{ t('installation.app_details.title') }}
          </h2>
        </div>
        <Button
          :severity="autoConfirmed ? 'success' : 'secondary'"
          class="h-8 min-w-[6.5rem] transition-all duration-200"
          :disabled="detailsLoading || executable_path === ''"
          @click="confirmSelection"
          :icon="autoConfirmed ? 'mir-verified' : 'mir-auto_awesome'"
          :label="
            autoConfirmed
              ? t('installation.app_details.auto_filled')
              : t('installation.app_details.auto_fill')
          "
        />
      </div>
    </template>

    <div class="space-y-3 p-3">
      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium">{{ t('installation.app_details.icon') }}</span>
          <p class="text-xs">
            {{ t('installation.app_details.icon_optional') }}
          </p>
        </div>
        <div class="w-full">
          <div class="flex items-center gap-2">
            <div class="group relative">
              <div
                class="flex size-12 items-center justify-center overflow-hidden rounded-lg border border-slate-200 shadow-sm dark:border-zinc-600"
              >
                <img v-if="icon" :src="icon" class="size-12 object-contain" alt="App Icon" />
                <span v-else class="mir-apps text-2xl" />
              </div>
              <Button
                v-if="icon"
                type="button"
                severity="danger"
                text
                raised
                class="invisible !absolute !-right-1.5 !-top-1.5 !h-5 !w-5 !min-w-0 scale-75 !p-0 opacity-0 transition-all duration-200 ease-out hover:scale-110 group-hover:visible group-hover:scale-100 group-hover:opacity-100"
                @click="clearIcon"
              >
                <span class="mir-close !text-xs" />
              </Button>
            </div>
            <span v-if="!icon" class="text-xs">
              {{ t('installation.app_details.icon_extract_hint') }}
            </span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium">{{ t('installation.app_details.name') }}</span>
        </div>
        <div class="w-full">
          <InputText
            v-model="name"
            :placeholder="t('installation.app_details.name')"
            class="h-8 w-full text-sm"
            :invalid="!!nameError"
            :title="nameError"
          />
        </div>
      </div>

      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium">{{ t('installation.app_details.publisher') }}</span>
          <p class="text-xs">
            {{ t('installation.app_details.publisher_optional') }}
          </p>
        </div>
        <div class="w-full">
          <InputText
            v-model="publisher"
            :placeholder="t('installation.app_details.publisher')"
            class="h-8 w-full text-sm"
          />
        </div>
      </div>

      <div class="flex items-center gap-2">
        <div class="w-24">
          <span class="text-sm font-medium">{{ t('installation.app_details.version') }}</span>
          <p class="text-xs">
            {{ t('installation.app_details.version_optional') }}
          </p>
        </div>
        <div class="w-full">
          <InputText
            v-model="version"
            :placeholder="t('installation.app_details.version')"
            class="h-8 w-full text-sm"
          />
        </div>
      </div>
    </div>

    <div
      v-if="detailsLoading"
      class="absolute inset-0 flex flex-col items-center justify-center gap-2 backdrop-blur-[0.125rem]"
    >
      <h3 class="text-base font-semibold">
        {{ t('installation.app_details.loading_details') }}
      </h3>
      <ProgressBar :mode="progressMode" :value="detailsLoadingProgress" class="w-40" />
      <p class="text-sm">
        {{
          [
            t('installation.app_details.preparing'),
            t('installation.app_details.extracting'),
            t('installation.app_details.reading'),
            t('installation.app_details.processing'),
          ][Math.floor(detailsLoadingProgress / 25) - 1] || t('installation.app_details.loading')
        }}
      </p>
    </div>
  </Panel>
</template>
