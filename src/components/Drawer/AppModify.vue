<script setup lang="ts">
import { exec } from '@/exec';
import { generalStore, libraryStore } from '@/main';
import { goTo } from '@/router';
import Button from 'primevue/button';
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';


const { t } = useI18n();
const id_modifying = ref(generalStore.drawer.app_modify[1]);
const name = ref('');
const icon = ref('');
const publisher = ref('');
const version = ref('');
const custom_icon = ref(false);
const current_user_only = ref(true);
const create_desktop_shortcut = ref(false);
const create_registry_key = ref(false);
const create_start_menu_shortcut = ref(false);
const add_to_path = ref<[boolean, string]>([false, '']);
const install_path = ref('');
const parent_install_path = ref('');
const pathError = ref(false);
const nameError = ref(false);
watch(() => generalStore.drawer.app_modify[0], async (visible) => {
  if (visible) {
    id_modifying.value = generalStore.drawer.app_modify[1];
    if (id_modifying.value) {
      const app = await libraryStore.getApp(id_modifying.value);
      if (app) {
        name.value = app.details.info.name;
        icon.value = app.details.info.icon;
        publisher.value = app.details.info.publisher;
        version.value = app.details.info.version;
        custom_icon.value = app.details.config.custom_icon;
        current_user_only.value = app.details.current_user_only;
        create_desktop_shortcut.value = app.details.config.create_desktop_shortcut;
        create_registry_key.value = app.details.config.create_registry_key;
        create_start_menu_shortcut.value = app.details.config.create_start_menu_shortcut;
        add_to_path.value = [app.details.config.add_to_path[0], app.details.config.add_to_path[1]];
        install_path.value = app.details.install_path;
        parent_install_path.value = app.details.install_path.replace(/\\[^\\]+$/, '');
      }
    }
  }
});
async function selectIcon() { }
async function handleModifyClick() {
  nameError.value = !name.value;
  pathError.value = !parent_install_path.value;
  let final_install_path = '';
  try {
    const validatedPath = await exec('ValidatePath', { path: parent_install_path.value });
    final_install_path = `${validatedPath}\\${name.value}`;
    install_path.value = final_install_path;
  } catch (error) {
    pathError.value = true;
    globalThis.$errorHandler.showError(error);
  }
  if (nameError.value || pathError.value) return;
  try {
    const newApp = {
      id: id_modifying.value,
      timestamp_add: '',
      timestamp_update: '',
      installed: true,
      url: '',
      archive_password: '',
      details: {
        current_user_only: current_user_only.value,
        info: {
          name: name.value,
          icon: icon.value,
          publisher: publisher.value,
          version: version.value,
        },
        config: {
          custom_icon: custom_icon.value,
          create_desktop_shortcut: create_desktop_shortcut.value,
          create_start_menu_shortcut: create_start_menu_shortcut.value,
          create_registry_key: create_registry_key.value,
          add_to_path: [add_to_path.value[0], add_to_path.value[1]],
        },
        install_path: final_install_path,
        full_path: '',
      },
      validation_status: {
        file_exists: false,
        registry_valid: false,
        path_exists: false,
      },
    };
    await exec('ModifyApp', { new_app: newApp, id: id_modifying.value });
    generalStore.drawer.app_modify[1] = '';
    goTo('/Home');
  } catch (error) {
    globalThis.$errorHandler.showError(error);
  }
}
</script>

<template>
  <Drawer v-model:visible="generalStore.drawer.app_modify[0]" position="bottom" :style="{ height: '80vh' }"
    class="rounded-lg">
    <template #header>
      <div class="flex items-center gap-2">
        <span class="mir-apps text-xl"></span>
        <div>
          <h3 class="text-lg font-medium">{{ t('ui.install.app_details.title') }}</h3>
        </div>
      </div>
    </template>
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
              <label class="w-24 text-sm font-medium">{{ t('cls.install.config.install_path') }}</label>
              <div class="w-full">
                <InputText v-model="parent_install_path" :placeholder="t('ui.select_placeholder.dir')"
                  class="h-8 w-full text-sm" :invalid="pathError" />
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
                      <InputText v-model="add_to_path[1]" :placeholder="t('ui.select_placeholder.path_directory')"
                        class="h-8 w-full text-sm" />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Panel>
      </div>
    </div>
    <div class="flex items-center justify-end px-4 py-3">
      <Button severity="primary" class="h-8 w-28 text-sm transition-all duration-200" @click="handleModifyClick"
        icon="mir-install_desktop" :label="t('g.modify')" />
    </div>
  </Drawer>
</template>
