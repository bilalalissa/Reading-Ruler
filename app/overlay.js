const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();
const rulerId = appWindow.label.replace(/^overlay-/, "");
const ruler = document.querySelector("#ruler");
const resizeHandles = document.querySelectorAll("[data-resize-direction]");

let geometrySaveTimer = null;
let currentSettings = null;
let imageRequestId = 0;

function hexToRgba(hex, alpha) {
  const normalized = hex.replace("#", "");
  const value = Number.parseInt(normalized, 16);
  const red = (value >> 16) & 255;
  const green = (value >> 8) & 255;
  const blue = value & 255;

  return `rgba(${red}, ${green}, ${blue}, ${alpha})`;
}

async function loadBackgroundImage(settings) {
  const requestId = ++imageRequestId;
  if (settings.pattern !== "image" || !settings.backgroundImagePath) {
    ruler.style.setProperty("--background-image", "none");
    return;
  }

  try {
    const dataUrl = await invoke("background_image_data_url", { rulerId });
    if (requestId !== imageRequestId) {
      return;
    }
    ruler.style.setProperty(
      "--background-image",
      dataUrl ? `url("${dataUrl}")` : "none",
    );
  } catch (error) {
    console.error("Unable to load background image", error);
    ruler.style.setProperty("--background-image", "none");
  }
}

function applySettings(settings) {
  currentSettings = settings;
  const borderColor = settings.borderColor || "#006858";
  const backgroundColor = settings.backgroundColor || "#ffef75";
  const opacity = Number(settings.backgroundOpacity ?? 0.46);
  const spacing = Number(settings.patternSpacing ?? 18);

  ruler.style.setProperty("--border-thickness", `${settings.borderThickness}px`);
  ruler.style.setProperty("--border-color", borderColor);
  ruler.style.setProperty("--background-fill", hexToRgba(backgroundColor, opacity));
  ruler.style.setProperty("--stripe-fill-light", hexToRgba(backgroundColor, opacity * 0.42));
  ruler.style.setProperty("--stripe-fill-strong", hexToRgba(backgroundColor, opacity * 0.78));
  ruler.style.setProperty("--pattern-spacing", `${spacing}px`);
  ruler.dataset.pattern = settings.pattern || "striped";
  ruler.classList.toggle(
    "click-through-active",
    Boolean(settings.clickThrough && !settings.editMode),
  );
  ruler.classList.toggle("edit-mode", Boolean(settings.editMode));
  loadBackgroundImage(settings);
}

async function selectThisRuler() {
  try {
    await invoke("select_ruler", { rulerId });
  } catch (error) {
    console.error("Unable to select ruler", error);
  }
}

async function saveCurrentGeometry() {
  try {
    const scaleFactor = await appWindow.scaleFactor();
    const position = await appWindow.outerPosition();
    const size = await appWindow.innerSize();
    const x = Math.round(position.x / scaleFactor);
    const y = Math.round(position.y / scaleFactor);
    const width = Math.round(size.width / scaleFactor);
    const height = Math.round(size.height / scaleFactor);

    await invoke("save_ruler_geometry", { rulerId, x, y, width, height });
  } catch (error) {
    console.error("Unable to save overlay geometry", error);
  }
}

function scheduleGeometrySave() {
  window.clearTimeout(geometrySaveTimer);
  geometrySaveTimer = window.setTimeout(saveCurrentGeometry, 220);
}

ruler.addEventListener("pointerdown", async (event) => {
  if (event.button !== 0) {
    return;
  }
  await selectThisRuler();
  if (event.target.closest("[data-resize-direction]")) {
    return;
  }

  event.preventDefault();

  try {
    await appWindow.startDragging();
  } catch (error) {
    console.error("Unable to start overlay drag", error);
  }
});

resizeHandles.forEach((handle) => {
  handle.addEventListener("pointerdown", async (event) => {
    if (event.button !== 0) {
      return;
    }

    await selectThisRuler();
    event.preventDefault();
    event.stopPropagation();

    try {
      await appWindow.startResizeDragging(handle.dataset.resizeDirection);
    } catch (error) {
      console.error("Unable to start overlay resize", error);
    }
  });
});

appWindow.onMoved(() => {
  scheduleGeometrySave();
});

appWindow.onResized(() => {
  scheduleGeometrySave();
});

listen("settings-changed", (event) => {
  if (event.payload?.rulerId === rulerId && event.payload.settings) {
    applySettings(event.payload.settings);
  }
});

invoke("load_app_settings")
  .then((settings) => {
    const rulerSettings = settings.rulers.find((ruler) => ruler.id === rulerId);
    if (rulerSettings) {
      applySettings(rulerSettings);
    }
  })
  .catch((error) => console.error("Unable to load overlay settings", error));
