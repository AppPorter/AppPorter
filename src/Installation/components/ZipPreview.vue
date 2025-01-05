<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useInstallationConfigStore } from "@/stores/installation_config";
import { storeToRefs } from "pinia";
import { watchEffect, ref, watch } from "vue";
import JSZip from "jszip";
import { readFile } from "@tauri-apps/plugin-fs";
import TreeView from "./TreeView.vue";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Label } from "@/components/ui/label";

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
}>();

const installationConfig = useInstallationConfigStore();
const { executable_path } = storeToRefs(installationConfig);
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

    // Auto select if only one executable
    const executables = filterPaths(zipContent.value.paths, "exe");
    if (executables.length === 1) {
      executable_path.value = executables[0];
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
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-sm flex items-center gap-2">
        <span
          v-svg="'zip'"
          class="w-6 h-6 overflow-hidden flex items-center justify-center"
        ></span>
        Files in Archive
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- File tree -->
      <div class="h-[200px] overflow-y-auto border rounded-md">
        <div v-if="loading" class="text-sm text-muted-foreground p-2">
          Loading...
        </div>
        <div
          v-else-if="fileTree.length === 0"
          class="text-sm text-muted-foreground p-2"
        >
          No files found in archive
        </div>
        <TreeView
          v-else
          :items="fileTree"
          :selected-path="executable_path"
          :auto-expand-root="true"
          @select="(path) => (executable_path = path)"
        />
      </div>

      <!-- Filter options -->
      <RadioGroup v-model="filterMode" class="flex items-center gap-4">
        <div
          v-for="mode in filterModes"
          :key="mode.value"
          class="flex items-center gap-2"
        >
          <RadioGroupItem :value="mode.value" :id="'filter-' + mode.value" />
          <Label :for="'filter-' + mode.value" class="flex items-center gap-2">
            <span
              v-svg="
                mode.value === 'exe'
                  ? 'executable'
                  : mode.value === 'executable'
                    ? 'script'
                    : 'file'
              "
              class="w-5 h-5 overflow-hidden flex items-center justify-center"
            ></span>
            {{ mode.label }}
          </Label>
        </div>
      </RadioGroup>
    </CardContent>
  </Card>
</template>
