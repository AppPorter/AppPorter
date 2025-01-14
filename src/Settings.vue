<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();
const goBack = () => {
  router.back();
};
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #title>
      <div class="flex items-center gap-2.5">
        <div class="p-1.5 rounded-md bg-primary-50 dark:bg-primary-900/20">
          <span
            class="material-symbols-rounded text-xl text-primary-600 dark:text-primary-400"
          >
            settings
          </span>
        </div>
        <div>
          <h2
            class="text-lg font-medium leading-tight text-surface-900 dark:text-surface-0"
          >
            Settings
          </h2>
          <p class="text-xs text-surface-600 dark:text-surface-400 mt-0.5">
            Configure application preferences
          </p>
        </div>
      </div>
    </template>

    <!-- Settings content -->
    <template #content>
      <div class="space-y-6">
        <!-- Settings groups -->
        <div class="space-y-4">
          <div class="bg-surface-50 dark:bg-surface-800 rounded-lg p-4">
            <h3
              class="text-sm font-medium text-surface-900 dark:text-surface-0 mb-3"
            >
              Installation Defaults
            </h3>
            <!-- Settings items -->
            <!-- ...existing code... -->
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>
