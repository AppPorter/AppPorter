<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { useSettingsStore } from "@/stores/settings";
import { open } from "@tauri-apps/plugin-dialog";
import { storeToRefs } from "pinia";
import { nextTick, onMounted } from "vue";
import ZipPreview from "./components/ZipPreview.vue";

const settingsStore = useSettingsStore();
const {
  installation: {
    current_user_only: settings_current_user_only,
    all_users: {
      create_desktop_shortcut: all_create_desktop_shortcut,
      create_registry_key: all_create_registry_key,
      create_start_menu_shortcut: all_create_start_menu_shortcut,
      install_path: all_install_path,
    },
    current_user: {
      create_desktop_shortcut: current_create_desktop_shortcut,
      create_registry_key: current_create_registry_key,
      create_start_menu_shortcut: current_create_start_menu_shortcut,
      install_path: current_install_path,
    },
  },
} = settingsStore;

const installationConfig = useInstallationConfigStore();
const { zip_path } = installationConfig;
const {
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
} = storeToRefs(installationConfig);

current_user_only.value = settings_current_user_only;

function updateConfigByMode(isCurrentUser: boolean) {
  if (isCurrentUser) {
    create_desktop_shortcut.value = current_create_desktop_shortcut;
    create_registry_key.value = current_create_registry_key;
    create_start_menu_shortcut.value = current_create_start_menu_shortcut;
    install_path.value = current_install_path;
  } else {
    create_desktop_shortcut.value = all_create_desktop_shortcut;
    create_registry_key.value = all_create_registry_key;
    create_start_menu_shortcut.value = all_create_start_menu_shortcut;
    install_path.value = all_install_path;
  }
}

onMounted(async () => {
  await nextTick();
  updateConfigByMode(current_user_only.value);
});

function handleInstallModeChange(checked: boolean) {
  current_user_only.value = checked;
  updateConfigByMode(checked);
}

async function select_install_path() {
  const path = await open({
    directory: true,
    multiple: false,
  });
  install_path.value = String(path);
}

function start_installation() {}
</script>

<template>
  <div class="p-4 space-y-4">
    <!-- Add ZipPreview component -->
    <ZipPreview :zip-path="zip_path" />

    <Card>
      <CardHeader class="pb-3">
        <CardTitle class="text-lg">Installation Options</CardTitle>
        <p class="text-xs">
          Selected file: <b>{{ zip_path }}</b>
        </p>
      </CardHeader>
      <CardContent class="space-y-4">
        <!-- Install Mode -->
        <div class="flex items-center gap-3">
          <Label class="w-24 text-sm">Install Mode</Label>
          <div class="flex items-center gap-2">
            <Label for="install_mode">All Users</Label>
            <Switch
              id="install_mode"
              :checked="current_user_only"
              @update:checked="handleInstallModeChange"
            />
            <Label for="install_mode">Current User</Label>
          </div>
        </div>

        <!-- Install Path -->
        <div class="flex items-center gap-3">
          <Label class="w-24 text-sm">Install Path</Label>
          <div class="flex-1 flex gap-2">
            <Input
              v-model="install_path"
              type="text"
              placeholder="Choose installation directory"
              class="text-sm"
            >
              <span v-svg="'folder'" class="w-4 h-4 opacity-70"></span>
            </Input>
            <Button variant="secondary" @click="select_install_path">
              <span v-svg="'open_folder'" class="w-4 h-4"></span>
              Browse
            </Button>
          </div>
        </div>

        <!-- Shortcuts -->
        <div class="flex items-start gap-3">
          <Label class="w-24 mt-1 text-sm">Shortcuts</Label>
          <div class="space-y-1.5">
            <div class="flex items-center space-x-2">
              <Checkbox
                id="create_desktop_shortcut"
                :checked="create_desktop_shortcut"
                @update:checked="
                  (checked) => (create_desktop_shortcut = checked)
                "
              />
              <Label for="create_desktop_shortcut">
                Create Desktop Shortcut
              </Label>
            </div>
            <div class="flex items-center space-x-2">
              <Checkbox
                id="create_start_menu_shortcut"
                :checked="create_start_menu_shortcut"
                @update:checked="
                  (checked) => (create_start_menu_shortcut = checked)
                "
              />
              <Label for="create_start_menu_shortcut">
                Create Start Menu Shortcut
              </Label>
            </div>
            <div class="flex items-center space-x-2">
              <Checkbox
                id="create_registry_key"
                :checked="create_registry_key"
                @update:checked="(checked) => (create_registry_key = checked)"
              />
              <Label for="create_registry_key"> Create Registry Entry </Label>
            </div>
          </div>
        </div>

        <!-- Install Button -->
        <div class="flex justify-end pt-2">
          <Button
            class="w-28 flex items-center justify-center gap-2"
            @click="start_installation"
          >
            <span v-svg="'install'" class="w-4 h-4"></span>
            Install
          </Button>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
