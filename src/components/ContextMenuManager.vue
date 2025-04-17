<script setup lang="ts">
import ContextMenu from 'primevue/contextmenu'
import type { MenuItem } from 'primevue/menuitem'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const contextMenu = ref()

// Context menu setup
const editMenuItems = ref<MenuItem[]>([
  {
    label: t('edit.cut'),
    icon: 'mir-content_cut',
    command: () => document.execCommand('cut'),
  },
  {
    label: t('edit.copy'),
    icon: 'mir-content_copy',
    command: () => document.execCommand('copy'),
  },
  {
    label: t('edit.paste'),
    icon: 'mir-content_paste',
    command: async () =>
      document.execCommand('insertText', false, await navigator.clipboard.readText()),
  },
  { separator: true },
  {
    label: t('edit.select_all'),
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
  handleContextMenu
})
</script>

<template>
  <ContextMenu ref="contextMenu" :model="editMenuItems" />
</template>