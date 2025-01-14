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
import Tree from "primevue/tree"; // Add this import
import type { TreeNode as PrimeTreeNode } from "primevue/treenode";
import { computed, nextTick, ref, watch, watchEffect } from "vue";

// Define our custom node data type
interface CustomNodeData {
  path: string;
  isExecutable?: boolean;
}

// Define our custom tree node interface
interface CustomTreeNode {
  key: string;
  label: string;
  icon?: string;
  children?: CustomTreeNode[];
  selectable?: boolean;
  data?: CustomNodeData;
}

// Use type assertion for tree data
const fileTree = ref<CustomTreeNode[]>([]);

// Add FilterMode type definition
type FilterMode = "exe" | "executable" | "all";

// Define filter modes
const filterModes = {
  exe: { value: "exe" as const, label: ".exe Only" },
  executable: { value: "executable" as const, label: "Executable Files" },
  all: { value: "all" as const, label: "All Files" },
} as const;

// Change default filter mode
const filterMode = ref<FilterMode>(filterModes.exe.value);

// Cached zip content
const zipContent = ref<{ paths: string[]; zip: JSZip } | null>(null);

// Add expanded keys state for PrimeVue Tree
const expandedKeys = ref<{ [key: string]: boolean }>({});

// Add function to get file type priority
function getFilePriority(name: string): number {
  if (name.toLowerCase().endsWith(".exe")) return 0;
  if (name.toLowerCase().endsWith(".bat")) return 1;
  if (name.toLowerCase().endsWith(".ps1")) return 2;
  return 3;
}

// Update getFileIcon function
function getFileIcon(fileName: string): string {
  const lowerName = fileName.toLowerCase();
  if (lowerName.endsWith(".exe")) {
    return "terminal";
  }
  if (lowerName.endsWith(".zip") || lowerName.endsWith(".ps1")) {
    return "code";
  }
  return "draft";
}

// Updated pathsToTree function with auto-expand and better sorting
function pathsToTree(paths: string[]): CustomTreeNode[] {
  const root: CustomTreeNode[] = [];

  // First pass: build tree structure
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
          selectable: true, // Keep all nodes selectable
          data: {
            path: currentPath,
            isExecutable,
          },
        };
        current.push(newNode);
        current = newNode.children || [];
      }
    });
  });

  // Second pass: sort nodes recursively
  function sortNodes(nodes: CustomTreeNode[]): void {
    nodes.sort((a, b) => {
      const aHasChildren = !!a.children?.length;
      const bHasChildren = !!b.children?.length;

      // Files before folders
      if (aHasChildren !== bHasChildren) {
        return aHasChildren ? 1 : -1;
      }

      // For files, sort by priority then name
      if (!aHasChildren && !bHasChildren) {
        const priorityA = getFilePriority(a.label);
        const priorityB = getFilePriority(b.label);
        if (priorityA !== priorityB) {
          return priorityA - priorityB;
        }
      }

      // Finally sort by name
      return a.label.localeCompare(b.label);
    });

    // Recursively sort children
    nodes.forEach((node) => {
      if (node.children?.length) {
        sortNodes(node.children);
      }
    });
  }

  sortNodes(root);

  // Auto-expand logic for single root folder
  if (root.length === 1 && root[0].children) {
    expandedKeys.value[root[0].key] = true;
  }

  return root;
}

const props = defineProps<{
  zipPath: string;
  detailsLoading?: boolean; // Add this prop
}>();

const emit = defineEmits<{
  (e: "loading", value: boolean): void;
  (e: "progress", value: number): void;
}>();

const installationConfig = useInstallationConfigStore();
const { app_icon, app_name, app_publisher, app_version, executable_path } =
  storeToRefs(installationConfig);
const loading = ref(false);

// Filter paths based on mode
function filterPaths(paths: string[], mode: FilterMode): string[] {
  return paths.filter((path) => {
    const isExe = path.toLowerCase().endsWith(".exe");
    const isExecutable = isExe || /\.(bat|ps1)$/i.test(path);

    switch (mode) {
      case "exe":
        return isExe;
      case "executable":
        return isExecutable;
      case "all":
        return true;
    }
  });
}

// Add new function to check if path is in first two levels
function isInFirstTwoLevels(path: string): boolean {
  return path.split("/").length <= 2;
}

// Add auto confirmed state
const autoConfirmed = ref(false);

// Add selection style computing
const getNodeClass = computed(() => (node: PrimeTreeNode) => {
  const customNode = node as unknown as CustomTreeNode;
  if (
    !customNode.data?.isExecutable ||
    customNode.data.path !== executable_path.value
  )
    return "";

  return ""; // Remove text color effect
});

// Modified watch effect
watchEffect(async () => {
  if (!props.zipPath) return;

  loading.value = true;
  try {
    if (!zipContent.value) {
      const data = await readFile(props.zipPath);
      const zip = await JSZip.loadAsync(data);
      const paths = Object.keys(zip.files).filter(
        (path) => !path.endsWith("/"),
      );
      zipContent.value = { paths, zip };
    }

    const filteredPaths = filterPaths(zipContent.value.paths, filterMode.value);
    fileTree.value = pathsToTree(filteredPaths);

    // Check for executables in first two levels
    const topLevelExecutables = zipContent.value.paths.filter(
      (path) => /\.(exe|bat|ps1)$/i.test(path) && isInFirstTwoLevels(path),
    );

    // If there's exactly one exe and no other executables in top levels
    const topLevelExes = topLevelExecutables.filter((path) =>
      path.toLowerCase().endsWith(".exe"),
    );
    if (topLevelExes.length === 1 && topLevelExecutables.length === 1) {
      executable_path.value = topLevelExes[0];
      // Update to use proper selection format
      selectedNode.value = { [topLevelExes[0]]: true };
      // Auto confirm after small delay to ensure UI is ready
      setTimeout(() => {
        if (!isConfirmed.value) {
          autoConfirmed.value = true; // Set auto confirmed state
          confirmSelection();
        }
      }, 100);
    } else {
      autoConfirmed.value = false; // Reset auto confirmed state
      // Auto select if only one exe exists anywhere
      const allExecutables = filterPaths(zipContent.value.paths, "exe");
      if (allExecutables.length === 1) {
        executable_path.value = allExecutables[0];
      }
    }
  } catch (error) {
    console.error("Failed to read zip:", error);
    fileTree.value = [];
  } finally {
    loading.value = false;
  }
});

// Watch filter mode changes
watch(filterMode, () => {
  if (zipContent.value) {
    const filteredPaths = filterPaths(zipContent.value.paths, filterMode.value);
    fileTree.value = pathsToTree(filteredPaths);
  }
});

// Add confirmation state
const isConfirmed = ref(false);

// Modify confirmation handler
async function confirmSelection() {
  isConfirmed.value = true;
  emit("loading", true);

  const appDetailsCard = document.querySelector(".app-details-card");
  appDetailsCard?.classList.add("loading");

  try {
    const arg = JSON.stringify({
      zip_path: props.zipPath,
      executable_path: executable_path.value,
    });

    listen("get_details", (event) => {
      console.log(event.payload);
    });

    const result = await invoke("execute_command", {
      command: "GetDetails",
      arg: arg,
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

// Reset confirmation when executable changes
watch(executable_path, () => {
  isConfirmed.value = false;
  autoConfirmed.value = false;
});

// Add tree selection handling
const selectedNode = ref<{ [key: string]: boolean }>({});

// Update onNodeSelect handler to handle both files and folders
function onNodeSelect(node: PrimeTreeNode) {
  const customNode = node as unknown as CustomTreeNode;

  if (node.children) {
    // For folders, convert selection to expand/collapse
    expandedKeys.value[node.key as string] =
      !expandedKeys.value[node.key as string];
    expandedKeys.value = { ...expandedKeys.value }; // Trigger reactivity
    nextTick(() => {
      selectedNode.value = {}; // Clear selection for folder
    });
    return;
  }

  if (customNode.data?.isExecutable) {
    // For executable files, handle normally
    executable_path.value = customNode.data.path;
    selectedNode.value = { [node.key as string]: true };
  } else {
    // For non-executable files, clear selection
    selectedNode.value = {};
  }
}

// Add loading states for expand/collapse
const isExpanding = ref(false);
const isCollapsing = ref(false);

// Modify expand/collapse functions with loading states
const expandAll = async () => {
  if (isExpanding.value) return;
  isExpanding.value = true;
  try {
    for (const node of fileTree.value) {
      await expandNode(node);
    }
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

const expandNode = async (node: CustomTreeNode) => {
  if (node.children && node.children.length) {
    expandedKeys.value[node.key] = true;
    for (const child of node.children) {
      await expandNode(child);
    }
  }
};
</script>

<template>
  <Panel
    class="h-full flex flex-col shadow-sm border border-surface-200 dark:border-surface-700 rounded-md overflow-hidden"
  >
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

    <div class="flex-1 flex flex-col min-h-0 p-1">
      <div
        class="flex-1 min-h-0 border border-surface-200 dark:border-surface-700 rounded-md overflow-hidden"
      >
        <div v-if="loading" class="text-sm opacity-60 p-1.5">Loading...</div>
        <div v-else-if="fileTree.length === 0" class="text-sm opacity-60 p-1.5">
          No files found in archive
        </div>
        <Tree
          v-else
          :value="fileTree"
          v-model:selectionKeys="selectedNode"
          v-model:expandedKeys="expandedKeys"
          class="h-full overflow-auto border-none [&_.p-tree-container_.p-treenode]:py-0.5 [&_.p-tree-container_.p-treenode_.p-treenode-content]:px-1.5 [&_.p-tree-container_.p-treenode_.p-treenode-content]:py-0.5 [&_.p-tree-container_.p-treenode_.p-treenode-content]:rounded-sm [&_.p-treenode-content.p-highlight]:bg-green-50 cursor-pointer"
          selectionMode="single"
          toggleOnClick
          @node-select="onNodeSelect"
        >
          <template #default="{ node }">
            <div
              class="flex items-center gap-1.5 rounded px-1"
              :class="getNodeClass(node)"
            >
              <span class="material-symbols-rounded text-lg">{{
                node.icon
              }}</span>
              <span>{{ node.label }}</span>
            </div>
          </template>
        </Tree>
      </div>

      <!-- Filter options -->
      <div class="shrink-0 mt-1 space-y-1">
        <div
          class="bg-surface-50 dark:bg-surface-800 rounded-md p-2 space-y-1.5"
        >
          <div
            v-for="mode in filterModes"
            :key="mode.value"
            class="flex items-center gap-2 px-1.5 py-0.5"
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
              <span class="material-symbols-rounded">
                {{
                  mode.value === "exe"
                    ? "terminal"
                    : mode.value === "executable"
                      ? "code"
                      : "description"
                }}
              </span>
              <span>{{ mode.label }}</span>
            </label>
          </div>
        </div>

        <!-- Confirm button -->
        <div class="flex justify-end">
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
