export function generateMaterialIconsClasses() {
  let style = document.getElementById("material-icons-style");
  if (!style) {
    style = document.createElement("style");
    style.id = "material-icons-style";
    document.head.appendChild(style);
  }

  const processedIcons = new Set<string>();
  let updateScheduled = false;

  const updateStyles = () => {
    if (updateScheduled) return;
    updateScheduled = true;

    requestAnimationFrame(() => {
      const rules = Array.from(processedIcons)
        .map(
          (iconName) => `
          [class*="mir ${iconName}"]::before,
          [class*=" mir ${iconName}"]::before,
          [class^="mir ${iconName}"]::before {
            font-family: 'Material Symbols Rounded' !important;
            content: "${iconName}" !important;
          }
        `,
        )
        .join("\n");

      style!.textContent = `
        .mir::before {
          display: inline-block !important;
          font-style: normal;
          font-weight: normal !important;
          font-variant: normal;
          text-transform: none;
          line-height: 1;
          vertical-align: -0.125em;
          -webkit-font-smoothing: antialiased;
          -moz-osx-font-smoothing: grayscale;
          width: 1em;
          height: 1em;
          font-variation-settings: 'FILL' 0, 'wght' 400, 'GRAD' 0, 'opsz' 24;
        }
        ${rules}
      `;
      updateScheduled = false;
    });
  };

  const processElement = (el: Element) => {
    if (!el.classList?.contains("mir")) return;

    const classList = Array.from(el.classList);
    const mirIndex = classList.indexOf("mir");
    if (mirIndex !== -1 && mirIndex + 1 < classList.length) {
      const nextClass = classList[mirIndex + 1];
      if (nextClass && !nextClass.includes("-") && !nextClass.includes(":")) {
        processedIcons.add(nextClass);
      }
    }
  };

  // Initial scan and style generation
  document.querySelectorAll("*").forEach(processElement);
  updateStyles();

  const observer = new MutationObserver((mutations) => {
    let hasChanges = false;
    for (const mutation of mutations) {
      for (const node of mutation.addedNodes) {
        if (node instanceof Element) {
          processElement(node);
          node.querySelectorAll("*").forEach(processElement);
          hasChanges = true;
        }
      }
    }
    if (hasChanges) {
      updateStyles();
    }
  });

  observer.observe(document.body, {
    childList: true,
    subtree: true,
  });

  return () => observer.disconnect();
}
