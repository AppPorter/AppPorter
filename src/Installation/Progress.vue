<script setup lang="ts">
import Button from "primevue/button";
import Panel from "primevue/panel";
import ProgressBar from "primevue/progressbar";
import { ref } from "vue";

// Core states for installation progress tracking
const progress = ref(0);
const currentStep = ref("");
const status = ref<"progress" | "success" | "error">("progress");
const statusMessage = ref("");
</script>

<template>
  <Panel class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #header>
      <div class="flex items-center gap-2">
        <span
          class="material-symbols-rounded text-2xl"
          :class="{
            'text-primary-500 dark:text-primary-400': status === 'progress',
            'text-green-500 dark:text-green-400': status === 'success',
            'text-red-500 dark:text-red-400': status === 'error',
          }"
        >
          {{
            status === "success"
              ? "task_alt"
              : status === "error"
                ? "error"
                : "pending"
          }}
        </span>
        <span class="font-medium text-surface-900 dark:text-surface-0"
          >Installation Progress</span
        >
      </div>
    </template>

    <div class="p-4 space-y-4">
      <ProgressBar
        :value="progress"
        class="h-2"
        :class="{
          'bg-surface-200 dark:bg-surface-700': status === 'progress',
          'bg-green-100 dark:bg-green-900/30': status === 'success',
          'bg-red-100 dark:bg-red-900/30': status === 'error',
        }"
      />

      <div class="text-center space-y-1">
        <p class="text-sm font-medium text-surface-900 dark:text-surface-0">
          {{ currentStep }}
        </p>
        <p class="text-xs text-surface-600 dark:text-surface-400">
          {{ statusMessage }}
        </p>
      </div>

      <div class="flex justify-end gap-2">
        <Button v-if="status === 'success'" severity="success">
          <span class="material-symbols-rounded mr-2">done</span>
          Finish
        </Button>
        <Button v-if="status === 'error'" severity="danger">
          <span class="material-symbols-rounded mr-2">refresh</span>
          Retry
        </Button>
      </div>
    </div>
  </Panel>
</template>
