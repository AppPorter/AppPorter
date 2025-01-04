<script setup lang="ts">
// Remove ChevronRight import as we'll use our own icons
// import { ChevronRight } from "lucide-vue-next";
import { ref, watch } from "vue";

interface TreeNode {
  name: string;
  path: string;
  type: "file" | "directory";
  children?: TreeNode[];
  isExecutable?: boolean;
}

const props = defineProps<{
  items: TreeNode[];
  selectedPath: string;
  autoExpandRoot?: boolean;
}>();

defineEmits<{
  (e: "select", path: string): void;
}>();

const expandedDirs = ref<Set<string>>(new Set());

watch(
  () => props.items,
  (items) => {
    if (
      props.autoExpandRoot &&
      items.length === 1 &&
      items[0].type === "directory"
    ) {
      expandedDirs.value.add(items[0].path);
    }
  },
  { immediate: true },
);

function toggleDir(path: string) {
  if (expandedDirs.value.has(path)) {
    expandedDirs.value.delete(path);
  } else {
    expandedDirs.value.add(path);
  }
}
</script>

<template>
  <div class="pl-2">
    <template v-for="item in items" :key="item.path">
      <!-- Directory node -->
      <div
        v-if="item.type === 'directory'"
        class="flex items-center gap-2 py-1 px-2 text-sm text-gray-600 cursor-pointer rounded transition-colors hover:bg-gray-100 hover:text-gray-900"
        :class="{ 'bg-gray-200': expandedDirs.has(item.path) }"
        @click="toggleDir(item.path)"
      >
        <span
          class="w-4 h-4 shrink-0 flex items-center justify-center -ml-0.5"
          :class="{ 'rotate-90': expandedDirs.has(item.path) }"
          >▸</span
        >
        <span
          v-svg="expandedDirs.has(item.path) ? 'open_folder' : 'folder'"
          class="w-4 h-4 shrink-0"
        ></span>
        <span class="truncate">{{ item.name }}</span>
      </div>

      <!-- File node -->
      <div
        v-else
        class="flex items-center gap-1 py-1 px-2 pl-6 text-sm rounded transition-colors"
        :class="[
          item.isExecutable
            ? 'cursor-pointer hover:bg-gray-100 hover:text-gray-900 text-gray-800 font-medium'
            : 'text-gray-500',
          {
            'bg-blue-50 text-blue-700 font-medium': selectedPath === item.path,
          },
        ]"
        @click="item.isExecutable && $emit('select', item.path)"
      >
        <span
          v-svg="item.isExecutable ? 'executable' : 'file'"
          class="w-4 h-4 shrink-0 opacity-70 mr-1"
        ></span>
        <span class="truncate">{{ item.name }}</span>
      </div>

      <!-- Recursive children -->
      <TreeView
        v-if="item.children && expandedDirs.has(item.path)"
        class="pl-4"
        :items="item.children"
        :selected-path="selectedPath"
        :auto-expand-root="false"
        @select="$emit('select', $event)"
      />
    </template>
  </div>
</template>
