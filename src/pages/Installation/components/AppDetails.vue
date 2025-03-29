<script setup lang="ts">
import ExecutableSelector from '@/components/ExecutableSelector.vue'
import { useInstallationConfigStore } from '@/stores/installation_config'
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import Divider from 'primevue/divider'
import Drawer from 'primevue/drawer'
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

function clearIcon() {
  icon.value = ''
}

const drawerVisible = ref(false)
</script>

<template>
  <Panel class="shadow-sm">
    <template #header>
      <div class="flex flex-col">
        <div class="flex items-center gap-1.5">
          <span class="mir-apps text-lg" />
          <h2 class="text-base font-medium">
            {{ t('installation.app_details.title') }}
          </h2>
        </div>
        <p class="ml-6 mt-0.5 text-xs">
          {{ t('installation.config.selected_file') }}:
          <span class="break-all font-medium">{{ zip_path }}</span>
        </p>
      </div>
    </template>
    <div class="space-y-2 p-2">
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{
          t('installation.config.executable_path')
        }}</label>
        <div class="w-full">
          <div class="flex flex-1 gap-2">
            <InputText
              v-model="executable_path"
              :placeholder="t('installation.config.choose_dir')"
              class="h-8 w-full text-sm"
            />
            <Button
              class="h-8 w-36"
              severity="secondary"
              @click="drawerVisible = true"
              icon="mir-folder_open"
              :label="t('installation.config.browse')"
            />
          </div>
        </div>
      </div>
    </div>
    <Divider />
    <div class="space-y-2 p-2">
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">
          {{ t('installation.app_details.icon') }}
          <p class="text-xs font-normal">{{ t('installation.app_details.icon_optional') }}</p>
        </label>
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
        <label class="w-24 text-sm font-medium">{{ t('installation.app_details.name') }}</label>
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
        <label class="w-24 text-sm font-medium">
          {{ t('installation.app_details.publisher') }}
          <p class="text-xs font-normal">{{ t('installation.app_details.publisher_optional') }}</p>
        </label>
        <div class="w-full">
          <InputText
            v-model="publisher"
            :placeholder="t('installation.app_details.publisher')"
            class="h-8 w-full text-sm"
          />
        </div>
      </div>

      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">
          {{ t('installation.app_details.version') }}
          <p class="text-xs font-normal">{{ t('installation.app_details.version_optional') }}</p>
        </label>
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
  <Drawer
    v-model:visible="drawerVisible"
    header="Select Executable"
    position="bottom"
    :style="{ height: '95vh' }"
    class="rounded-lg"
  >
    <div class="h-full overflow-hidden">
      <ExecutableSelector
        :zip-path="zip_path"
        :details-loading="detailsLoading"
        @close="drawerVisible = false"
      />
    </div>
  </Drawer>
</template>
