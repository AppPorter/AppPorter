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
import { ref, watch, watchEffect } from "vue";
import TreeView from "./TreeView.vue";

// Define tree node type
interface TreeNode {
  name: string;
  path: string;
  type: "file" | "directory";
  children?: TreeNode[];
  isExecutable?: boolean;
}

type FilterMode = "exe" | "executable" | "all";

// Define filter modes
const filterModes = {
  exe: { value: "exe", label: ".exe Only" },
  executable: { value: "executable", label: "Executable Files" },
  all: { value: "all", label: "All Files" },
} as const;

// Change default filter mode
const filterMode = ref<FilterMode>(filterModes.exe.value);

// Cached zip content
const zipContent = ref<{ paths: string[]; zip: JSZip } | null>(null);

// Convert flat paths to tree structure
function pathsToTree(paths: string[]): TreeNode[] {
  const root: TreeNode[] = [];

  paths.forEach((path) => {
    const parts = path.split("/");
    let current = root;

    parts.forEach((part, index) => {
      const isLast = index === parts.length - 1;
      const isExecutable = isLast && /\.(exe|bat|ps1)$/i.test(part);
      const existingNode = current.find((node) => node.name === part);

      if (existingNode) {
        current = existingNode.children || [];
      } else {
        const newNode: TreeNode = {
          name: part,
          path: parts.slice(0, index + 1).join("/"),
          type: isLast ? "file" : "directory",
          children: isLast ? undefined : [],
          isExecutable,
        };
        current.push(newNode);
        current = newNode.children || [];
      }
    });
  });

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
const fileTree = ref<TreeNode[]>([]);
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
</script>

<template>
  <Panel
    class="h-full flex flex-col shadow-sm border rounded-md overflow-hidden"
    :class="[
      props.detailsLoading ? 'opacity-60 pointer-events-none' : 'opacity-100',
    ]"
  >
    <template #header>
      <div class="flex items-center gap-2 py-1">
        <span class="material-symbols-rounded text-lg text-gray-600"
          >folder_zip</span
        >
        <span class="text-base font-medium">Files in Archive</span>
      </div>
    </template>

    <div class="flex-1 flex flex-col min-h-0 space-y-3 p-4">
      <!-- File tree container -->
      <div class="flex-1 min-h-0 border rounded-md overflow-hidden bg-white">
        <div v-if="loading" class="text-sm text-gray-500 p-2">Loading...</div>
        <div
          v-else-if="fileTree.length === 0"
          class="text-sm text-gray-500 p-2"
        >
          No files found in archive
        </div>
        <TreeView
          v-else
          :items="fileTree"
          :selected-path="executable_path"
          :auto-expand-root="true"
          @select="(path) => (executable_path = path)"
          class="h-full overflow-auto"
        />
      </div>

      <!-- Filter options -->
      <div class="shrink-0 space-y-3">
        <div class="bg-gray-50/80 rounded-md p-2.5 space-y-1.5">
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
            :class="[
              !isConfirmed && !autoConfirmed && 'bg-white hover:bg-gray-50',
            ]"
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
