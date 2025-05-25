<script setup lang="ts">
import { useAppListStore } from '@/stores/app_list'
import { invoke } from '@tauri-apps/api/core'
import Button from 'primevue/button'
import DataView from 'primevue/dataview'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import Menu from 'primevue/menu'
import Panel from 'primevue/panel'
import Tag from 'primevue/tag'
import { useConfirm } from 'primevue/useconfirm'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const appListStore = useAppListStore()
const { t } = useI18n()
const confirm = useConfirm()
const contextMenu = ref()
const selectedApp = ref()
const filters = ref({ global: { value: null, matchMode: 'contains' } })
const loading = ref(false)
const route = useRoute()

const sortKey = ref('name')
const sortOrder = ref(1)
const sortOptions = [
  { label: t('sort.name'), value: 'name' },
  { label: t('sort.publisher'), value: 'publisher' },
  { label: t('sort.date'), value: 'timestamp' },
]

const sortedApps = computed(() => {
  const apps = [...appListStore.links]
  return apps.sort((a, b) => {
    let valueA, valueB
    if (sortKey.value === 'name') {
      valueA = a.details.info.name.toLowerCase()
      valueB = b.details.info.name.toLowerCase()
    } else if (sortKey.value === 'publisher') {
      valueA = a.details.info.publisher.toLowerCase()
      valueB = b.details.info.publisher.toLowerCase()
    } else {
      valueA = a.timestamp
      valueB = b.timestamp
    }
    return sortOrder.value * (valueA < valueB ? -1 : valueA > valueB ? 1 : 0)
  })
})

const showPaginator = computed(() => {
  return appListStore.links.length > 100
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

  // Handle copy_only mode
  if (data.copy_only) {
    return {
      icon: 'mir-folder_copy',
      severity: 'info',
      value: t('app_list.copy_only'),
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

function showMenu(event) {
  selectedApp.value = event.data
  contextMenu.value.show(event.originalEvent)
}

const menuItems = computed(() => [
  {
    label: t('install'),
    icon: 'mir-install_desktop',
    command: () => installApp(),
    visible: () => selectedApp.value && !selectedApp.value.installed,
  },
  {
    label: t('open'),
    icon: 'mir-terminal',
    command: () => openApp(),
    visible: () => selectedApp.value?.installed && !selectedApp.value?.copy_only,
  },
  {
    label: t('open_folder'),
    icon: 'mir-folder',
    command: () => openInstallFolder(),
    visible: () => selectedApp.value?.installed && selectedApp.value?.copy_only,
  },
  {
    label: t('open_install_folder'),
    icon: 'mir-folder',
    command: () => openInstallFolder(),
    visible: () => selectedApp.value?.installed && !selectedApp.value?.copy_only,
  },
  {
    label: t('open_registry'),
    icon: 'mir-app_registration',
    command: () => openRegistry(),
    visible: () => selectedApp.value?.installed && !selectedApp.value?.copy_only,
  },
  {
    label: selectedApp.value?.copy_only ? t('delete') : (selectedApp.value?.installed ? t('uninstall') : t('remove')),
    icon: 'mir-delete',
    command: () => (selectedApp.value?.installed ? (selectedApp.value?.copy_only ? confirmDelete() : confirmUninstall()) : confirmRemove()),
    visible: () => selectedApp.value !== undefined,
  },
])

async function openApp() {
  if (!selectedApp.value) return

  await invoke('execute_command', {
    command: {
      name: 'Open',
      path: selectedApp.value.details.paths.full_path,
    },
  })
  loading.value = false
}

async function openInstallFolder() {
  if (!selectedApp.value) return

  await invoke('execute_command', {
    command: {
      name: 'OpenFolder',
      path: selectedApp.value.details.paths.full_path,
    },
  })
}

async function openRegistry() {
  if (!selectedApp.value) return

  await invoke('execute_command', {
    command: {
      name: 'OpenRegistry',
      app_name: selectedApp.value.details.info.name,
      current_user_only: selectedApp.value.details.config.current_user_only,
    },
  })
}

async function confirmUninstall() {
  if (!selectedApp.value) return

  const app = appListStore.getAppByTimestamp(selectedApp.value.timestamp)
  if (!app) return

  await new Promise((resolve, reject) => {
    confirm.require({
      message: t('app_list.confirm_uninstall_message', {
        name: app.details.info.name,
      }),
      group: 'dialog',
      header: t('app_list.confirm_uninstall_header'),
      icon: 'mir-warning',
      rejectProps: {
        label: t('cancel'),
        severity: 'secondary',
        outlined: true,
        icon: 'mir-close',
      },
      acceptProps: {
        label: t('uninstall'),
        severity: 'danger',
        icon: 'mir-warning',
      },
      accept: async () => {
        await appListStore.executeUninstall(selectedApp.value.timestamp)
        resolve(true)
      },
      reject: () => reject(),
    })
  })
}

async function confirmDelete() {
  if (!selectedApp.value) return

  const app = appListStore.getAppByTimestamp(selectedApp.value.timestamp)
  if (!app) return

  await new Promise((resolve, reject) => {
    confirm.require({
      message: t('app_list.confirm_delete_message', {
        name: app.details.info.name,
      }),
      group: 'dialog',
      header: t('app_list.confirm_delete_header'),
      icon: 'mir-warning',
      rejectProps: {
        label: t('cancel'),
        severity: 'secondary',
        outlined: true,
        icon: 'mir-close',
      },
      acceptProps: {
        label: t('delete'),
        severity: 'danger',
        icon: 'mir-delete',
      },
      accept: async () => {
        // Since we don't have a specific method for copy_only apps yet,
        // we'll use the general removeApp method
        await appListStore.removeApp(selectedApp.value.timestamp)
        resolve(true)
      },
      reject: () => reject(),
    })
  })
}

async function confirmRemove() {
  if (!selectedApp.value) return

  await new Promise((resolve, reject) => {
    confirm.require({
      message: t('app_list.confirm_remove_message', {
        name: selectedApp.value.details.info.name,
      }),
      group: 'dialog',
      header: t('app_list.confirm_remove_header'),
      icon: 'mir-warning',
      rejectProps: {
        label: t('cancel'),
        severity: 'secondary',
        outlined: true,
        icon: 'mir-close',
      },
      acceptProps: {
        label: t('remove'),
        severity: 'danger',
        icon: 'mir-delete',
      },
      accept: async () => {
        await appListStore.removeApp(selectedApp.value.timestamp)
        resolve(true)
      },
      reject: () => reject(),
    })
  })
}

async function installApp() {
  if (!selectedApp.value) return
  loading.value = true
  await invoke('execute_command', {
    command: {
      name: 'InstallWithLink',
      url: selectedApp.value.url,
      timestamp: selectedApp.value.timestamp,
    },
  })
  loading.value = false
}

const appToValidate = ref()

function handleStatusClick(app) {
  if (app.installed) {
    appToValidate.value = app

    // Skip validation for copy_only apps as they don't need validation
    if (app.copy_only) {
      return;
    }

    const validation = app.details.validation_status
    const fileExists = validation.file_exists
    const registryValid = validation.registry_valid

    if (!fileExists && !registryValid) {
      confirm.require({
        message: t('validation.issue', { name: app.details.info.name }) + t('validation.missing_both'),
        header: t('validation.title'),
        icon: 'mir-warning',
        rejectProps: {
          label: t('uninstall'),
          icon: 'mir-delete',
          severity: 'danger',
          variant: 'outlined',
        },
        acceptProps: {
          label: t('reinstall'),
          icon: 'mir-refresh',
        },
        accept: () => handleValidationAction('reinstall'),
        reject: () => handleValidationAction('uninstall'),
      })
    } else if (!fileExists) {
      confirm.require({
        message: t('validation.issue', { name: app.details.info.name }) + t('validation.missing_file'),
        header: t('validation.title'),
        icon: 'mir-warning',
        rejectProps: {
          label: t('uninstall'),
          icon: 'mir-delete',
          severity: 'danger',
          variant: 'outlined',
        },
        acceptProps: {
          label: t('reinstall'),
          icon: 'mir-refresh',
        },
        accept: () => handleValidationAction('reinstall'),
        reject: () => handleValidationAction('uninstall'),
      })
    } else if (!registryValid) {
      confirm.require({
        message:
          t('validation.issue', { name: app.details.info.name }) + t('validation.missing_registry'),
        header: t('validation.title'),
        icon: 'mir-warning',
        rejectProps: {
          label: t('uninstall'),
          icon: 'mir-delete',
          severity: 'danger',
          variant: 'outlined',
        },
        acceptProps: {
          label: t('reinstall'),
          icon: 'mir-build',
        },
        accept: () => handleValidationAction('repair'),
        reject: () => handleValidationAction('uninstall'),
      })
    }
  }
}

async function handleValidationAction(action: 'reinstall' | 'repair' | 'uninstall') {
  if (!appToValidate.value) return

  if (action === 'uninstall') {
    await new Promise((resolve, reject) => {
      confirm.require({
        message: t('confirm_uninstall_message', {
          name: appToValidate.value.details.info.name,
        }),
        group: 'dialog',
        header: t('confirm_uninstall_header'),
        icon: 'mir-warning',
        rejectProps: {
          label: t('cancel'),
          severity: 'secondary',
          outlined: true,
          icon: 'mir-close',
        },
        acceptProps: {
          label: t('uninstall'),
          severity: 'danger',
          icon: 'mir-warning',
        },
        accept: async () => {
          await appListStore.executeUninstall(appToValidate.value.timestamp)
          resolve(true)
        },
        reject: () => reject(),
      })
    })
  } else {
    await invoke('execute_command', {
      command: {
        name: action === 'reinstall' ? 'Reinstall' : 'Repair',
        timestamp: appToValidate.value.timestamp,
      },
    })
    await loadAppList()
  }
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
    <Panel class="mb-4 size-full shadow-sm">
      <template #header>
        <div class="flex w-full flex-wrap items-center justify-between gap-4">
          <div class="flex items-center gap-2">
            <span class="mir-apps text-xl"></span>
            <div class="min-w-32">
              <h2 class="text-lg font-medium">{{ t('app_list.all_apps') }}</h2>
              <p class="mt-0.5 text-xs">{{ t('app_list.description') }}</p>
            </div>
          </div>

          <div class="flex flex-wrap items-center divide-x divide-surface-200 dark:divide-surface-700">
            <div class="flex items-center gap-2 px-4">
              <Select v-model="sortKey" :options="sortOptions" class="w-40 text-sm" optionLabel="label"
                optionValue="value" size="small" />
              <Button icon="mir-swap_vert" outlined severity="secondary" class="size-8 p-0 shadow-sm"
                @click="sortOrder *= -1" />
            </div>

            <div class="pl-4">
              <IconField>
                <InputIcon>
                  <i class="mir-search" />
                </InputIcon>
                <InputText v-model="filters.global.value" :placeholder="t('search')" class="h-8 text-sm" />
              </IconField>
            </div>
          </div>
        </div>
      </template>

      <DataView :value="sortedApps" :loading="loading" :paginator="showPaginator" :rows="100"
        :rows-per-page-options="[50, 100, 200, 500]"
        filterBy="details.info.name,details.info.publisher,details.info.version" :filter-value="filters.global.value"
        dataKey="timestamp">
        <template #list="{ items }">
          <div class="grid">
            <div v-for="item in items" :key="item.timestamp"
              class="w-full border-b border-surface-200 dark:border-surface-700">
              <div class="flex items-center p-4" @contextmenu.prevent="showMenu({ originalEvent: $event, data: item })">
                <div class="mr-4 flex items-center justify-center">
                  <div
                    class="flex size-10 items-center justify-center overflow-hidden rounded-lg bg-surface-50 dark:bg-surface-800">
                    <img v-if="item.details.info.icon" :src="item.details.info.icon" class="size-8 object-contain"
                      alt="App Icon" />
                    <span v-else class="mir-apps text-2xl"></span>
                  </div>
                </div>

                <div class="flex-1">
                  <div class="mb-2 flex flex-col">
                    <span class="text-sm font-medium">{{ item.details.info.name || item.url }}</span>
                    <div v-if="!item.copy_only"
                      class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400">
                      <span>{{ item.details.info.version || t('app.unknown_version') }}</span>
                      <span class="opacity-50">•</span>
                      <span>{{ item.details.info.publisher || t('app.unknown_publisher') }}</span>
                    </div>
                  </div>

                  <div class="flex items-center gap-2 text-xs">
                    <template v-if="item.details.paths.install_path">
                      <span class="break-all opacity-75">{{ item.details.paths.install_path }}</span>
                      <span class="opacity-50">•</span>
                    </template>
                    <span class="opacity-75">{{ formatTimestamp(item.timestamp) }}</span>
                  </div>
                </div>

                <Tag :value="getAppStatus(item).value" :severity="getAppStatus(item).severity"
                  :icon="getAppStatus(item).icon" class="mr-2 cursor-pointer text-center text-xs"
                  @click="handleStatusClick(item)" />

                <Button icon="mir-more_vert" outlined severity="secondary" class="size-8 p-0 shadow-sm"
                  @click="showMenu({ originalEvent: $event, data: item })" />
              </div>
            </div>
          </div>
        </template>

        <template #empty>
          <div class="flex flex-col items-center justify-center py-8">
            <span class="mir-apps text-4xl opacity-30"></span>
            <p class="mt-2 text-center">{{ t('app_list.no_apps_found') }}</p>
            <p class="text-center text-sm opacity-70">{{ t('app_list.install_first') }}</p>
          </div>
        </template>
      </DataView>
    </Panel>

    <!-- Context Menu -->
    <Menu ref="contextMenu" :model="menuItems" :popup="true" />
  </div>
</template>
