export function generateMaterialIconsClasses() {
  let style = document.getElementById("material-icons-style");
  if (!style) {
    style = document.createElement("style");
    style.id = "material-icons-style";
    document.head.appendChild(style);
  }

  const processedIcons = new Set<string>();
  let updateTimer: number | null = null;

  const updateStyles = () => {
    const rules = Array.from(processedIcons)
      .map(
        (iconName) => `
        .mir.${iconName}::before {
          font-family: 'Material Symbols Rounded' !important;
          content: "${iconName}" !important;
          font-variation-settings: 'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24;
        }`,
      )
      .join("\n");

    style!.textContent = rules;
  };

  const debouncedUpdate = () => {
    if (updateTimer) {
      clearTimeout(updateTimer);
    }
    updateTimer = window.setTimeout(updateStyles, 100);
  };

  const processElement = (el: Element) => {
    if (!el.classList?.contains("mir")) return;

    const iconClass = Array.from(el.classList).find(
      (cls) => cls !== "mir" && !cls.includes("-") && !cls.includes(":"),
    );

    if (iconClass && !processedIcons.has(iconClass)) {
      processedIcons.add(iconClass);
      debouncedUpdate();
    }
  };

  document.querySelectorAll(".mir").forEach(processElement);

  const observer = new MutationObserver((mutations) => {
    let hasChanges = false;
    mutations.forEach((mutation) => {
      mutation.addedNodes.forEach((node) => {
        if (node instanceof Element) {
          processElement(node);
          node.querySelectorAll(".mir").forEach(processElement);
          hasChanges = true;
        }
      });
    });

    if (hasChanges) {
      debouncedUpdate();
    }
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });

  return () => observer.disconnect();
}
