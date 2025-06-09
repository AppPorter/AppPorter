<script setup lang="ts">
import { AppListStore } from '@/stores/app_list'
import { invoke } from '@tauri-apps/api/core'
import DataView from 'primevue/dataview'
import Panel from 'primevue/panel'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import AppListContextMenu from './components/AppListContextMenu.vue'
import AppListHeader from './components/AppListHeader.vue'
import AppListItem from './components/AppListItem.vue'
import AppListValidation from './components/AppListValidation.vue'

const appListStore = AppListStore()
const { t } = useI18n()
const contextMenu = ref()
const validation = ref()
const selectedApp = ref()
const filters = ref({ global: { value: null, matchMode: 'contains' } })
const loading = ref(false)
const route = useRoute()

// Combine apps and libs into a unified list
const links = computed(() => {
  const apps = appListStore.apps.map(app => ({ ...app, type: 'app' }))
  const libs = appListStore.libs.map(lib => ({
    ...lib,
    type: 'lib',
    details: {
      info: {
        name: lib.details.name,
        icon: '',
        publisher: '',
        version: ''
      },
      config: {
        ...lib.details.config,
        current_user_only: false,
        create_desktop_shortcut: false,
        create_start_menu_shortcut: false,
        create_registry_key: false,
        archive_exe_path: '',
      },
      paths: {
        parent_install_path: lib.details.paths.parent_install_path,
        install_path: lib.details.paths.install_path,
        full_path: lib.details.paths.install_path,
      },
      validation_status: {
        file_exists: lib.details.validation_status.file_exists,
        registry_valid: false,
        path_exists: lib.details.validation_status.path_exists,
      }
    }
  }))
  return [...apps, ...libs]
})

const sortKey = ref('name')
const sortOrder = ref(1)

const sortedApps = computed(() => {
  const allLinks = [...links.value]
  return allLinks.sort((a, b) => {
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
  return links.value.length > 100
})

async function loadAppList() {
  loading.value = true
  await appListStore.loadAppList()
  loading.value = false
}

function showMenu(event) {
  selectedApp.value = event.data
  contextMenu.value.show(event.originalEvent)
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

function handleStatusClick(app) {
  validation.value?.handleStatusClick(app)
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
        <AppListHeader v-model:sort-key="sortKey" v-model:sort-order="sortOrder"
          v-model:search-value="filters.global.value" />
      </template>

      <DataView :value="sortedApps" :loading="loading" :paginator="showPaginator" :rows="100"
        :rows-per-page-options="[50, 100, 200, 500]"
        filterBy="details.info.name,details.info.publisher,details.info.version" :filter-value="filters.global.value"
        dataKey="timestamp">
        <template #list="{ items }">
          <div class="grid">
            <AppListItem v-for="item in items" :key="item.timestamp" :item="item" @context-menu="showMenu"
              @status-click="handleStatusClick" />
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
    <AppListContextMenu ref="contextMenu" :selected-app="selectedApp" @install-app="installApp"
      @load-app-list="loadAppList" />

    <!-- Validation Handler -->
    <AppListValidation ref="validation" @load-app-list="loadAppList" />
  </div>
</template>
