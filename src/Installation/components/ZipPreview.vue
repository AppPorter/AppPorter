<script setup lang="ts">
import { useInstallationConfigStore } from "@/stores/installation_config";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { readFile } from "@tauri-apps/plugin-fs";
import JSZip from "jszip";
import { storeToRefs } from "pinia";
import Button from "primevue/button";
import Panel from "primevue/panel";
import RadioButton from "primevue/radiobutton";
import Tree from "primevue/tree";
import type { TreeNode } from "primevue/treenode";
import { computed, nextTick, ref, watchEffect } from "vue";

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
  exe: { value: "exe", label: ".exe Only", icon: "terminal" },
  executable: { value: "executable", label: "Executable Files", icon: "code" },
  all: { value: "all", label: "All Files", icon: "description" },
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
const isConfirmed = ref(false);
const autoConfirmed = ref(false);
const isExpanding = ref(false);
const isCollapsing = ref(false);

// Store
const store = useInstallationConfigStore();
const { app_icon, app_name, app_publisher, app_version, executable_path } =
  storeToRefs(store);

// Cache
const zipCache = ref<{ paths: string[]; zip: JSZip } | null>(null);

// Computed
const filteredPaths = computed(() => {
  if (!zipCache.value) return [];
  return filterFilesByMode(zipCache.value.paths, filterMode.value);
});

const isLoading = computed(() => status.value === "loading");
const isEmpty = computed(() => fileTree.value.length === 0);

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
    ? "terminal"
    : ext.endsWith(".ps1") || ext.endsWith(".bat")
      ? "code"
      : "draft";
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
  if (!props.zipPath || zipCache.value) return;

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
    confirmSelection();
  }
}

async function confirmSelection() {
  isConfirmed.value = true;
  emit("loading", true);

  const appDetailsCard = document.querySelector(".app-details-card");
  appDetailsCard?.classList.add("loading");

  try {
    listen("get_details", (event) => {
      console.log(event.payload);
    });

    const result = await invoke("execute_command", {
      command: {
        name: "GetDetails",
        path: {
          zip_path: props.zipPath,
          executable_path: executable_path.value,
        },
      },
    });
    listen("get_details", () => {});

    if (typeof result === "string") {
      const parsedResult = JSON.parse(result);

      if ("error" in parsedResult) {
        throw new Error(parsedResult.error);
      }

      const details = Array.isArray(parsedResult) ? parsedResult : null;
      if (!details) {
        throw new Error("Invalid response format");
      }

      await new Promise((resolve) => setTimeout(resolve, 100));

      app_name.value = details[0];
      app_version.value = details[1];
      app_publisher.value = details[2] || "";
      app_icon.value = details[3] || "";
    } else {
      throw new Error("Unexpected response type");
    }
  } catch (error) {
    console.error("Failed to get details:", error);
    isConfirmed.value = false;
  } finally {
    emit("loading", false);
    appDetailsCard?.classList.remove("loading");
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
  loadZipContent();
});

watchEffect(() => {
  if (zipCache.value) {
    fileTree.value = buildFileTree(filteredPaths.value);
  }
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
          <span class="material-symbols-rounded text-lg opacity-80"
            >folder_zip</span
          >
          <span class="text-base font-medium truncate">Files in Archive</span>
        </div>
        <div class="flex gap-1 ml-2 shrink-0">
          <Button
            type="button"
            class="p-1 h-6 min-w-0 hover:bg-surface-100 dark:hover:bg-surface-600"
            severity="secondary"
            :disabled="isExpanding"
            v-tooltip.bottom="'Expand All'"
            @click="expandAll"
          >
            <span
              class="material-symbols-rounded text-base"
              :class="{ 'animate-spin': isExpanding }"
            >
              {{ isExpanding ? "progress_activity" : "unfold_more" }}
            </span>
          </Button>
          <Button
            type="button"
            class="p-1 h-6 min-w-0 hover:bg-surface-100 dark:hover:bg-surface-600"
            severity="secondary"
            :disabled="isCollapsing"
            v-tooltip.bottom="'Collapse All'"
            @click="collapseAll"
          >
            <span
              class="material-symbols-rounded text-base"
              :class="{ 'animate-spin': isCollapsing }"
            >
              {{ isCollapsing ? "progress_activity" : "unfold_less" }}
            </span>
          </Button>
        </div>
      </div>
    </template>

    <div class="flex-1 flex flex-col p-1">
      <!-- File Tree -->
      <div
        class="card rounded-md border border-surface-200 dark:border-surface-700 h-[22rem] overflow-hidden"
      >
        <div v-if="isLoading" class="text-sm opacity-60">Loading...</div>
        <div v-else-if="isEmpty" class="text-sm opacity-60">No files found</div>
        <Tree
          v-else
          :value="fileTree"
          v-model:selectionKeys="selectedNode"
          v-model:expandedKeys="expandedKeys"
          class="h-full overflow-auto"
          selectionMode="single"
          toggleOnClick
          @node-select="handleNodeSelect"
        >
          <template #default="{ node }">
            <div class="flex items-center gap-1.5 rounded px-1">
              <span class="material-symbols-rounded text-lg">{{
                node.icon
              }}</span>
              <span>{{ node.label }}</span>
            </div>
          </template>
        </Tree>
      </div>

      <!-- Filter Controls -->
      <div class="absolute bottom-2 left-2">
        <div
          class="bg-surface-50 dark:bg-surface-800 rounded-md p-2 space-y-1.5"
        >
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
              <span class="material-symbols-rounded">{{ mode.icon }}</span>
              <span class="text-sm">{{ mode.label }}</span>
            </label>
          </div>
        </div>
      </div>

      <!-- Action Button -->
      <div class="absolute bottom-2 right-2">
        <div class="flex justify-end w-full">
          <Button
            :severity="
              isConfirmed || autoConfirmed
                ? autoConfirmed
                  ? 'info'
                  : 'success'
                : 'secondary'
            "
            class="h-8 text-sm min-w-[7rem] transition-all duration-200"
            :disabled="!executable_path || isConfirmed"
            @click="executable_path && !autoConfirmed && confirmSelection()"
          >
            <span class="material-symbols-rounded text-lg mr-1">
              {{ isConfirmed || autoConfirmed ? "check_circle" : "task_alt" }}
            </span>
            {{ autoConfirmed ? "Auto Confirmed" : "Confirm" }}
          </Button>
        </div>
      </div>
    </div>
  </Panel>
</template>
