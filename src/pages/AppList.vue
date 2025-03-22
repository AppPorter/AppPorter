<script setup lang="ts">
import { useAppListStore } from '@/stores/app_list'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import Column from 'primevue/column'
import ConfirmDialog from 'primevue/confirmdialog'
import type { DataTableRowContextMenuEvent } from 'primevue/datatable'
import DataTable from 'primevue/datatable'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import Menu from 'primevue/menu'
import Panel from 'primevue/panel'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const appListStore = useAppListStore()
const { t } = useI18n()
const toast = useToast()
const confirm = useConfirm()
const contextMenu = ref()
const selectedApp = ref()
const filters = ref({ global: { value: null, matchMode: 'contains' } })
const loading = ref(false)
const route = useRoute()

const installedApps = computed(() => {
  return appListStore.links.filter((app) => app.installed)
})

const showPaginator = computed(() => {
  return installedApps.value.length > 100
})

async function loadAppList() {
  loading.value = true
  await appListStore.loadAppList()
  loading.value = false
}

function getAppStatus(data) {
  if (!data.installed) {
    return {
      icon: 'mir-cloud_download',
      severity: 'secondary',
      value: t('app_list.not_installed'),
    }
  }

  const validation = data.details.validation_status
  const isValid = validation.file_exists && validation.registry_valid

  if (isValid) {
    return {
      icon: 'mir-check',
      value: t('app_list.installed'),
    }
  }

  return {
    icon: 'mir-error',
    severity: 'warn',
    value: t('app_list.validation_error'),
  }
}

function showMenu(event: DataTableRowContextMenuEvent) {
  selectedApp.value = event.data
  contextMenu.value.show(event.originalEvent)
}

const menuItems = computed(() => [
  {
    label: t('app_list.open'),
    icon: 'mir-terminal',
    command: () => openApp(),
  },
  {
    label: t('app_list.open_install_folder'),
    icon: 'mir-folder',
    command: () => openInstallFolder(),
  },
  {
    label: t('app_list.open_registry'),
    icon: 'mir-app_registration',
    command: () => openRegistry(),
  },
  {
    label: t('app_list.uninstall'),
    icon: 'mir-delete',
    command: () => confirmUninstall(),
  },
  {
    label: t('app_list.remove'),
    icon: 'mir-close',
    command: () => removeApp(),
  },
])

async function openApp() {
  if (!selectedApp.value) return

  await invoke('execute_command', {
    command: {
      name: 'Open',
      path: selectedApp.value.details.full_path,
    },
  })
  loading.value = false
}

async function openInstallFolder() {
  if (!selectedApp.value) return

  await invoke('execute_command', {
    command: {
      name: 'OpenFolder',
      path: selectedApp.value.details.full_path,
    },
  })
}

async function openRegistry() {
  if (!selectedApp.value) return

  await invoke('execute_command', {
    command: {
      name: 'OpenRegistry',
      app_name: selectedApp.value.details.name,
      current_user_only: selectedApp.value.details.current_user_only,
    },
  })
}

function confirmUninstall() {
  if (!selectedApp.value) return

  confirm.require({
    message: t('app_list.confirm_uninstall_message', { name: selectedApp.value.details.name }),
    header: t('app_list.confirm_uninstall_header'),
    icon: 'mir-warning',
    acceptLabel: t('app_list.uninstall'),
    rejectLabel: t('app_list.cancel'),
    accept: uninstallApp,
  })
}

function uninstallApp() {
  toast.add({
    severity: 'success',
    summary: t('app_list.uninstall_success'),
    detail: selectedApp.value.details.name,
    life: 3000,
  })
}

function removeApp() {
  if (!selectedApp.value) return

  confirm.require({
    message: t('app_list.confirm_remove_message', { name: selectedApp.value.details.name }),
    header: t('app_list.confirm_remove_header'),
    group: 'dialog',
    icon: 'mir-warning',
    rejectProps: {
      label: t('app_list.cancel'),
      severity: 'secondary',
      outlined: true,
      icon: 'mir-close',
    },
    acceptProps: {
      label: t('app_list.remove'),
      severity: 'danger',
      icon: 'mir-warning',
    },
    accept: () => {
      appListStore.removeApp(selectedApp.value.url)
      toast.add({
        severity: 'info',
        summary: t('app_list.remove_success'),
        detail: selectedApp.value.details.name,
        life: 3000,
      })
    },
  })
}

function formatTimestamp(timestamp) {
  return new Date(timestamp * 1000).toLocaleDateString()
}

watch(
  () => route.path,
  (newPath) => {
    if (newPath === '/AppList') {
      loadAppList()
    }
  }
)

onMounted(() => {
  loadAppList()
})
</script>

<template>
  <div class="flex size-full flex-col overflow-auto">
    <Panel class="mb-4 size-full">
      <template #header>
        <div class="flex w-full items-center justify-between">
          <div class="flex items-center gap-2">
            <span class="mir-apps text-xl"></span>
            <div>
              <h2 class="text-lg font-medium">{{ t('app_list.installed_apps') }}</h2>
              <p class="mt-0.5 text-xs">{{ t('app_list.description') }}</p>
            </div>
          </div>

          <IconField>
            <InputIcon>
              <i class="mir-search" />
            </InputIcon>
            <InputText
              v-model="filters.global.value"
              :placeholder="t('app_list.search')"
              class="h-8 text-sm"
            />
          </IconField>
        </div>
      </template>

      <DataTable
        :value="installedApps"
        :loading="loading"
        v-model:filters="filters"
        filterDisplay="menu"
        :globalFilterFields="['details.name', 'details.publisher', 'details.version']"
        stripedRows
        :paginator="showPaginator"
        :rows="100"
        :rowsPerPageOptions="[50, 100, 200, 500]"
        tableStyle="min-width: 50rem"
        @row-contextmenu="showMenu"
        responsiveLayout="scroll"
      >
        <Column header="Status">
          <template #body="slotProps">
            <Tag
              :value="getAppStatus(slotProps.data).value"
              :severity="getAppStatus(slotProps.data).severity"
              :icon="getAppStatus(slotProps.data).icon"
            />
          </template>
        </Column>
        <Column field="details.icon" header="" style="width: 56px">
          <template #body="slotProps">
            <div class="flex items-center justify-center">
              <div
                class="flex size-10 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800"
              >
                <img
                  v-if="slotProps.data.details.icon"
                  :src="slotProps.data.details.icon"
                  class="size-8 object-contain"
                  alt="App Icon"
                />
                <span v-else class="mir-apps text-2xl"></span>
              </div>
            </div>
          </template>
        </Column>

        <Column field="details.name" :header="t('app_list.name')" sortable>
          <template #body="slotProps">
            <div class="flex flex-col">
              <span class="text-sm font-medium">{{ slotProps.data.details.name }}</span>
              <span class="text-xs text-slate-500 dark:text-slate-400">
                {{ slotProps.data.details.version || 'N/A' }}
              </span>
            </div>
          </template>
        </Column>

        <Column field="details.publisher" :header="t('app_list.publisher')" sortable>
          <template #body="slotProps">
            <div class="break-all text-sm">
              {{ slotProps.data.details.publisher }}
            </div>
          </template>
        </Column>

        <Column field="details.install_path" :header="t('app_list.location')" sortable>
          <template #body="slotProps">
            <div class="break-all text-sm">
              {{ slotProps.data.details.install_path }}
            </div>
          </template>
        </Column>

        <Column field="timestamp" :header="t('app_list.installed_date')" sortable>
          <template #body="slotProps">
            {{ formatTimestamp(slotProps.data.timestamp) }}
          </template>
        </Column>

        <Column :exportable="false" style="width: 4rem">
          <template #body="slotProps">
            <Button
              icon="mir-more_vert"
              outlined
              severity="secondary"
              class="size-8 p-0"
              @click="
                ($event) =>
                  showMenu({
                    originalEvent: $event,
                    data: slotProps.data,
                    index: installedApps.indexOf(slotProps.data),
                  })
              "
            />
          </template>
        </Column>

        <template #empty>
          <div class="flex flex-col items-center justify-center py-8">
            <span class="mir-apps text-4xl opacity-30"></span>
            <p class="mt-2 text-center">{{ t('app_list.no_apps_found') }}</p>
            <p class="text-center text-sm opacity-70">{{ t('app_list.install_first') }}</p>
          </div>
        </template>
      </DataTable>
    </Panel>

    <!-- Context Menu -->
    <Menu ref="contextMenu" :model="menuItems" :popup="true" />

    <!-- Confirmation Dialog -->
    <ConfirmDialog />
  </div>
</template>
