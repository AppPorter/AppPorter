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
import ZipPreview from "./components/ZipPreview.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

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
  app_icon,
  app_name,
  app_publisher,
  app_version,
  current_user_only,
  create_desktop_shortcut,
  create_registry_key,
  create_start_menu_shortcut,
  install_path,
} = storeToRefs(installationConfig);

function updateConfig(isCurrentUser: boolean) {
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

current_user_only.value = settings_current_user_only;
updateConfig(current_user_only.value);

function handleInstallModeChange(checked: boolean) {
  current_user_only.value = checked;
  updateConfig(checked);
}

async function select_install_path() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  // Only update if a directory was selected
  if (selected) {
    install_path.value = String(selected);
  }
}

async function start_installation() {
  try {
    const args = JSON.stringify({
      installationConfig,
    });
    console.log(args);
    const result = await invoke("execute_command", {
      command: "GetDetails",
      args,
    });
    console.log(result);
  } catch (error) {
    console.error("Failed to get details:", error);
  }
}

const detailsLoading = ref(false);
</script>

<template>
  <div class="h-[calc(100vh-144px)] p-4 pb-16 flex gap-4">
    <!-- Left Column -->
    <div class="flex-1 min-w-[400px] space-y-4">
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
              </Input>
              <Button variant="secondary" @click="select_install_path">
                <span v-svg="'folder_open'" class="w-4 h-4"></span>
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
        </CardContent>
      </Card>

      <!-- App Details Card -->
      <Card class="relative">
        <CardHeader class="pb-3">
          <div class="flex justify-between items-center">
            <CardTitle class="text-lg">App Details</CardTitle>
            <div
              class="w-12 h-12 rounded-md overflow-hidden shrink-0"
              :class="[
                !app_icon &&
                  'border-2 border-dashed border-muted-foreground/25',
              ]"
            >
              <img
                v-if="app_icon"
                :src="app_icon"
                class="w-full h-full object-contain"
                alt="App Icon"
              />
            </div>
          </div>
        </CardHeader>
        <CardContent class="space-y-4">
          <!-- App Name -->
          <div class="flex items-center gap-3">
            <Label class="w-24 text-sm">App Name</Label>
            <Input
              v-model="app_name"
              type="text"
              placeholder="Application Name"
              class="text-sm"
            />
          </div>

          <!-- Publisher -->
          <div class="flex items-center gap-3">
            <div class="w-24">
              <Label class="text-sm">Publisher</Label>
              <p class="text-xs text-muted-foreground">(Optional)</p>
            </div>
            <Input
              v-model="app_publisher"
              type="text"
              placeholder="Publisher Name"
              class="text-sm"
            />
          </div>

          <!-- Version -->
          <div class="flex items-center gap-3">
            <div class="w-24">
              <Label class="text-sm">Version</Label>
              <p class="text-xs text-muted-foreground">(Optional)</p>
            </div>
            <Input
              v-model="app_version"
              type="text"
              placeholder="1.0.0"
              class="text-sm"
            />
          </div>
        </CardContent>

        <!-- Loading Overlay -->
        <div
          v-if="detailsLoading"
          class="absolute inset-0 backdrop-blur-sm bg-background/50 flex flex-col items-center justify-center"
        >
          <h3 class="text-lg font-semibold">App Details</h3>
          <p class="text-sm text-muted-foreground">Loading...</p>
        </div>
      </Card>
    </div>

    <!-- Right Column -->
    <div class="min-w-[320px] w-[30%]">
      <ZipPreview
        :zip-path="zip_path"
        @loading="(value) => (detailsLoading = value)"
      />
    </div>
  </div>

  <!-- Install Button -->
  <div class="fixed bottom-12 right-14 z-40">
    <Button
      size="lg"
      class="w-32 flex items-center justify-center gap-2"
      @click="start_installation"
    >
      <span v-svg="'install'" class="w-4 h-4"></span>
      Install
    </Button>
  </div>
</template>

<style scoped>
.border-dashed {
  background: repeating-linear-gradient(
    45deg,
    transparent,
    transparent 5px,
    rgba(0, 0, 0, 0.03) 5px,
    rgba(0, 0, 0, 0.03) 10px
  );
}
.backdrop-blur-sm {
  backdrop-filter: blur(4px);
  transition: all 0.2s ease;
}
</style>
