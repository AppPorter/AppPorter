<script setup lang="ts">
// Core imports
import { window } from "@/main";
import { goTo } from "@/plugins/router";
import { useSettingsStore } from "@/stores/settings";
import { invoke } from "@tauri-apps/api/core";
import { exit } from "@tauri-apps/plugin-process";
import { ref } from "vue";

// PrimeVue components and types
import ConfirmDialog from "primevue/confirmdialog";
import ContextMenu from "primevue/contextmenu";
import Menubar from "primevue/menubar";
import type { MenuItem } from "primevue/menuitem";
import Toast from "primevue/toast";
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";

// Store and utility initialization
const settingsStore = useSettingsStore();
const confirm = useConfirm();
const toast = useToast();

const dismiss_warning = ref(false);

// Window control handlers
function close_button() {
  if (settingsStore.minimize_to_tray_on_close) {
    window.hide();
  } else {
    exit(0);
  }
}

function minimize_button() {
  window.minimize();
}

// Warning
const solve = (event) => {
  confirm.require({
    target: event.currentTarget,
    message:
      "Do you want to restart the application with administrator privileges?",
    group: "popup",
    icon: "material-symbols-rounded warning",
    rejectProps: {
      label: "Cancel",
      severity: "secondary",
      outlined: true,
    },
    acceptProps: {
      label: "Yes",
    },
    accept: () => {
      if (!settingsStore.debug) {
        invoke("execute_command", {
          command: {
            name: "Elevate",
            revert: false,
          },
        });
        toast.add({
          severity: "success",
          summary: "Restart to apply the change",
          detail:
            "You have to restart the application to run the application with administrator privileges.",
          life: 3000,
        });
      }
    },
    reject: () => {},
  });
};
const button_items = [
  {
    label: "Dismiss Once",
    command: () => {
      dismiss_warning.value = true;
    },
  },
  //{
  //  label: "Dismiss Forever",
  //  command: () => {
  //    toast.add({
  //      severity: "info",
  //      summary: "You can enable this warning from settings",
  //      life: 3000,
  //    });
  //  },
  //},
];

// Navigation menu configuration
const menu_items = [
  {
    label: "Installation",
    icon: "material-symbols-rounded text-xl",
    iconClass: "install_desktop",
    command: () => goTo("/"),
  },
  {
    label: "AppList",
    icon: "material-symbols-rounded text-xl",
    iconClass: "apps",
    command: () => goTo("/AppList"),
  },
  {
    label: "Settings",
    icon: "material-symbols-rounded text-xl",
    iconClass: "settings",
    command: () => goTo("/Settings"),
  },
];

// Context menu configuration
const contextMenu = ref();
const editMenuItems = ref<MenuItem[]>([
  {
    label: "Cut",
    icon: "material-symbols-rounded text-base",
    iconClass: "content_cut",
    command: () => document.execCommand("cut"),
  },
  {
    label: "Copy",
    icon: "material-symbols-rounded text-base",
    iconClass: "content_copy",
    command: () => document.execCommand("copy"),
  },
  {
    label: "Paste",
    icon: "material-symbols-rounded text-base",
    iconClass: "content_paste",
    command: () => document.execCommand("paste"),
  },
  { separator: true },
  {
    label: "Select All",
    icon: "material-symbols-rounded text-base",
    iconClass: "select_all",
    command: () => document.execCommand("selectAll"),
  },
]);

// Context menu handler
function handleContextMenu(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (
    target instanceof HTMLInputElement ||
    target instanceof HTMLTextAreaElement
  ) {
    contextMenu.value?.show(event);
  }
  event.preventDefault();
}
</script>

<template>
  <div class="select-none w-screen h-screen" @contextmenu="handleContextMenu">
    <!-- System Dialogs -->
    <Toast position="bottom-left" class="z-40" />
    <ConfirmDialog group="dialog">
      <template #icon>
        <span class="material-symbols-rounded text-[32px]">warning</span>
      </template>
    </ConfirmDialog>

    <!-- Window Controls -->
    <div class="fixed top-0 right-0 h-auto z-50 flex">
      <button
        class="w-12 h-8 hover:bg-[#363636] hover:text-white flex items-center justify-center"
        @click="minimize_button"
      >
        <span class="material-symbols-rounded">remove</span>
      </button>
      <button
        class="w-12 h-8 hover:bg-[#c42b1c] hover:text-white flex items-center justify-center"
        @click="close_button"
      >
        <span class="material-symbols-rounded">close</span>
      </button>
    </div>

    <!-- Title Bar & Navigation -->
    <div class="fixed w-full z-30">
      <div
        class="flex items-center pr-24 w-full"
        style="-webkit-app-region: drag"
      >
        <span class="text-lg font-semibold pt-3 pl-6 pb-2">AppPorter</span>

        <!-- Warning -->
        <Message
          size="small"
          severity="warn"
          class="mx-4 py-0 w-full"
          v-if="!settingsStore.elevated && !dismiss_warning"
        >
          <template #icon
            ><span class="material-symbols-rounded">warning</span> </template
          >Application is not running with administrator privileges. Some
          features may be unavailable.
          <ConfirmPopup group="popup"></ConfirmPopup
          ><SplitButton
            @click="solve($event)"
            label="Solve"
            outlined
            severity="warn"
            class="z-40 left-2"
            size="small"
            :model="button_items"
          ></SplitButton
        ></Message>
      </div>

      <!-- Navigation Menu -->
      <div class="flex justify-center px-4">
        <Menubar :model="menu_items" class="border-none shadow-none w-full">
          <template #item="{ item }">
            <div class="flex items-center px-2">
              <span :class="[item.icon, 'flex items-center']">{{
                item.iconClass
              }}</span>
              <span class="ml-1">{{ item.label }}</span>
            </div>
          </template>
        </Menubar>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="pt-[100px] px-4 pb-6 h-full z-30 gap-2 flex">
      <RouterView />
    </div>

    <!-- Context Menu -->
    <ContextMenu ref="contextMenu" :model="editMenuItems">
      <template #item="{ item }">
        <div v-if="!item.separator" class="flex items-center">
          <span v-if="item.icon" :class="[item.icon, 'flex items-center mr-2']">
            {{ item.iconClass }}
          </span>
          <span>{{ item.label }}</span>
        </div>
      </template>
    </ContextMenu>

    <!-- Status Bar -->
    <div
      class="fixed bottom-0 left-0 right-0 h-6 px-4 flex items-center text-sm"
    >
      <p>{{ $route.fullPath }}</p>
    </div>
  </div>
</template>
