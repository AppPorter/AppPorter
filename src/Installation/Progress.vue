<script setup lang="ts">
import Button from "primevue/button";
import Panel from "primevue/panel";
import ProgressBar from "primevue/progressbar";
import { ref } from "vue";

const progress = ref(0);
const currentStep = ref("");
const status = ref<"progress" | "success" | "error">("progress");
const statusMessage = ref("");

// 这里之后添加安装进度的实际逻辑
</script>

<template>
  <Panel class="border border-gray-200/80 dark:border-gray-800/80 shadow-sm">
    <template #header>
      <div class="flex items-center gap-2">
        <span
          class="material-symbols-rounded text-2xl"
          :class="{
            'text-blue-500': status === 'progress',
            'text-green-500': status === 'success',
            'text-red-500': status === 'error',
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
        <span class="font-medium">Installation Progress</span>
      </div>
    </template>

    <div class="p-4 space-y-4">
      <ProgressBar
        :value="progress"
        class="h-2"
        :class="{
          'surface-200 dark:surface-700': status === 'progress',
          'bg-green-100 dark:bg-green-900/30': status === 'success',
          'bg-red-100 dark:bg-red-900/30': status === 'error',
        }"
      />

      <div class="text-center">
        <p class="text-sm font-medium">{{ currentStep }}</p>
        <p class="text-xs opacity-60">{{ statusMessage }}</p>
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
