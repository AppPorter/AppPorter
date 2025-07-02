<script setup lang="ts">
import { exec } from '@/exec';
import { generalStore, installConfig, settingsStore } from '@/main';
import { goTo } from '@/router';
import { open } from '@tauri-apps/plugin-dialog';
import { storeToRefs } from 'pinia';
import Button from 'primevue/button';
import { useConfirm } from 'primevue/useconfirm';
import { computed, ref, toRef } from 'vue';
import { useI18n } from 'vue-i18n';

const { archive_exe_path, zip_path, app_details } = storeToRefs(installConfig)
const { t } = useI18n()

const name = toRef(app_details.value.info, 'name')
const icon = toRef(app_details.value.info, 'icon')
const publisher = toRef(app_details.value.info, 'publisher')
const version = toRef(app_details.value.info, 'version')
const custom_icon = toRef(app_details.value.config, 'custom_icon')
const current_user_only = toRef(app_details.value, 'current_user_only')
const create_desktop_shortcut = toRef(app_details.value.config, 'create_desktop_shortcut')
const create_registry_key = toRef(app_details.value.config, 'create_registry_key')
const create_start_menu_shortcut = toRef(app_details.value.config, 'create_start_menu_shortcut')
const add_to_path = toRef(app_details.value.config, 'add_to_path')
const install_path = toRef(app_details.value, 'install_path')

let originalIcon = icon.value
if (!custom_icon.value) originalIcon = icon.value

const { current_user_only: settings_current_user_only, all_users, current_user } = settingsStore.app_install
const directoryDrawerVisible = ref(false)
const installModeOptions = [
  { label: t('cls.install.modes.current_user'), value: true },
  { label: t('cls.install.modes.all_users'), value: false },
]
const formatted_app_path = computed(() => {
  if (!install_path.value || !name.value) return ''
  return `${install_path.value}\\${name.value}`
})
function updateConfig(isCurrentUser: boolean) {
  const src = isCurrentUser ? current_user : all_users
  create_desktop_shortcut.value = src.create_desktop_shortcut
  create_registry_key.value = src.create_registry_key
  create_start_menu_shortcut.value = src.create_start_menu_shortcut
  add_to_path.value = [src.add_to_path, '']
  install_path.value = src.install_path
}
current_user_only.value = settings_current_user_only
updateConfig(current_user_only.value)
async function select_install_path() {
  const selected = await open({ directory: true, multiple: false })
  if (selected) install_path.value = String(selected)
}
function handleInstallModeChange(e: { value: boolean }) {
  current_user_only.value = e.value
  updateConfig(e.value)
}
async function selectIcon() {
  const selected = await open({ multiple: false, filters: [{ name: 'Icon Files', extensions: ['ico', 'png', 'exe'] }] })
  if (selected) {
    if (!custom_icon.value) originalIcon = icon.value
    icon.value = await exec<string>('ConvertIconToBase64', { path: selected })
    custom_icon.value = true
  }
}
function restoreOriginalIcon() {
  icon.value = originalIcon
  custom_icon.value = false
}

const confirm = useConfirm()
generalStore.page = 'Install_App_Config'
const pathError = ref(false)
const nameError = ref(false)

function handleBackClick() { goTo('/Home') }

async function handleInstallClick() {
  nameError.value = !name.value
  pathError.value = !install_path.value
  try {
    const validatedPath = await exec('ValidatePath', { path: install_path.value })
    install_path.value = `${validatedPath}\\${name.value}`
  } catch (error) {
    pathError.value = true
    globalThis.$errorHandler.showError(error)
  }
  if (nameError.value || pathError.value) return
  try {
    await exec('CheckPathEmpty', { path: install_path.value })
    await new Promise((resolve, reject) => {
      confirm.require({
        message: t('ui.install.confirm_install', { name: name.value }),
        group: 'dialog',
        icon: 'mir-install_desktop',
        header: t('ui.install.start_install'),
        rejectProps: { label: t('g.cancel'), severity: 'secondary', outlined: true, icon: 'mir-close' },
        acceptProps: { label: t('cls.install.self'), icon: 'mir-navigate_next' },
        accept: () => resolve(true),
        reject: () => reject(),
      })
    })
  } catch (error) {
    if (error == 'Directory is not empty') {
      await new Promise((resolve, reject) => {
        confirm.require({
          message: t('ui.install.force_install_confirm', { name: name.value }),
          group: 'dialog',
          icon: 'mir-warning',
          header: t('g.warning'),
          rejectProps: { label: t('g.cancel'), severity: 'secondary', outlined: true, icon: 'mir-close' },
          acceptProps: { label: t('ui.install.force_install'), severity: 'danger', icon: 'mir-install_desktop' },
          accept: () => resolve(true),
          reject: () => reject(),
        })
      })
    } else {
      globalThis.$errorHandler.showError(error)
      return
    }
  }
  goTo('/Install/App/Progress')
}
</script>

<template>
  <div class="flex size-full flex-col overflow-hidden">
    <div class="flex-1 overflow-auto">
      <div class="flex flex-wrap gap-4 px-1 md:flex-nowrap">
        <div class="min-w-72 flex-1 space-y-2">
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
                      <Button type="button" severity="secondary" class="h-8 w-24" @click="selectIcon"
                        icon="mir-folder_open" :label="t('g.browse')" />
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
                  <InputText v-model="publisher" :placeholder="t('cls.install.app.publisher')"
                    class="h-8 w-full text-sm" />
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
          </Panel>
          <Panel class="shadow-sm">
            <template #header>
              <div class="flex flex-col">
                <div class="flex items-center gap-1.5">
                  <span class="mir-settings" />
                  <h2 class="text-base font-medium">
                    {{ t('ui.install.config.install_options') }}
                  </h2>
                </div>
              </div>
            </template>
            <div class="space-y-2 p-1">
              <div class="flex items-center gap-2">
                <label class="w-24 text-sm font-medium">{{ t('cls.install.modes.self') }}</label>
                <div class="flex w-full items-center gap-2 rounded-lg px-2 py-1">
                  <SelectButton v-model="current_user_only" :options="installModeOptions" optionLabel="label"
                    optionValue="value" :allowEmpty="false" size="small" @change="handleInstallModeChange"
                    class="w-64" />
                </div>
              </div>
              <div class="flex items-center gap-2">
                <label class="w-24 text-sm font-medium">{{ t('cls.install.config.install_path') }}</label>
                <div class="w-full">
                  <div class="flex flex-1 gap-2">
                    <InputText v-model="install_path" :placeholder="t('ui.select_placeholder.dir')"
                      class="h-8 w-full text-sm" :invalid="pathError" @input="$emit('update:pathError', false)" />
                    <Button class="h-8 w-36" severity="secondary" @click="select_install_path" icon="mir-folder_open"
                      :label="t('g.browse')" />
                  </div>
                  <div v-if="install_path && name" class="mt-1 text-xs text-gray-500">
                    {{ t('ui.install.final_path') }}: {{ formatted_app_path }}
                  </div>
                </div>
              </div>
              <div class="flex items-start gap-2">
                <label class="mt-1 w-24 text-sm font-medium">{{ t('g.option') }}</label>
                <div class="w-full">
                  <div class="flex-1 space-y-1 rounded-lg p-1.5">
                    <div class="flex items-center gap-2">
                      <Checkbox v-model="create_desktop_shortcut" :binary="true" inputId="desktop_shortcut" />
                      <label for="desktop_shortcut" class="text-sm">{{ t('cls.install.shortcuts.desktop') }}</label>
                    </div>
                    <div class="flex items-center gap-2">
                      <Checkbox v-model="create_start_menu_shortcut" :binary="true" inputId="start_menu_shortcut" />
                      <label for="start_menu_shortcut" class="text-sm">{{ t('cls.install.shortcuts.start_menu')
                        }}</label>
                    </div>
                    <div class="flex items-center gap-2">
                      <Checkbox v-model="create_registry_key" :binary="true" inputId="registry_key" />
                      <label for="registry_key" class="text-sm">{{ t('cls.install.shortcuts.registry_key') }}</label>
                    </div>
                    <div class="flex flex-col gap-1">
                      <div class="flex items-center gap-2">
                        <Checkbox v-model="add_to_path[0]" :binary="true" inputId="add_to_path" />
                        <label for="add_to_path" class="text-sm">{{ t('cls.install.shortcuts.add_to_path') }}</label>
                      </div>
                      <div v-if="add_to_path[0]" class="ml-6 mt-1">
                        <div class="flex gap-2">
                          <InputText v-model="add_to_path[1]" :placeholder="t('ui.select_placeholder.path_directory')"
                            class="h-8 w-full text-sm" />
                          <Button class="h-8 w-36" severity="secondary" @click="directoryDrawerVisible = true"
                            icon="mir-folder_open" :label="t('g.browse')" />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </Panel>
        </div>
      </div>
    </div>
    <div class="flex items-center justify-between px-4 py-3">
      <Button severity="secondary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleBackClick"
        icon="mir-arrow_back" :label="t('g.back')" outlined />
      <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleInstallClick"
        icon="mir-install_desktop" :label="t('cls.install.self')" />
    </div>
  </div>
</template>
