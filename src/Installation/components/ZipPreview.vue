<script setup lang="ts">
import { useInstallationConfigStore } from "@/stores/installation_config";
import { useSettingsStore } from "@/stores/settings";
import { readFile } from "@tauri-apps/plugin-fs";
import Color from "color";
import JSZip from "jszip";
import { storeToRefs } from "pinia";
import Button from "primevue/button";
import Panel from "primevue/panel";
import ProgressBar from "primevue/progressbar";
import RadioButton from "primevue/radiobutton";
import Tree from "primevue/tree";
import type { TreeNode } from "primevue/treenode";
import { computed, nextTick, onMounted, ref, watchEffect } from "vue";

const settingsStore = useSettingsStore();

// Types
type FilterMode = "exe" | "executable" | "all";
type FileStatus = "loading" | "ready" | "error";

interface Props {
  zipPath: string;
  detailsLoading?: boolean;
}

interface CustomNodeData {
  path: string;
  isExecutable?: boolean;
}

interface CustomTreeNode extends TreeNode {
  key: string;
  label: string;
  icon?: string;
  children?: CustomTreeNode[];
  selectable?: boolean;
  data?: CustomNodeData;
}

// Core constants
const FILTER_MODES = {
  exe: { value: "exe", label: ".exe Only", icon: "mir terminal" },
  executable: {
    value: "executable",
    label: "Executable Files",
    icon: "mir code",
  },
  all: { value: "all", label: "All Files", icon: "mir description" },
} as const;

const FILE_PRIORITIES = {
  exe: 0,
  bat: 1,
  ps1: 2,
  other: 3,
} as const;

// Props & emits
const props = defineProps<Props>();
const emit = defineEmits<{
  (event: "loading", value: boolean): void;
  (event: "progress", value: number): void;
}>();

// Core states
const status = ref<FileStatus>("ready");
const filterMode = ref<FilterMode>("exe");
const expandedKeys = ref<Record<string, boolean>>({});
const selectedNode = ref<Record<string, boolean>>({});
const fileTree = ref<CustomTreeNode[]>([]);
const isExpanding = ref(false);
const isCollapsing = ref(false);

// Store
const store = useInstallationConfigStore();
const { icon, name, publisher, version, executable_path } = storeToRefs(store);

// Cache
const zipCache = ref<{ paths: string[]; zip: JSZip } | null>(null);

// Computed
const filteredPaths = computed(() => {
  if (!zipCache.value) return [];
  return filterFilesByMode(zipCache.value.paths, filterMode.value);
});

const hasScanned = computed(() => zipCache.value !== null);
const isEmpty = computed(() => hasScanned.value && fileTree.value.length === 0);

// Loading states
const progressMode = ref<"indeterminate" | "determinate">("indeterminate");
const loadingProgress = ref(0);

// Core functions
function getFilePriority(name: string): number {
  const ext = name.toLowerCase().split(".").pop() || "";
  return ext in FILE_PRIORITIES
    ? FILE_PRIORITIES[ext as keyof typeof FILE_PRIORITIES]
    : FILE_PRIORITIES.other;
}

function getFileIcon(fileName: string): string {
  const ext = fileName.toLowerCase();
  return ext.endsWith(".exe")
    ? "mir terminal"
    : ext.endsWith(".ps1") || ext.endsWith(".bat")
      ? "mir code"
      : "mir draft";
}

function filterFilesByMode(paths: string[], mode: FilterMode): string[] {
  return paths.filter((path) => {
    const isExe = path.toLowerCase().endsWith(".exe");
    const isExecutable = isExe || /\.(bat|ps1)$/i.test(path);
    return mode === "exe" ? isExe : mode === "executable" ? isExecutable : true;
  });
}

// Tree operations
function buildFileTree(paths: string[]): CustomTreeNode[] {
  const root: CustomTreeNode[] = [];

  paths.forEach((path) => {
    const parts = path.split("/");
    let current = root;

    parts.forEach((part, index) => {
      const isLast = index === parts.length - 1;
      const isExecutable = isLast && /\.(exe|bat|ps1)$/i.test(part);
      const currentPath = parts.slice(0, index + 1).join("/");
      const existingNode = current.find((node) => node.label === part);

      if (existingNode) {
        current = existingNode.children || [];
      } else {
        const newNode: CustomTreeNode = {
          key: currentPath,
          label: part,
          icon: isLast ? getFileIcon(part) : "folder",
          children: isLast ? undefined : [],
          selectable: true,
          data: { path: currentPath, isExecutable },
        };
        current.push(newNode);
        current = newNode.children || [];
      }
    });
  });

  // Sort nodes recursively
  const sortNodes = (nodes: CustomTreeNode[]): void => {
    nodes.sort((a, b) => {
      const aHasChildren = !!a.children?.length;
      const bHasChildren = !!b.children?.length;
      if (aHasChildren !== bHasChildren) return aHasChildren ? 1 : -1;
      if (!aHasChildren && !bHasChildren) {
        const priorityDiff =
          getFilePriority(a.label) - getFilePriority(b.label);
        if (priorityDiff !== 0) return priorityDiff;
      }
      return a.label.localeCompare(b.label);
    });
    nodes.forEach((node) => node.children?.length && sortNodes(node.children));
  };

  sortNodes(root);
  if (root.length === 1 && root[0].children) {
    expandedKeys.value[root[0].key] = true;
  }

  return root;
}

function handleNodeSelect(node: TreeNode) {
  const customNode = node as CustomTreeNode;
  if (customNode.children) {
    expandedKeys.value[customNode.key] = !expandedKeys.value[customNode.key];
    expandedKeys.value = { ...expandedKeys.value };
    nextTick(() => (selectedNode.value = {}));
    return;
  }

  if (customNode.data?.isExecutable) {
    executable_path.value = customNode.data.path;
    selectedNode.value = { [customNode.key]: true };
  } else {
    selectedNode.value = {};
  }
}

// File operations
async function loadZipContent() {
  if (!props.zipPath) return;

  status.value = "loading";
  try {
    const data = await readFile(props.zipPath);
    const zip = await JSZip.loadAsync(data);
    const paths = Object.keys(zip.files).filter((path) => !path.endsWith("/"));
    zipCache.value = { paths, zip };

    fileTree.value = buildFileTree(filteredPaths.value);
    handleAutoExeSelection();
  } catch (error) {
    console.error("Failed to read zip:", error);
    fileTree.value = [];
    status.value = "error";
  } finally {
    status.value = "ready";
  }
}

function handleAutoExeSelection() {
  if (!zipCache.value) return;

  const topLevelExes = zipCache.value.paths.filter(
    (path) =>
      path.toLowerCase().endsWith(".exe") && path.split("/").length <= 2,
  );

  if (topLevelExes.length === 1) {
    executable_path.value = topLevelExes[0];
    selectedNode.value = { [topLevelExes[0]]: true };
  }
}

// Add expansion handlers
const expandAll = async () => {
  if (isExpanding.value) return;
  isExpanding.value = true;
  try {
    const expandNode = async (node: CustomTreeNode) => {
      if (node.children?.length) {
        expandedKeys.value[node.key] = true;
        await Promise.all(node.children.map(expandNode));
      }
    };
    await Promise.all(fileTree.value.map(expandNode));
    expandedKeys.value = { ...expandedKeys.value };
  } finally {
    isExpanding.value = false;
  }
};

const collapseAll = async () => {
  if (isCollapsing.value) return;
  isCollapsing.value = true;
  try {
    expandedKeys.value = {};
    await nextTick();
  } finally {
    isCollapsing.value = false;
  }
};

// Effects
watchEffect(() => {
  if (!props.zipPath) return;
  zipCache.value = null;
  selectedNode.value = {};
  executable_path.value = "";
  loadZipContent();
});

watchEffect(() => {
  if (zipCache.value) {
    fileTree.value = buildFileTree(filteredPaths.value);
  }
});

onMounted(async () => {
  if (props.zipPath) {
    emit("loading", true);
    try {
      progressMode.value = "indeterminate";
      await loadZipContent();
      loadingProgress.value = 100;

      setTimeout(() => {
        emit("loading", false);
        loadingProgress.value = 0;
      }, 500);
    } catch (error) {
      console.error("Failed to load zip content:", error);
      emit("loading", false);
    }
  }
});

function selectColor(): string {
  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    return Color(settingsStore.color).lightness(80).hex();
  } else {
    return Color(settingsStore.color).lightness(20).hex();
  }
}
const selectedColor = ref(selectColor());
window
  .matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", () => {
    selectedColor.value = selectColor();
  });
</script>

<template>
  <Panel
    class="h-full flex flex-col shadow-sm border border-surface-200 dark:border-surface-700 rounded-md overflow-hidden relative"
  >
    <!-- Header -->
    <template #header>
      <div class="flex justify-between items-center w-full">
        <div class="flex items-center gap-1 flex-1 min-w-0">
          <span class="mir folder_zip text-lg opacity-80"></span>
          <span class="text-base font-medium">Files in Archive</span>
        </div>
        <div class="flex gap-1 ml-2 shrink-0">
          <Button
            type="button"
            class="p-1 h-6 min-w-0 hover:bg-surface-100 dark:hover:bg-surface-600"
            severity="secondary"
            :disabled="isExpanding"
            v-tooltip.bottom="'Expand All'"
            @click="expandAll"
            :icon="isExpanding ? 'mir progress_activity' : 'mir unfold_more'"
          />
          <Button
            type="button"
            class="p-1 h-6 min-w-0 hover:bg-surface-100 dark:hover:bg-surface-600"
            severity="secondary"
            :disabled="isCollapsing"
            v-tooltip.bottom="'Collapse All'"
            @click="collapseAll"
            :icon="isCollapsing ? 'mir progress_activity' : 'mir unfold_less'"
          />
        </div>
      </div>
    </template>

    <div class="flex-1 flex flex-col p-1">
      <!-- File Tree -->
      <div class="card h-[54vh]">
        <Tree
          v-if="hasScanned && !isEmpty"
          :value="fileTree"
          v-model:selectionKeys="selectedNode"
          v-model:expandedKeys="expandedKeys"
          class="h-full overflow-auto bg-surface-50 dark:bg-surface-800"
          selectionMode="single"
          toggleOnClick
          @node-select="handleNodeSelect"
        />

        <!-- Empty State Overlay -->
        <div
          v-if="hasScanned && isEmpty"
          class="absolute inset-0 backdrop-blur-[2px] bg-surface-0/60 dark:bg-surface-900/60 flex flex-col items-center justify-center gap-2"
        >
          <span
            class="mir folder_off text-4xl text-surface-400 dark:text-surface-600"
          ></span>
          <p class="text-sm text-surface-600 dark:text-surface-400">
            No files found
          </p>
        </div>
      </div>

      <!-- Filter Controls -->
      <div class="absolute bottom-2 left-2 w-full">
        <div class="rounded-md p-2 space-y-1.5">
          <div
            v-for="mode in Object.values(FILTER_MODES)"
            :key="mode.value"
            class="flex items-center gap-2"
          >
            <RadioButton
              v-model="filterMode"
              :value="mode.value"
              :inputId="'filter-' + mode.value"
            />
            <label
              :for="'filter-' + mode.value"
              class="flex items-center gap-2.5 cursor-pointer"
            >
              <span class="mir" :class="mode.icon"></span>
              <span class="text-sm">{{ mode.label }}</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading Overlay -->
    <div
      v-if="props.detailsLoading"
      class="absolute inset-0 backdrop-blur-[2px] bg-surface-0/60 dark:bg-surface-900/60 flex flex-col items-center justify-center gap-2 transition-all duration-300"
    >
      <h3 class="text-base font-semibold text-surface-900 dark:text-surface-0">
        Reading Archive
      </h3>
      <ProgressBar :mode="progressMode" :value="loadingProgress" class="w-40" />
      <p class="text-sm text-surface-600 dark:text-surface-400">
        {{ loadingProgress === 100 ? "Completed" : "Loading..." }}
      </p>
    </div>
  </Panel>
</template>
