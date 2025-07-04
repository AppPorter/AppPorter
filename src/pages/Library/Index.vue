<script setup lang="ts">
import { exec } from '@/exec'
import { libraryStore } from '@/main'
import Button from 'primevue/button'
import DataView from 'primevue/dataview'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import Panel from 'primevue/panel'
import Select from 'primevue/select'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import ContextMenu from './components/ContextMenu.vue'
import Item from './components/Item.vue'

const { t } = useI18n()
const contextMenu = ref()
const selectedApp = ref()
const filters = ref()
const loading = ref(false)
const route = useRoute()

const apps = computed(() => libraryStore.apps)
const tools = computed(() => libraryStore.tools)
const urls = computed(() => libraryStore.urls)

const sortKey = ref('name')
const sortOrder = ref(1)

const sortOptions = [
  { label: t('cls.sort.name'), value: 'name' },
  { label: t('cls.sort.publisher'), value: 'publisher' },
  { label: t('cls.sort.date'), value: 'timestamp' },
]

const currentSortKey = computed({
  get: () => sortKey.value,
  set: (value: string) => sortKey.value = value
})

const currentSearchValue = computed({
  get: () => filters.value,
  set: (value: string) => filters.value = value
})

function toggleSortOrder() {
  sortOrder.value = sortOrder.value * -1
}

const sortedApps = computed(() => {
  const allApps = [...apps.value]
  return allApps.sort((a, b) => {
    let valueA, valueB
    if (sortKey.value === 'name') {
      valueA = a.details.info.name.toLowerCase()
      valueB = b.details.info.name.toLowerCase()
    } else if (sortKey.value === 'publisher') {
      valueA = a.details.info.publisher.toLowerCase()
      valueB = b.details.info.publisher.toLowerCase()
    } else {
      valueA = a.timestamp_add
      valueB = b.timestamp_add
    }
    return sortOrder.value * (valueA < valueB ? -1 : valueA > valueB ? 1 : 0)
  })
})

const sortedTools = computed(() => {
  const allTools = [...tools.value]
  return allTools.sort((a, b) => {
    let valueA, valueB
    if (sortKey.value === 'name') {
      valueA = a.details.name.toLowerCase()
      valueB = b.details.name.toLowerCase()
    } else {
      valueA = a.timestamp_add
      valueB = b.timestamp_add
    }
    return sortOrder.value * (valueA < valueB ? -1 : valueA > valueB ? 1 : 0)
  })
})

const sortedUrls = computed(() => {
  const allUrls = [...urls.value]
  return allUrls.sort((a, b) => {
    let valueA, valueB
    if (sortKey.value === 'name') {
      valueA = a.url.toLowerCase()
      valueB = b.url.toLowerCase()
    } else {
      valueA = a.timestamp
      valueB = b.timestamp
    }
    return sortOrder.value * (valueA < valueB ? -1 : valueA > valueB ? 1 : 0)
  })
})

const allItems = computed(() => {
  return [
    ...sortedApps.value.map(item => ({ ...item, type: 'app' })),
    ...sortedTools.value.map(item => ({ ...item, type: 'tool' })),
    ...sortedUrls.value.map(item => ({ ...item, type: 'url' })),
  ]
})

const showPaginator = computed(() => {
  return apps.value.length > 100
})

async function loadLibrary() {
  loading.value = true
  await libraryStore.loadLibrary()
  loading.value = false
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

async function checkUpdates() {
  loading.value = true
  await exec('CheckForUpdates')
  await libraryStore.loadLibrary()
  loading.value = false
}

const showContextMenu = (event: { originalEvent: Event; data: unknown }) => {
  selectedApp.value = event.data
  contextMenu.value?.show(event.originalEvent)
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
        <div class="flex w-full flex-wrap items-center justify-between gap-4">
          <div class="flex items-center gap-2">
            <span class="mir-apps text-xl"></span>
            <div class="min-w-32">
              <h2 class="text-lg font-medium">{{ t('ui.library.all_apps') }}</h2>
              <p class="mt-0.5 text-xs">{{ t('ui.library.description') }}</p>
            </div>
          </div>

          <div class="flex flex-wrap items-center divide-x divide-surface-200 dark:divide-surface-700">
            <div class="flex items-center gap-2 px-4">
              <Button icon="mir-update" outlined severity="secondary" class="h-8 shadow-sm" :loading="loading"
                @click="checkUpdates" :label="t('g.check_updates')" />
              <Select v-model="currentSortKey" :options="sortOptions" class="max-w-40 text-sm" optionLabel="label"
                optionValue="value" size="small" />
              <Button icon="mir-swap_vert" outlined severity="secondary" class="size-8 p-0 shadow-sm"
                @click="toggleSortOrder" />
            </div>

            <div class="pl-4">
              <IconField>
                <InputIcon>
                  <i class="mir-search" />
                </InputIcon>
                <InputText v-model="currentSearchValue" :placeholder="t('g.search')" class="h-8 text-sm" />
              </IconField>
            </div>
          </div>
        </div>
      </template>

      <DataView :value="allItems" :loading="loading" :paginator="showPaginator" :rows="100"
        :rows-per-page-options="[50, 100, 200, 500]"
        filterBy="details.info.name,details.info.publisher,details.info.version" :filter-value="filters"
        dataKey="timestamp">
        <template #list="{ items }">
          <div class="grid">
            <Item v-for="item in items" :key="String(item.timestamp) + (item._type || '')" :item="item"
              @contextMenu="showContextMenu" />
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

    <ContextMenu ref="contextMenu" :selected-app="selectedApp" @install-app="installApp" @load-library="loadLibrary" />
  </div>
</template>
