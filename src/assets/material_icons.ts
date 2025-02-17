// All used icons collection
const USED_ICONS = [
  "settings",
  "tune",
  "install_desktop",
  "person",
  "group",
  "save",
  "folder_zip",
  "folder_open",
  "navigate_next",
  "remove",
  "close",
  "warning",
  "apps",
  "content_cut",
  "content_copy",
  "content_paste",
  "select_all",
  "download",
  "task_alt",
  "terminal",
  "home",
  "check_circle",
  "auto_awesome",
  "code",
  "draft",
  "description",
  "folder_off",
  "progress_activity",
  "unfold_more",
  "unfold_less",
  "verified",
] as const;

export function generateMaterialIconsClasses() {
  const style = document.createElement("style");
  style.id = "material-icons-style";

  const rules = USED_ICONS.map(
    (iconName) => `
    .mir.${iconName}::before {
      font-family: 'Material Symbols Rounded' !important;
      content: "${iconName}" !important;
      font-variation-settings: 'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24;
    }`,
  ).join("\n");

  style.textContent = rules;
  document.head.appendChild(style);

  return () => {
    style.remove();
  };
}
