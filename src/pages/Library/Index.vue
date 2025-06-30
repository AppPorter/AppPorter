<script setup lang="ts">
import { LibraryStore } from '@/stores/library'
import { exec } from '@/exec'
import DataView from 'primevue/dataview'
import Panel from 'primevue/panel'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import LibraryContextMenu from './components/LibraryContextMenu.vue'
import LibraryHeader from './components/LibraryHeader.vue'
import LibraryItem from './components/LibraryItem.vue'
import LibraryValidation from './components/LibraryValidation.vue'

const libraryStore = LibraryStore()
const { t } = useI18n()
const contextMenu = ref()
const validation = ref()
const selectedApp = ref()
const filters = ref({ global: { value: null, matchMode: 'contains' } })
const loading = ref(false)
const route = useRoute()

const links = computed(() => {
  const apps = libraryStore.apps.map(app => ({ ...app, type: 'app' }))
  const tools = libraryStore.tools.map(tool => ({
    ...tool,
    type: 'tool',
    details: {
      info: {
        name: tool.details.name,
        icon: '',
        publisher: '',
        version: ''
      },
      config: {
        create_desktop_shortcut: false,
        create_start_menu_shortcut: false,
        create_registry_key: false,
        add_to_path: tool.details.add_to_path,
      },
      install_path: tool.details.install_path,
      full_path: tool.details.install_path,
      validation_status: {
        file_exists: tool.validation_status.file_exists,
        registry_valid: false,
        path_exists: tool.validation_status.path_exists,
      }
    }
  }))
  const urls = libraryStore.urls.map(urlItem => ({
    ...urlItem,
    type: 'url',
    installed: false,
    details: {
      info: {
        name: urlItem.url,
        icon: '',
        publisher: '',
        version: ''
      },
      config: {
        create_desktop_shortcut: false,
        create_start_menu_shortcut: false,
        create_registry_key: false,
        add_to_path: [false, ''],
      },
      install_path: '',
      full_path: '',
      validation_status: {
        file_exists: false,
        registry_valid: false,
        path_exists: false,
      }
    }
  }))
  return [...apps, ...tools, ...urls]
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

async function loadLibrary() {
  loading.value = true
  await libraryStore.loadLibrary()
  loading.value = false
}

function showMenu(event) {
  selectedApp.value = event.data
  contextMenu.value.show(event.originalEvent)
}

async function installApp() {
  if (!selectedApp.value) return
  loading.value = true
  await exec('InstallWithLink', {
    url: selectedApp.value.url,
    timestamp: selectedApp.value.timestamp,
  })
  loading.value = false
}

function handleStatusClick(app) {
  validation.value?.handleStatusClick(app)
}

watch(
  () => route.path,
  (newPath) => {
    if (newPath === '/Library') {
      loadLibrary()
    }
  }
)

onMounted(() => {
  loadLibrary()
})
</script>

<template>
  <div class="flex size-full flex-col overflow-auto">
    <Panel class="mb-4 w-full shadow-sm">
      <template #header>
        <LibraryHeader v-model:sort-key="sortKey" v-model:sort-order="sortOrder"
          v-model:search-value="filters.global.value" />
      </template>

      <DataView :value="sortedApps" :loading="loading" :paginator="showPaginator" :rows="100"
        :rows-per-page-options="[50, 100, 200, 500]"
        filterBy="details.info.name,details.info.publisher,details.info.version" :filter-value="filters.global.value"
        dataKey="timestamp">
        <template #list="{ items }">
          <div class="grid">
            <LibraryItem v-for="item in items" :key="item.timestamp" :item="item" @context-menu="showMenu"
              @status-click="handleStatusClick" />
          </div>
        </template>

        <template #empty>
          <div class="flex flex-col items-center justify-center py-8">
            <span class="mir-apps text-4xl opacity-30"></span>
            <p class="mt-2 text-center">{{ t('ui.library.no_apps_found') }}</p>
            <p class="text-center text-sm opacity-70">{{ t('ui.library.install_first') }}</p>
          </div>
        </template>
      </DataView>
    </Panel>

    <LibraryContextMenu ref="contextMenu" :selected-app="selectedApp" @install-app="installApp"
      @load-library="loadLibrary" />

    <LibraryValidation ref="validation" @load-library="loadLibrary" />
  </div>
</template>
