<script setup lang="ts">
import { exec } from '@/exec'
import { open } from '@tauri-apps/plugin-dialog';
import { storeToRefs } from 'pinia';
import { toRef } from 'vue';
import { useI18n } from 'vue-i18n';
import { installConfig } from '@/main';

defineProps<{
  nameError: boolean
  detailsLoading: boolean
  detailsLoadingProgress: number
  progressMode: 'indeterminate' | 'determinate'
}>()

const { archive_exe_path, zip_path, app_details } = storeToRefs(installConfig)

let originalIcon = ''

const name = toRef(app_details.value.info, 'name')
const icon = toRef(app_details.value.info, 'icon')
const publisher = toRef(app_details.value.info, 'publisher')
const version = toRef(app_details.value.info, 'version')
const custom_icon = toRef(app_details.value.config, 'custom_icon')

if (!custom_icon.value) {
  originalIcon = icon.value
}

const { t } = useI18n()

async function selectIcon() {
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'Icon Files',
      extensions: ['ico', 'png', 'exe']
    }]
  })
  if (selected) {
    if (!custom_icon.value) {
      originalIcon = icon.value
    }

    const base64Icon = await exec<string>('ConvertIconToBase64', {
      path: selected
    })

    icon.value = base64Icon
    custom_icon.value = true
  }
}

function restoreOriginalIcon() {
  icon.value = originalIcon
  custom_icon.value = false
}
</script>

<template>
  <Panel class="shadow-sm">
    <template #header>
      <div class="flex flex-col">
        <div class="flex items-center gap-1.5">
          <span class="mir-apps text-lg" />
          <h2 class="text-base font-medium">
            {{ t('ui.install.app_details.title') }}
          </h2>
        </div>
        <p class="ml-6 mt-0.5 text-xs">
          {{ t('ui.install.selected_file') }}:
          <span class="break-all font-medium">{{ zip_path }}</span>
        </p>
        <p class="ml-6 mt-0.5 text-xs">
          {{ t('ui.install.executable_path') }}:
          <span class="break-all font-medium">{{ archive_exe_path }}</span>
        </p>
      </div>
    </template>
    <div class="space-y-2 p-2">
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">
          {{ t('g.icon') }}
          <p class="text-xs font-normal">{{ t('g.optional') }}</p>
        </label>
        <div class="w-full">
          <div class="flex items-center gap-2">
            <div
              class="flex size-12 items-center justify-center overflow-hidden rounded-lg border border-slate-200 shadow-sm dark:border-zinc-600">
              <img v-if="icon" :src="icon" class="size-12 object-contain" alt="App Icon" />
              <span v-else class="mir-apps text-2xl" />
            </div>
            <div class="flex gap-2">
              <Button type="button" severity="secondary" class="h-8 w-24" @click="selectIcon" icon="mir-folder_open"
                :label="t('g.browse')" />
              <Button v-if="custom_icon" type="button" severity="secondary" class="h-8 w-24"
                @click="restoreOriginalIcon" icon="mir-reset_image" :label="t('g.restore')" />
            </div>
            <span v-show="!icon" class="text-xs">
              {{ t('ui.install.app_details.icon_extract_hint') }}
            </span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('cls.install.app.name') }}</label>
        <div class="w-full">
          <InputText v-model="name" :placeholder="t('cls.install.app.name')" class="h-8 w-full text-sm"
            :invalid="nameError" />
        </div>
      </div>

      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">
          {{ t('cls.install.app.publisher') }}
          <p class="text-xs font-normal">{{ t('g.optional') }}</p>
        </label>
        <div class="w-full">
          <InputText v-model="publisher" :placeholder="t('cls.install.app.publisher')" class="h-8 w-full text-sm" />
        </div>
      </div>

      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">
          {{ t('cls.install.app.version') }}
          <p class="text-xs font-normal">{{ t('g.optional') }}</p>
        </label>
        <div class="w-full">
          <InputText v-model="version" :placeholder="t('cls.install.app.version')" class="h-8 w-full text-sm" />
        </div>
      </div>
    </div>

    <div v-if="detailsLoading" class="absolute inset-0 flex items-center justify-center backdrop-blur-[0.125rem]">
      <h3 class="text-base font-semibold">
        {{ t('g.loading') }}
      </h3>
    </div>
  </Panel>
</template>
