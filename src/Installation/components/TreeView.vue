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

// Add sorting function
function getFileTypePriority(filename: string): number {
  const ext = filename.toLowerCase().split(".").pop() || "";
  // Priority order: exe > bat > ps1 > others
  switch (ext) {
    case "exe":
      return 0;
    case "bat":
      return 1;
    case "ps1":
      return 2;
    default:
      return 3;
  }
}

function sortItems(items: TreeNode[]): TreeNode[] {
  return [...items].sort((a, b) => {
    // First sort by type (files first)
    if (a.type !== b.type) {
      return a.type === "file" ? -1 : 1;
    }

    // If both are files, sort by file type priority
    if (a.type === "file") {
      const priorityA = getFileTypePriority(a.name);
      const priorityB = getFileTypePriority(b.name);
      if (priorityA !== priorityB) {
        return priorityA - priorityB;
      }
    }

    // Finally sort alphabetically
    return a.name.localeCompare(b.name);
  });
}
</script>

<template>
  <div class="pl-2">
    <template v-for="item in sortItems(items)" :key="item.path">
      <!-- Directory node -->
      <div
        v-if="item.type === 'directory'"
        class="flex items-center gap-2 py-1 px-2 text-sm text-surface-600 dark:text-surface-400 cursor-pointer rounded-sm transition-colors hover:bg-surface-100 dark:hover:bg-surface-800"
        :class="{
          'bg-surface-100 dark:bg-surface-800': expandedDirs.has(item.path),
        }"
        @click="toggleDir(item.path)"
      >
        <span class="material-symbols-rounded text-lg">
          {{ expandedDirs.has(item.path) ? "expand_more" : "navigate_next" }}
        </span>
        <span class="material-symbols-rounded">
          {{ expandedDirs.has(item.path) ? "folder_open" : "folder" }}
        </span>
        <span class="truncate">{{ item.name }}</span>
      </div>

      <!-- File node -->
      <div
        v-else
        class="flex items-center gap-2 py-1 px-2 pl-9 text-sm rounded-sm transition-colors"
        :class="[
          item.isExecutable
            ? 'cursor-pointer hover:bg-primary-50 dark:hover:bg-primary-900/20 text-surface-800 dark:text-surface-200'
            : 'text-surface-500 dark:text-surface-400',
          selectedPath === item.path &&
            'bg-primary-100 dark:bg-primary-900/30 text-primary-800 dark:text-primary-200',
        ]"
        @click="item.isExecutable && $emit('select', item.path)"
      >
        <span class="material-symbols-rounded">
          {{ item.isExecutable ? "terminal" : "description" }}
        </span>
        <span class="truncate">{{ item.name }}</span>
      </div>

      <!-- Recursive children -->
      <TreeView
        v-if="item.children && expandedDirs.has(item.path)"
        class="pl-6"
        :items="sortItems(item.children)"
        :selected-path="selectedPath"
        :auto-expand-root="false"
        @select="$emit('select', $event)"
      />
    </template>
  </div>
</template>
