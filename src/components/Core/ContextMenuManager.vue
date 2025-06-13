<script setup lang="ts">
import { readText } from '@tauri-apps/plugin-clipboard-manager'
import ContextMenu from 'primevue/contextmenu'
import type { MenuItem } from 'primevue/menuitem'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const contextMenu = ref()

// Context menu setup
const editMenuItems = ref<MenuItem[]>([
  {
    label: t('cls.edit.cut'),
    icon: 'mir-content_cut',
    command: () => document.execCommand('cut'),
  },
  {
    label: t('cls.edit.copy'),
    icon: 'mir-content_copy',
    command: () => document.execCommand('copy'),
  },
  {
    label: t('cls.edit.paste'),
    icon: 'mir-content_paste',
    command: async () => {
      try {
        document.execCommand('insertText', false, await readText())
      } catch { /* empty */ }
    },
  },
  { separator: true },
  {
    label: t('cls.edit.select_all'),
    icon: 'mir-select_all',
    command: () => document.execCommand('selectAll'),
  },
])

// Show context menu only for text input elements
function handleContextMenu(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
    contextMenu.value?.show(event)
  }
  event.preventDefault()
}

defineExpose({
  handleContextMenu,
})
</script>

<template>
  <ContextMenu ref="contextMenu" :model="editMenuItems" />
</template>
