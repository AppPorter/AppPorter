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
import { computed, ref, watch, watchEffect } from "vue";

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
          icon: isLast
            ? isExecutable
              ? "material-symbols-rounded terminal"
              : "material-symbols-rounded description"
            : "material-symbols-rounded folder",
          children: isLast ? undefined : [],
          selectable: isExecutable,
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

// Add selected class state
const selectedClass = computed(() => {
  if (!executable_path.value) return "";
  return autoConfirmed.value
    ? "bg-blue-100 text-blue-800"
    : "bg-green-100 text-green-800";
});

// Add selection style computing
const getNodeClass = computed(() => (node: PrimeTreeNode) => {
  const customNode = node as unknown as CustomTreeNode;
  if (
    !customNode.data?.isExecutable ||
    customNode.data.path !== executable_path.value
  )
    return "";

  return autoConfirmed.value
    ? "bg-[#e8f0fe] outline outline-1 outline-[#1a73e8]"
    : "bg-[#e8f0fe]";
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

// Update onNodeSelect handler to use PrimeVue's type first then cast to our type
function onNodeSelect(node: PrimeTreeNode) {
  const customNode = node as unknown as CustomTreeNode;
  if (customNode.data?.isExecutable) {
    executable_path.value = customNode.data.path;
  } else {
    selectedNode.value = {}; // Clear selection if not executable
  }
}
</script>

<template>
  <Panel
    class="h-full flex flex-col shadow-sm border border-gray-200/80 dark:border-gray-800/80 rounded-md overflow-hidden"
    :class="[
      props.detailsLoading ? 'opacity-60 pointer-events-none' : 'opacity-100',
    ]"
  >
    <template #header>
      <div class="flex items-center gap-2 py-1">
        <span class="material-symbols-rounded text-lg opacity-80"
          >folder_zip</span
        >
        <span class="text-base font-medium">Files in Archive</span>
      </div>
    </template>

    <div class="flex-1 flex flex-col min-h-0 space-y-3 p-4">
      <div
        class="flex-1 min-h-0 border border-gray-200/80 dark:border-gray-800/80 rounded-md overflow-hidden"
      >
        <div v-if="loading" class="text-sm opacity-60 p-2">Loading...</div>
        <div v-else-if="fileTree.length === 0" class="text-sm opacity-60 p-2">
          No files found in archive
        </div>
        <Tree
          v-else
          :value="fileTree"
          v-model:selectionKeys="selectedNode"
          v-model:expandedKeys="expandedKeys"
          class="h-full overflow-auto border-none [&_.p-tree-container_.p-treenode]:py-1 [&_.p-tree-container_.p-treenode_.p-treenode-content]:px-2 [&_.p-tree-container_.p-treenode_.p-treenode-content]:py-1 [&_.p-tree-container_.p-treenode_.p-treenode-content]:rounded-sm"
          selectionMode="single"
          @node-select="onNodeSelect"
        >
          <template #default="{ node }">
            <div
              class="flex items-center gap-2 rounded px-1"
              :class="getNodeClass(node)"
            >
              <span :class="node.icon"></span>
              <span>{{ node.label }}</span>
            </div>
          </template>
        </Tree>
      </div>

      <!-- Filter options -->
      <div class="shrink-0 space-y-3">
        <div
          class="bg-gray-50/50 dark:bg-gray-900/50 rounded-md p-2.5 space-y-1.5"
        >
          <div
            v-for="mode in filterModes"
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
              class="flex items-center gap-2 cursor-pointer"
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
            :severity="isConfirmed || autoConfirmed ? 'success' : 'secondary'"
            class="h-9 text-sm min-w-[8rem] transition-all duration-200"
            :disabled="!executable_path || isConfirmed"
            @click="executable_path && !autoConfirmed && confirmSelection()"
          >
            <span class="material-symbols-rounded text-lg mr-1.5">
              {{ isConfirmed || autoConfirmed ? "check_circle" : "task_alt" }}
            </span>
            {{ autoConfirmed ? "Auto Confirmed" : "Confirm" }}
          </Button>
        </div>
      </div>
    </div>
  </Panel>
</template>
