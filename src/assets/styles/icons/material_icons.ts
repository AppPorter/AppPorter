// Material icon names used throughout the application
const USED_ICONS = [
  'settings',
  'tune',
  'install_desktop',
  'person',
  'group',
  'save',
  'folder_zip',
  'folder_open',
  'navigate_next',
  'remove',
  'close',
  'warning',
  'apps',
  'content_cut',
  'content_copy',
  'content_paste',
  'select_all',
  'download',
  'task_alt',
  'terminal',
  'home',
  'check_circle',
  'auto_awesome',
  'code',
  'draft',
  'description',
  'folder_off',
  'progress_activity',
  'unfold_more',
  'unfold_less',
  'verified',
  'info',
  'error',
  'arrow_back',
  'check',
] as const

/**
 * Dynamically injects Material Icons CSS classes
 * @returns Cleanup function that removes the injected styles
 */
export function generateMaterialIconsClasses() {
  const style = document.createElement('style')
  style.id = 'material-icons-style'

  style.textContent = USED_ICONS.map(
    (icon) => `
    .mir.${icon}::before {
      font-family: 'Material Symbols Rounded' !important;
      content: "${icon}" !important;
      font-variation-settings: 'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24;
    }`
  ).join('\n')

  document.head.appendChild(style)

  // Return cleanup function
  return () => style.remove()
}
