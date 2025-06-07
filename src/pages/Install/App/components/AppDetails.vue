<script setup lang="ts">
import ExecutableSelector from '@/components/ZipPreview/ExecutableSelector.vue'
import { InstallConfigStore } from '@/stores/install_config'
import { storeToRefs } from 'pinia'
import { ref, toRef } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  nameError: boolean
  detailsLoading: boolean
  detailsLoadingProgress: number
  progressMode: 'indeterminate' | 'determinate'
  executablePathError?: boolean
}>()

const installConfig = InstallConfigStore()
const { zip_path, app_details } = storeToRefs(installConfig)

// Direct refs to nested properties for v-model binding
const name = toRef(app_details.value.info, 'name')
const icon = toRef(app_details.value.info, 'icon')
const publisher = toRef(app_details.value.info, 'publisher')
const version = toRef(app_details.value.info, 'version')
const executable_path = toRef(app_details.value.config, 'archive_exe_path')

const { t } = useI18n()
const detailsLoading = ref(false)
const drawerVisible = ref(false)

function clearIcon() {
  icon.value = ''
}

function closeDrawer() {
  drawerVisible.value = false
}

function handleDetailsLoading(loading: boolean) {
  detailsLoading.value = loading
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
      </div>
    </template>
    <div class="space-y-2 p-2">
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">{{ t('ui.install.executable_path') }}</label>
        <div class="w-full">
          <div class="flex flex-1 gap-2">
            <InputText v-model="executable_path" :placeholder="t('ui.install.select_executable')"
              class="h-8 w-full text-sm" :invalid="executablePathError" />
            <Button class="h-8 w-36" severity="secondary" @click="drawerVisible = true" icon="mir-folder_open"
              :label="t('g.browse')" />
          </div>
        </div>
      </div>
    </div>
    <Divider />
    <div class="space-y-2 p-2">
      <div class="flex items-center gap-2">
        <label class="w-24 text-sm font-medium">
          {{ t('g.icon') }}
          <p class="text-xs font-normal">{{ t('g.optional') }}</p>
        </label>
        <div class="w-full">
          <div class="flex items-center gap-2">
            <div class="group relative">
              <div
                class="flex size-12 items-center justify-center overflow-hidden rounded-lg border border-slate-200 shadow-sm dark:border-zinc-600">
                <img v-if="icon" :src="icon" class="size-12 object-contain" alt="App Icon" />
                <span v-else class="mir-apps text-2xl" />
              </div>
              <Button v-show="icon" type="button" severity="danger" text raised
                class="!absolute !-right-1.5 !-top-1.5 !h-5 !w-5 !min-w-0 scale-75 !p-0 opacity-0 transition-all duration-200 ease-out hover:scale-110 group-hover:scale-100 group-hover:opacity-100"
                @click="clearIcon">
                <span class="mir-close !text-xs" />
              </Button>
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
  <Drawer v-model:visible="drawerVisible" :header="t('ui.install.select_executable')" position="bottom"
    :style="{ height: '95vh' }" class="rounded-lg">
    <div class="h-full overflow-hidden">
      <ExecutableSelector :zip-path="zip_path" :details-loading="detailsLoading" @close="closeDrawer"
        @loading="handleDetailsLoading" />
    </div>
  </Drawer>
</template>
