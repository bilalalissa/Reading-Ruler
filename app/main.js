const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { availableMonitors } = window.__TAURI__.window;

const stateEl = document.querySelector("#overlay-state");
const modeStateEl = document.querySelector("#mode-state");
const clickThroughStateEl = document.querySelector("#click-through-state");
const toggleButton = document.querySelector("#toggle-overlay");
const messageEl = document.querySelector("#message");
const shortcutEl = document.querySelector("#shortcut-label");
const shortcutInput = document.querySelector("#shortcut-input");
const saveShortcutButton = document.querySelector("#save-shortcut");
const resetButton = document.querySelector("#reset-settings");
const refreshTargetsButton = document.querySelector("#refresh-targets");
const importImageButton = document.querySelector("#import-image");
const pasteImageButton = document.querySelector("#paste-image");
const clearImageButton = document.querySelector("#clear-image");
const imageFileInput = document.querySelector("#image-file");
const imageStateEl = document.querySelector("#image-state");
const form = document.querySelector("#settings-form");

const rulerSelect = document.querySelector("#ruler-select");
const rulerNameInput = document.querySelector("#ruler-name");
const addRulerButton = document.querySelector("#add-ruler");
const duplicateRulerButton = document.querySelector("#duplicate-ruler");
const deleteRulerButton = document.querySelector("#delete-ruler");

const fields = {
  mode: document.querySelector("#mode"),
  monitorName: document.querySelector("#monitor-name"),
  targetWindowId: document.querySelector("#target-window-id"),
  borderThickness: document.querySelector("#border-thickness"),
  borderColor: document.querySelector("#border-color"),
  backgroundColor: document.querySelector("#background-color"),
  backgroundOpacity: document.querySelector("#background-opacity"),
  pattern: document.querySelector("#pattern"),
  patternSpacing: document.querySelector("#pattern-spacing"),
  width: document.querySelector("#width"),
  height: document.querySelector("#height"),
  x: document.querySelector("#x"),
  y: document.querySelector("#y"),
  clickThrough: document.querySelector("#click-through"),
  editMode: document.querySelector("#edit-mode"),
};

const geometryFields = new Set([fields.width, fields.height, fields.x, fields.y]);

let appSettings = null;
let currentRuler = null;
let saveTimer = null;
let targetPollTimer = null;
let isApplyingSettings = false;
let isSaving = false;
let monitors = [];

function activeElement() {
  return document.activeElement;
}

function activeRulerId() {
  return appSettings?.activeRulerId || currentRuler?.id || "";
}

function findRuler(rulerId = activeRulerId()) {
  return appSettings?.rulers?.find((ruler) => ruler.id === rulerId) || null;
}

function selectedTargetWindowId() {
  const value = fields.targetWindowId.value;
  return value ? Number(value) : null;
}

function setFieldValue(field, value, { respectFocus = true } = {}) {
  if (respectFocus && activeElement() === field) {
    return;
  }
  field.value = value ?? "";
}

function setCheckboxValue(field, value, { respectFocus = true } = {}) {
  if (respectFocus && activeElement() === field) {
    return;
  }
  field.checked = Boolean(value);
}

function numberFieldValue(field, fallback) {
  if (String(field.value).trim() === "") {
    return fallback;
  }
  const value = Number(field.value);
  return Number.isFinite(value) ? value : fallback;
}

function currentRulerFromFields() {
  return {
    ...currentRuler,
    id: activeRulerId(),
    name: rulerNameInput.value || currentRuler?.name || "Ruler",
    mode: fields.mode.value,
    borderThickness: numberFieldValue(fields.borderThickness, currentRuler?.borderThickness),
    borderColor: fields.borderColor.value,
    backgroundColor: fields.backgroundColor.value,
    backgroundOpacity: numberFieldValue(fields.backgroundOpacity, currentRuler?.backgroundOpacity),
    pattern: fields.pattern.value,
    patternSpacing: numberFieldValue(fields.patternSpacing, currentRuler?.patternSpacing),
    width: numberFieldValue(fields.width, currentRuler?.width),
    height: numberFieldValue(fields.height, currentRuler?.height),
    x: numberFieldValue(fields.x, currentRuler?.x),
    y: numberFieldValue(fields.y, currentRuler?.y),
    targetOffsetX: currentRuler?.targetOffsetX || 0,
    targetOffsetY: currentRuler?.targetOffsetY || 0,
    clickThrough: fields.clickThrough.checked,
    editMode: fields.editMode.checked,
    monitorName: fields.monitorName.value || null,
    targetWindowId: selectedTargetWindowId(),
    backgroundImagePath: currentRuler?.backgroundImagePath || null,
  };
}

function updateFacts(ruler) {
  modeStateEl.textContent =
    ruler.mode === "targeted" ? "Target app/window" : "Whole screen";
  clickThroughStateEl.textContent = ruler.clickThrough
    ? ruler.editMode
      ? "On, edit mode active"
      : "On"
    : "Off";
  shortcutEl.textContent = appSettings?.shortcut || "Control+Alt+R";
  imageStateEl.textContent = ruler.backgroundImagePath
    ? `Image: ${ruler.backgroundImagePath}`
    : "No image selected.";
  stateEl.textContent = `${ruler.name} ${ruler.visible ? "visible" : "hidden"}`;
  toggleButton.textContent = ruler.visible ? "Hide ruler" : "Show ruler";
}

function renderRulerSelector() {
  const selected = activeRulerId();
  rulerSelect.innerHTML = "";
  (appSettings?.rulers || []).forEach((ruler) => {
    const option = document.createElement("option");
    option.value = ruler.id;
    option.textContent = `${ruler.visible ? "[shown]" : "[hidden]"} ${ruler.name}`;
    rulerSelect.append(option);
  });
  rulerSelect.value = selected;
  deleteRulerButton.disabled = (appSettings?.rulers?.length || 0) <= 1;
}

function applyRulerToFields(ruler, options = {}) {
  const { syncPolling = true, respectFocus = true, geometryOnly = false } = options;
  currentRuler = ruler;
  isApplyingSettings = true;

  if (!geometryOnly) {
    setFieldValue(rulerNameInput, ruler.name, { respectFocus });
    setFieldValue(fields.mode, ruler.mode || "wholeScreen", { respectFocus });
    setFieldValue(fields.borderThickness, Math.round(ruler.borderThickness), { respectFocus });
    setFieldValue(fields.borderColor, ruler.borderColor, { respectFocus });
    setFieldValue(fields.backgroundColor, ruler.backgroundColor, { respectFocus });
    setFieldValue(fields.backgroundOpacity, ruler.backgroundOpacity, { respectFocus });
    setFieldValue(fields.pattern, ruler.pattern || "striped", { respectFocus });
    setFieldValue(fields.patternSpacing, Math.round(ruler.patternSpacing || 18), { respectFocus });
    setCheckboxValue(fields.clickThrough, ruler.clickThrough, { respectFocus });
    setCheckboxValue(fields.editMode, ruler.editMode, { respectFocus });
    setFieldValue(fields.monitorName, ruler.monitorName || "", { respectFocus });
    setFieldValue(fields.targetWindowId, ruler.targetWindowId || "", { respectFocus });
    if (!respectFocus || activeElement() !== shortcutInput) {
      shortcutInput.value = appSettings?.shortcut || "Control+Alt+R";
    }
  }

  setFieldValue(fields.width, Math.round(ruler.width), { respectFocus });
  setFieldValue(fields.height, Math.round(ruler.height), { respectFocus });
  setFieldValue(fields.x, Math.round(ruler.x), { respectFocus });
  setFieldValue(fields.y, Math.round(ruler.y), { respectFocus });
  updateFacts(ruler);

  isApplyingSettings = false;
  if (syncPolling) {
    syncTargetPolling();
  }
}

function applyAppSettings(settings, options = {}) {
  appSettings = settings;
  renderRulerSelector();
  const ruler = findRuler();
  if (ruler) {
    applyRulerToFields(ruler, options);
  }
}

async function saveRuler({ syncPolling = false, respectFocus = true } = {}) {
  if (isApplyingSettings || isSaving || !currentRuler) {
    return;
  }

  isSaving = true;
  try {
    const settings = currentRulerFromFields();
    appSettings = await invoke("save_ruler_settings", {
      rulerId: settings.id,
      settings,
    });
    applyAppSettings(appSettings, { syncPolling, respectFocus });
    messageEl.textContent = "Ruler saved.";
  } catch (error) {
    messageEl.textContent = String(error);
  } finally {
    isSaving = false;
  }
}

function scheduleSave() {
  if (isApplyingSettings) {
    return;
  }

  window.clearTimeout(saveTimer);
  saveTimer = window.setTimeout(() => {
    saveRuler({ respectFocus: true });
  }, 220);
}

async function loadMonitors() {
  try {
    monitors = await availableMonitors();
    const savedValue = fields.monitorName.value;
    fields.monitorName.innerHTML = '<option value="">Saved position</option>';
    monitors.forEach((monitor, index) => {
      const option = document.createElement("option");
      option.value = monitor.name || `Display ${index + 1}`;
      option.textContent = monitor.name || `Display ${index + 1}`;
      fields.monitorName.append(option);
    });
    fields.monitorName.value = savedValue;
  } catch (error) {
    messageEl.textContent = `Could not read monitors: ${error}`;
  }
}

function applySelectedMonitorGeometry() {
  const selected = fields.monitorName.value;
  if (!selected) {
    return;
  }

  const monitor = monitors.find(
    (item, index) => (item.name || `Display ${index + 1}`) === selected,
  );
  if (!monitor) {
    return;
  }

  fields.x.value = Math.round(monitor.workArea.position.x / monitor.scaleFactor);
  fields.y.value = Math.round(
    monitor.workArea.position.y / monitor.scaleFactor +
      (monitor.workArea.size.height / monitor.scaleFactor - Number(fields.height.value)) / 2,
  );
  fields.width.value = Math.round(monitor.workArea.size.width / monitor.scaleFactor);
}

async function refreshTargetWindows() {
  try {
    const selected = fields.targetWindowId.value;
    const targets = await invoke("list_target_windows");
    fields.targetWindowId.innerHTML = '<option value="">Select a target window</option>';
    targets.forEach((target) => {
      const option = document.createElement("option");
      option.value = String(target.id);
      const title = String(target.title || "").trim() || "Untitled window";
      option.textContent = `${target.ownerName}: ${title}`;
      option.title = `${target.ownerName}: ${title}`;
      fields.targetWindowId.append(option);
    });
    fields.targetWindowId.value = selected;
  } catch (error) {
    messageEl.textContent = `Could not list target windows: ${error}`;
  }
}

async function pollTargetWindow() {
  if (
    !currentRuler ||
    !currentRuler.visible ||
    fields.mode.value !== "targeted" ||
    !selectedTargetWindowId() ||
    isSaving
  ) {
    return;
  }

  try {
    const status = await invoke("track_target_window", {
      rulerId: activeRulerId(),
      windowId: selectedTargetWindowId(),
    });
    if (status.settings) {
      currentRuler = status.settings;
      applyRulerToFields(status.settings, {
        syncPolling: false,
        respectFocus: true,
        geometryOnly: geometryFields.has(activeElement()),
      });
    }
    messageEl.textContent = status.message;
  } catch (error) {
    messageEl.textContent = String(error);
  }
}

function syncTargetPolling() {
  window.clearInterval(targetPollTimer);
  targetPollTimer = null;
  if (currentRuler?.visible && fields.mode.value === "targeted" && selectedTargetWindowId()) {
    targetPollTimer = window.setInterval(pollTargetWindow, 1000);
    pollTargetWindow();
  }
}

async function loadSettings() {
  try {
    appSettings = await invoke("load_app_settings");
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: false });
    const path = await invoke("settings_path");
    messageEl.textContent = `Settings file: ${path}`;
  } catch (error) {
    messageEl.textContent = String(error);
  }
}

function readFileAsDataUrl(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.addEventListener("load", () => resolve(reader.result));
    reader.addEventListener("error", () => reject(reader.error));
    reader.readAsDataURL(file);
  });
}

async function saveBackgroundImage(dataUrl, sourceLabel) {
  try {
    appSettings = await invoke("save_background_image", {
      rulerId: activeRulerId(),
      dataUrl,
      sourceLabel,
    });
    applyAppSettings(appSettings, { syncPolling: false, respectFocus: false });
    messageEl.textContent = "Background image saved.";
  } catch (error) {
    messageEl.textContent = String(error);
  }
}

async function pasteImageFromClipboard() {
  try {
    if (navigator.clipboard?.read) {
      const items = await navigator.clipboard.read();
      for (const item of items) {
        const imageType = item.types.find((type) => type.startsWith("image/"));
        if (imageType) {
          const blob = await item.getType(imageType);
          const dataUrl = await readFileAsDataUrl(blob);
          await saveBackgroundImage(dataUrl, "clipboard");
          return;
        }
      }
    }
    messageEl.textContent = "No image found in clipboard. You can also press Command+V here.";
  } catch (error) {
    messageEl.textContent = `Clipboard image read failed. Press Command+V here or import a file. ${error}`;
  }
}

async function handlePasteEvent(event) {
  const item = Array.from(event.clipboardData?.items || []).find((clipboardItem) =>
    clipboardItem.type.startsWith("image/"),
  );
  if (!item) {
    return;
  }

  const file = item.getAsFile();
  if (!file) {
    return;
  }

  event.preventDefault();
  const dataUrl = await readFileAsDataUrl(file);
  await saveBackgroundImage(dataUrl, "clipboard");
}

toggleButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("toggle_ruler", { rulerId: activeRulerId() });
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: true });
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

rulerSelect.addEventListener("change", async () => {
  const selectedRulerId = rulerSelect.value;
  window.clearTimeout(saveTimer);
  await saveRuler({ syncPolling: false, respectFocus: true });
  try {
    appSettings = await invoke("select_ruler", { rulerId: selectedRulerId });
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: false });
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

rulerNameInput.addEventListener("change", async () => {
  try {
    appSettings = await invoke("rename_ruler", {
      rulerId: activeRulerId(),
      name: rulerNameInput.value,
    });
    applyAppSettings(appSettings, { syncPolling: false, respectFocus: false });
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

addRulerButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("create_ruler");
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: false });
    messageEl.textContent = "Ruler added.";
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

duplicateRulerButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("duplicate_ruler", { rulerId: activeRulerId() });
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: false });
    messageEl.textContent = "Ruler duplicated.";
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

deleteRulerButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("delete_ruler", { rulerId: activeRulerId() });
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: false });
    messageEl.textContent = "Ruler deleted.";
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

form.addEventListener("input", scheduleSave);
form.addEventListener("change", async (event) => {
  if (event.target === fields.monitorName) {
    applySelectedMonitorGeometry();
  }
  await saveRuler({ syncPolling: true, respectFocus: true });
});

refreshTargetsButton.addEventListener("click", refreshTargetWindows);

saveShortcutButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("set_shortcut", {
      shortcut: shortcutInput.value,
    });
    applyAppSettings(appSettings, { syncPolling: false, respectFocus: false });
    messageEl.textContent = "Shortcut saved.";
  } catch (error) {
    messageEl.textContent = String(error);
    shortcutInput.value = appSettings?.shortcut || "Control+Alt+R";
  }
});

resetButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("reset_active_ruler");
    applyAppSettings(appSettings, { syncPolling: true, respectFocus: false });
    messageEl.textContent = "Active ruler reset.";
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

importImageButton.addEventListener("click", () => {
  imageFileInput.click();
});

imageFileInput.addEventListener("change", async () => {
  const file = imageFileInput.files?.[0];
  if (!file) {
    return;
  }
  const dataUrl = await readFileAsDataUrl(file);
  await saveBackgroundImage(dataUrl, file.name || "imported-image");
  imageFileInput.value = "";
});

pasteImageButton.addEventListener("click", pasteImageFromClipboard);

clearImageButton.addEventListener("click", async () => {
  try {
    appSettings = await invoke("clear_background_image", { rulerId: activeRulerId() });
    applyAppSettings(appSettings, { syncPolling: false, respectFocus: false });
    messageEl.textContent = "Background image cleared.";
  } catch (error) {
    messageEl.textContent = String(error);
  }
});

window.addEventListener("focus", () => {
  refreshTargetWindows();
});

window.addEventListener("paste", handlePasteEvent);

listen("app-settings-changed", (event) => {
  if (!event.payload || isSaving) {
    return;
  }
  const previousActiveRulerId = activeRulerId();
  const nextActiveRulerId = event.payload.activeRulerId;
  const previousRuler = findRuler(nextActiveRulerId);
  const nextRuler = event.payload.rulers?.find((ruler) => ruler.id === nextActiveRulerId);
  const shouldSyncPolling =
    previousActiveRulerId !== nextActiveRulerId ||
    previousRuler?.visible !== nextRuler?.visible ||
    previousRuler?.mode !== nextRuler?.mode ||
    previousRuler?.targetWindowId !== nextRuler?.targetWindowId;
  applyAppSettings(event.payload, {
    syncPolling: shouldSyncPolling,
    respectFocus: true,
  });
});

listen("settings-changed", (event) => {
  if (!event.payload || isSaving) {
    return;
  }
  const { rulerId, settings } = event.payload;
  if (rulerId !== activeRulerId()) {
    return;
  }
  currentRuler = settings;
  applyRulerToFields(settings, {
    syncPolling: false,
    respectFocus: true,
    geometryOnly: geometryFields.has(activeElement()),
  });
});

async function init() {
  await loadMonitors();
  await refreshTargetWindows();
  await loadSettings();
}

init();
