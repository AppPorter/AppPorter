<script setup lang="ts">
import Button from 'primevue/button'
import IconField from 'primevue/iconfield'
import InputIcon from 'primevue/inputicon'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

interface Props {
    sortKey: string
    sortOrder: number
    searchValue: string
}

interface Emits {
    (e: 'update:sortKey', value: string): void
    (e: 'update:sortOrder', value: number): void
    (e: 'update:searchValue', value: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()
const { t } = useI18n()

const sortOptions = [
    { label: t('cls.sort.name'), value: 'name' },
    { label: t('cls.sort.publisher'), value: 'publisher' },
    { label: t('cls.sort.date'), value: 'timestamp' },
]

const currentSortKey = computed({
    get: () => props.sortKey,
    set: (value) => emit('update:sortKey', value)
})

const currentSearchValue = computed({
    get: () => props.searchValue,
    set: (value) => emit('update:searchValue', value)
})

function toggleSortOrder() {
    emit('update:sortOrder', props.sortOrder * -1)
}
</script>

<template>
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
                <Select v-model="currentSortKey" :options="sortOptions" class="w-40 text-sm" optionLabel="label"
                    optionValue="value" size="small" />
                <Button icon="mir-swap_vert" outlined severity="secondary" class="size-8 p-0 shadow-sm"
                    @click="toggleSortOrder" />
            </div>

            <div class="pl-4">
                <IconField>
                    <InputIcon>
                        <i class="mir-search" />
                    </InputIcon>
                    <InputText v-model="currentSearchValue" :placeholder="t('search')" class="h-8 text-sm" />
                </IconField>
            </div>
        </div>
    </div>
</template>
