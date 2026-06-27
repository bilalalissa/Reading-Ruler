use core_foundation::{
    array::CFArray,
    base::{CFType, CFTypeRef, TCFType},
    dictionary::CFDictionary,
    number::CFNumber,
    string::{CFString, CFStringRef},
};
use core_graphics::{
    geometry::{CGPoint, CGRect, CGSize},
    window::{
        kCGNullWindowID, kCGWindowBounds, kCGWindowLayer, kCGWindowListExcludeDesktopElements,
        kCGWindowListOptionOnScreenOnly, kCGWindowName, kCGWindowNumber, kCGWindowOwnerName,
        kCGWindowOwnerPID, CGWindowListCopyWindowInfo,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    ffi::c_void,
    fs,
    path::{Path, PathBuf},
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{
    menu::{MenuBuilder, SubmenuBuilder},
    App, AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, WebviewUrl,
    WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const MAIN_LABEL: &str = "main";
const OVERLAY_PREFIX: &str = "overlay-";
const DEFAULT_SHORTCUT_LABEL: &str = "Control+Alt+R";
const SETTINGS_FILE_NAME: &str = "settings.json";
const MAX_BACKGROUND_IMAGE_BYTES: usize = 10 * 1024 * 1024;
const MENU_SHOW_PANEL: &str = "show_control_panel";
const MENU_SHOW_RULER: &str = "show_ruler";
const MENU_HIDE_RULER: &str = "hide_ruler";
const MENU_TOGGLE_RULER: &str = "toggle_ruler";
const MENU_RESET_SETTINGS: &str = "reset_settings";
const MENU_SHOW_HELP: &str = "show_help";

type AXUIElementRef = *const c_void;

const AX_ERROR_SUCCESS: i32 = 0;
const AX_VALUE_CGPOINT_TYPE: i32 = 1;
const AX_VALUE_CGSIZE_TYPE: i32 = 2;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
    fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> i32;
    fn AXValueGetValue(value: CFTypeRef, value_type: i32, out_value: *mut c_void) -> bool;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
struct AppSettings {
    active_ruler_id: String,
    shortcut: String,
    rulers: Vec<RulerSettings>,
}

impl Default for AppSettings {
    fn default() -> Self {
        let ruler = RulerSettings::default();
        Self {
            active_ruler_id: ruler.id.clone(),
            shortcut: DEFAULT_SHORTCUT_LABEL.to_string(),
            rulers: vec![ruler],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
struct RulerSettings {
    id: String,
    name: String,
    visible: bool,
    mode: String,
    border_thickness: f64,
    border_color: String,
    background_color: String,
    background_opacity: f64,
    pattern: String,
    pattern_spacing: f64,
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    target_offset_x: f64,
    target_offset_y: f64,
    click_through: bool,
    edit_mode: bool,
    monitor_name: Option<String>,
    target_window_id: Option<u32>,
    background_image_path: Option<String>,
}

impl Default for RulerSettings {
    fn default() -> Self {
        Self {
            id: "ruler-1".to_string(),
            name: "Ruler 1".to_string(),
            visible: true,
            mode: "wholeScreen".to_string(),
            border_thickness: 3.0,
            border_color: "#006858".to_string(),
            background_color: "#ffef75".to_string(),
            background_opacity: 0.46,
            pattern: "striped".to_string(),
            pattern_spacing: 18.0,
            width: 760.0,
            height: 86.0,
            x: 220.0,
            y: 260.0,
            target_offset_x: 0.0,
            target_offset_y: 0.0,
            click_through: false,
            edit_mode: true,
            monitor_name: None,
            target_window_id: None,
            background_image_path: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RulerEvent {
    ruler_id: String,
    settings: RulerSettings,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TargetWindow {
    id: u32,
    owner_pid: i32,
    owner_name: String,
    title: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TargetTrackStatus {
    state: String,
    message: String,
    settings: Option<RulerSettings>,
}

#[tauri::command]
fn load_app_settings(app: AppHandle) -> Result<AppSettings, String> {
    load_or_create_app_settings(&app)
}

#[tauri::command]
fn select_ruler(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    ensure_ruler_exists(&settings, &ruler_id)?;
    settings.active_ruler_id = ruler_id;
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn create_ruler(app: AppHandle) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let mut ruler = active_ruler(&settings)?.clone();
    ruler.id = next_ruler_id(&settings);
    ruler.name = next_ruler_name(&settings);
    ruler.x = (ruler.x + 32.0).clamp(-10000.0, 10000.0);
    ruler.y = (ruler.y + 32.0).clamp(-10000.0, 10000.0);
    ruler.visible = true;
    settings.active_ruler_id = ruler.id.clone();
    settings.rulers.push(ruler);
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    sync_overlay_windows(&app, &settings)?;
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn duplicate_ruler(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let mut ruler = find_ruler(&settings, &ruler_id)?.clone();
    ruler.id = next_ruler_id(&settings);
    ruler.name = next_ruler_name(&settings);
    ruler.x = (ruler.x + 32.0).clamp(-10000.0, 10000.0);
    ruler.y = (ruler.y + 32.0).clamp(-10000.0, 10000.0);
    ruler.visible = true;
    settings.active_ruler_id = ruler.id.clone();
    settings.rulers.push(ruler);
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    sync_overlay_windows(&app, &settings)?;
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn delete_ruler(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    if settings.rulers.len() <= 1 {
        return Err("At least one ruler must remain.".to_string());
    }
    let label = overlay_label(&ruler_id);
    settings.rulers.retain(|ruler| ruler.id != ruler_id);
    if settings.active_ruler_id == ruler_id {
        settings.active_ruler_id = settings
            .rulers
            .first()
            .map(|ruler| ruler.id.clone())
            .unwrap_or_else(|| "ruler-1".to_string());
    }
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    if let Some(window) = app.get_webview_window(&label) {
        let _ = window.close();
    }
    sync_overlay_windows(&app, &settings)?;
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn rename_ruler(app: AppHandle, ruler_id: String, name: String) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let ruler = find_ruler_mut(&mut settings, &ruler_id)?;
    let name = name.trim();
    ruler.name = if name.is_empty() {
        ruler.id.clone()
    } else {
        name.chars().take(40).collect()
    };
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn save_ruler_settings(
    app: AppHandle,
    ruler_id: String,
    settings: RulerSettings,
) -> Result<AppSettings, String> {
    let mut app_settings = load_or_create_app_settings(&app)?;
    let previous = find_ruler(&app_settings, &ruler_id)?.clone();
    let mut ruler = settings;
    ruler.id = ruler_id.clone();
    ruler.name = if ruler.name.trim().is_empty() {
        previous.name.clone()
    } else {
        ruler.name
    };
    ruler.visible = previous.visible;
    prepare_targeted_ruler_for_save(&mut ruler, &previous);
    *find_ruler_mut(&mut app_settings, &ruler_id)? = ruler.normalized();
    let app_settings = app_settings.normalized();
    write_app_settings(&app, &app_settings)?;
    sync_one_overlay(&app, find_ruler(&app_settings, &ruler_id)?)?;
    emit_ruler_settings(&app, find_ruler(&app_settings, &ruler_id)?);
    emit_app_settings(&app, &app_settings);
    Ok(app_settings)
}

#[tauri::command]
fn save_ruler_geometry(
    app: AppHandle,
    ruler_id: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<AppSettings, String> {
    let mut app_settings = load_or_create_app_settings(&app)?;
    let previous = find_ruler(&app_settings, &ruler_id)?.clone();
    let ruler = find_ruler_mut(&mut app_settings, &ruler_id)?;
    ruler.x = x;
    ruler.y = y;
    ruler.width = width;
    ruler.height = height;
    prepare_targeted_ruler_for_save(ruler, &previous);
    *ruler = ruler.clone().normalized();
    app_settings.active_ruler_id = ruler_id.clone();
    let app_settings = app_settings.normalized();
    write_app_settings(&app, &app_settings)?;
    emit_ruler_settings(&app, find_ruler(&app_settings, &ruler_id)?);
    emit_app_settings(&app, &app_settings);
    Ok(app_settings)
}

#[tauri::command]
fn show_ruler(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    set_ruler_visibility(&app, &ruler_id, true)
}

#[tauri::command]
fn hide_ruler(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    set_ruler_visibility(&app, &ruler_id, false)
}

#[tauri::command]
fn toggle_ruler(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    let settings = load_or_create_app_settings(&app)?;
    let visible = find_ruler(&settings, &ruler_id)?.visible;
    set_ruler_visibility(&app, &ruler_id, !visible)
}

#[tauri::command]
fn active_ruler_visible(app: AppHandle) -> Result<bool, String> {
    let settings = load_or_create_app_settings(&app)?;
    Ok(active_ruler(&settings)?.visible)
}

#[tauri::command]
fn default_shortcut_label() -> &'static str {
    DEFAULT_SHORTCUT_LABEL
}

#[tauri::command]
fn set_shortcut(app: AppHandle, shortcut: String) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let previous_shortcut = settings.shortcut.clone();
    let shortcut = shortcut.trim().to_string();

    app.global_shortcut()
        .unregister_all()
        .map_err(|error| error.to_string())?;

    if let Err(error) = register_shortcut(&app, &shortcut) {
        let _ = register_shortcut(&app, &previous_shortcut);
        return Err(format!(
            "Could not register shortcut '{shortcut}'. It may conflict with another shortcut. {error}"
        ));
    }

    settings.shortcut = shortcut;
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn reset_active_ruler(app: AppHandle) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let previous = active_ruler(&settings)?.clone();
    let mut ruler = RulerSettings::default();
    ruler.id = previous.id.clone();
    ruler.name = previous.name.clone();
    ruler.visible = previous.visible;
    *find_ruler_mut(&mut settings, &previous.id)? = ruler;
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    sync_overlay_windows(&app, &settings)?;
    emit_ruler_settings(&app, find_ruler(&settings, &previous.id)?);
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn show_control_panel(app: AppHandle) -> Result<(), String> {
    show_main_window(&app)
}

#[tauri::command]
fn list_target_windows() -> Vec<TargetWindow> {
    target_windows()
}

#[tauri::command]
fn save_background_image(
    app: AppHandle,
    ruler_id: String,
    data_url: String,
    source_label: String,
) -> Result<AppSettings, String> {
    let (extension, bytes) = decode_image_data_url(&data_url)?;
    if bytes.len() > MAX_BACKGROUND_IMAGE_BYTES {
        return Err("Background image is too large. Use an image under 10 MB.".to_string());
    }

    let relative_path = store_background_image(&app, &extension, &source_label, &bytes)?;
    let mut settings = load_or_create_app_settings(&app)?;
    let ruler = find_ruler_mut(&mut settings, &ruler_id)?;
    ruler.background_image_path = Some(relative_path);
    ruler.pattern = "image".to_string();
    *ruler = ruler.clone().normalized();
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    sync_one_overlay(&app, find_ruler(&settings, &ruler_id)?)?;
    emit_ruler_settings(&app, find_ruler(&settings, &ruler_id)?);
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn clear_background_image(app: AppHandle, ruler_id: String) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let ruler = find_ruler_mut(&mut settings, &ruler_id)?;
    ruler.background_image_path = None;
    if ruler.pattern == "image" {
        ruler.pattern = "solid".to_string();
    }
    *ruler = ruler.clone().normalized();
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    sync_one_overlay(&app, find_ruler(&settings, &ruler_id)?)?;
    emit_ruler_settings(&app, find_ruler(&settings, &ruler_id)?);
    emit_app_settings(&app, &settings);
    Ok(settings)
}

#[tauri::command]
fn background_image_data_url(app: AppHandle, ruler_id: String) -> Result<Option<String>, String> {
    let settings = load_or_create_app_settings(&app)?;
    let ruler = find_ruler(&settings, &ruler_id)?;
    let Some(relative_path) = ruler.background_image_path.clone() else {
        return Ok(None);
    };

    let image_path = config_relative_path(&app, &relative_path)?;
    let bytes = fs::read(&image_path).map_err(|error| {
        format!(
            "Could not read background image at {}: {error}",
            image_path.display()
        )
    })?;
    let mime = mime_from_path(&image_path).ok_or_else(|| {
        format!(
            "Unsupported background image extension at {}",
            image_path.display()
        )
    })?;

    Ok(Some(format!(
        "data:{mime};base64,{}",
        base64_encode(&bytes)
    )))
}

#[tauri::command]
fn track_target_window(
    app: AppHandle,
    ruler_id: String,
    window_id: u32,
) -> Result<TargetTrackStatus, String> {
    let mut settings = load_or_create_app_settings(&app)?;
    let Some(target) = find_target_window(window_id) else {
        hide_overlay_window(&app, &ruler_id)?;
        if let Ok(ruler) = find_ruler_mut(&mut settings, &ruler_id) {
            ruler.visible = false;
            let settings = settings.normalized();
            let _ = write_app_settings(&app, &settings);
            if let Ok(ruler) = find_ruler(&settings, &ruler_id) {
                emit_ruler_settings(&app, ruler);
            }
            emit_app_settings(&app, &settings);
        }
        return Ok(TargetTrackStatus {
            state: "unavailable".to_string(),
            message: "Target window is unavailable, minimized, closed, or on another Space."
                .to_string(),
            settings: None,
        });
    };

    if let Some(frontmost_window) = frontmost_target_window() {
        if frontmost_window.id != target.id {
            hide_overlay_window(&app, &ruler_id)?;
            if let Ok(ruler) = find_ruler_mut(&mut settings, &ruler_id) {
                ruler.visible = false;
                let settings = settings.normalized();
                let _ = write_app_settings(&app, &settings);
                if let Ok(ruler) = find_ruler(&settings, &ruler_id) {
                    emit_ruler_settings(&app, ruler);
                }
                emit_app_settings(&app, &settings);
            }
            let message = if frontmost_window.owner_pid == target.owner_pid {
                format!(
                    "Selected {} window is not frontmost; overlay hidden until that exact window is active again.",
                    target.owner_name
                )
            } else {
                "Target app/window is not frontmost; overlay hidden until it is active again."
                    .to_string()
            };
            return Ok(TargetTrackStatus {
                state: "inactive".to_string(),
                message,
                settings: None,
            });
        }
    }

    let ruler = find_ruler_mut(&mut settings, &ruler_id)?;
    ruler.mode = "targeted".to_string();
    ruler.target_window_id = Some(target.id);
    ruler.x = target.x + ruler.target_offset_x;
    ruler.y = target.y + ruler.target_offset_y;
    ruler.visible = true;
    *ruler = ruler.clone().normalized();
    settings.active_ruler_id = ruler_id.clone();
    let settings = settings.normalized();
    write_app_settings(&app, &settings)?;
    let ruler = find_ruler(&settings, &ruler_id)?.clone();
    sync_one_overlay(&app, &ruler)?;
    emit_ruler_settings(&app, &ruler);
    emit_app_settings(&app, &settings);

    Ok(TargetTrackStatus {
        state: "tracking".to_string(),
        message: format!("Tracking {} - {}", target.owner_name, target.title),
        settings: Some(ruler),
    })
}

#[tauri::command]
fn settings_path(app: AppHandle) -> Result<String, String> {
    settings_file_path(&app).map(|path| path.display().to_string())
}

impl AppSettings {
    fn normalized(mut self) -> Self {
        if self.rulers.is_empty() {
            self.rulers.push(RulerSettings::default());
        }
        self.rulers = self
            .rulers
            .into_iter()
            .enumerate()
            .map(|(index, ruler)| ruler.with_fallback_identity(index).normalized())
            .collect();
        if !self
            .rulers
            .iter()
            .any(|ruler| ruler.id == self.active_ruler_id)
        {
            self.active_ruler_id = self.rulers[0].id.clone();
        }
        if self.shortcut.trim().is_empty() || self.shortcut == "Control + Option + R" {
            self.shortcut = DEFAULT_SHORTCUT_LABEL.to_string();
        }
        self
    }
}

impl RulerSettings {
    fn with_fallback_identity(mut self, index: usize) -> Self {
        if self.id.trim().is_empty() {
            self.id = format!("ruler-{}", index + 1);
        }
        if self.name.trim().is_empty() {
            self.name = format!("Ruler {}", index + 1);
        }
        self
    }

    fn normalized(mut self) -> Self {
        if !matches!(self.mode.as_str(), "wholeScreen" | "targeted") {
            self.mode = "wholeScreen".to_string();
        }
        self.border_thickness = self.border_thickness.clamp(0.0, 24.0);
        self.background_opacity = self.background_opacity.clamp(0.0, 1.0);
        if !matches!(
            self.pattern.as_str(),
            "solid"
                | "dotted"
                | "striped"
                | "grid"
                | "transparent"
                | "edgesAll"
                | "edgesTopBottom"
                | "edgesLeftRight"
                | "edgeTop"
                | "edgeBottom"
                | "edgeLeft"
                | "edgeRight"
                | "image"
        ) {
            self.pattern = "striped".to_string();
        }
        self.pattern_spacing = self.pattern_spacing.clamp(4.0, 96.0);
        self.width = self.width.clamp(120.0, 2400.0);
        self.height = self.height.clamp(32.0, 900.0);
        self.x = self.x.clamp(-10000.0, 10000.0);
        self.y = self.y.clamp(-10000.0, 10000.0);
        self.target_offset_x = self.target_offset_x.clamp(-10000.0, 10000.0);
        self.target_offset_y = self.target_offset_y.clamp(-10000.0, 10000.0);
        if self.pattern == "image" && self.background_image_path.is_none() {
            self.pattern = "solid".to_string();
        }
        self
    }
}

fn active_ruler(settings: &AppSettings) -> Result<&RulerSettings, String> {
    find_ruler(settings, &settings.active_ruler_id)
}

fn find_ruler<'a>(settings: &'a AppSettings, ruler_id: &str) -> Result<&'a RulerSettings, String> {
    settings
        .rulers
        .iter()
        .find(|ruler| ruler.id == ruler_id)
        .ok_or_else(|| format!("Ruler '{ruler_id}' is not available."))
}

fn find_ruler_mut<'a>(
    settings: &'a mut AppSettings,
    ruler_id: &str,
) -> Result<&'a mut RulerSettings, String> {
    settings
        .rulers
        .iter_mut()
        .find(|ruler| ruler.id == ruler_id)
        .ok_or_else(|| format!("Ruler '{ruler_id}' is not available."))
}

fn ensure_ruler_exists(settings: &AppSettings, ruler_id: &str) -> Result<(), String> {
    find_ruler(settings, ruler_id).map(|_| ())
}

fn next_ruler_id(settings: &AppSettings) -> String {
    let mut index = settings.rulers.len() + 1;
    loop {
        let id = format!("ruler-{index}");
        if !settings.rulers.iter().any(|ruler| ruler.id == id) {
            return id;
        }
        index += 1;
    }
}

fn next_ruler_name(settings: &AppSettings) -> String {
    let mut index = settings.rulers.len() + 1;
    loop {
        let name = format!("Ruler {index}");
        if !settings.rulers.iter().any(|ruler| ruler.name == name) {
            return name;
        }
        index += 1;
    }
}

fn overlay_label(ruler_id: &str) -> String {
    format!("{OVERLAY_PREFIX}{ruler_id}")
}

fn set_ruler_visibility(
    app: &AppHandle,
    ruler_id: &str,
    visible: bool,
) -> Result<AppSettings, String> {
    let mut settings = load_or_create_app_settings(app)?;
    let ruler = find_ruler_mut(&mut settings, ruler_id)?;
    ruler.visible = visible;
    settings.active_ruler_id = ruler_id.to_string();
    let settings = settings.normalized();
    write_app_settings(app, &settings)?;
    sync_one_overlay(app, find_ruler(&settings, ruler_id)?)?;
    emit_ruler_settings(app, find_ruler(&settings, ruler_id)?);
    emit_app_settings(app, &settings);
    Ok(settings)
}

fn prepare_targeted_ruler_for_save(ruler: &mut RulerSettings, previous: &RulerSettings) {
    if ruler.mode != "targeted" {
        return;
    }

    let Some(window_id) = ruler.target_window_id else {
        return;
    };
    let Some(target) = find_target_window(window_id) else {
        return;
    };

    if previous.mode != "targeted" || previous.target_window_id != Some(window_id) {
        ruler.x = target.x;
        ruler.y = target.y + ((target.height - ruler.height).max(0.0) / 2.0);
    }

    ruler.target_offset_x = ruler.x - target.x;
    ruler.target_offset_y = ruler.y - target.y;
}

fn load_or_create_app_settings(app: &AppHandle) -> Result<AppSettings, String> {
    let path = settings_file_path(app)?;

    if path.exists() {
        let file = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let value: Value = serde_json::from_str(&file).map_err(|error| {
            format!(
                "Could not read saved settings at {}: {error}",
                path.display()
            )
        })?;
        let settings = if value.get("rulers").is_some() {
            serde_json::from_value::<AppSettings>(value).map_err(|error| error.to_string())?
        } else {
            migrate_flat_settings(value)?
        };
        let settings = settings.normalized();
        write_app_settings(app, &settings)?;
        Ok(settings)
    } else {
        let settings = AppSettings::default().normalized();
        write_app_settings(app, &settings)?;
        Ok(settings)
    }
}

fn migrate_flat_settings(value: Value) -> Result<AppSettings, String> {
    #[derive(Deserialize)]
    #[serde(default)]
    #[serde(rename_all = "camelCase")]
    struct FlatSettings {
        mode: String,
        border_thickness: f64,
        border_color: String,
        background_color: String,
        background_opacity: f64,
        pattern: String,
        pattern_spacing: f64,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
        target_offset_x: f64,
        target_offset_y: f64,
        shortcut: String,
        click_through: bool,
        edit_mode: bool,
        monitor_name: Option<String>,
        target_window_id: Option<u32>,
        background_image_path: Option<String>,
    }

    impl Default for FlatSettings {
        fn default() -> Self {
            let ruler = RulerSettings::default();
            Self {
                mode: ruler.mode,
                border_thickness: ruler.border_thickness,
                border_color: ruler.border_color,
                background_color: ruler.background_color,
                background_opacity: ruler.background_opacity,
                pattern: ruler.pattern,
                pattern_spacing: ruler.pattern_spacing,
                width: ruler.width,
                height: ruler.height,
                x: ruler.x,
                y: ruler.y,
                target_offset_x: ruler.target_offset_x,
                target_offset_y: ruler.target_offset_y,
                shortcut: DEFAULT_SHORTCUT_LABEL.to_string(),
                click_through: ruler.click_through,
                edit_mode: ruler.edit_mode,
                monitor_name: ruler.monitor_name,
                target_window_id: ruler.target_window_id,
                background_image_path: ruler.background_image_path,
            }
        }
    }

    let flat = serde_json::from_value::<FlatSettings>(value).map_err(|error| error.to_string())?;
    let mut ruler = RulerSettings::default();
    ruler.mode = flat.mode;
    ruler.border_thickness = flat.border_thickness;
    ruler.border_color = flat.border_color;
    ruler.background_color = flat.background_color;
    ruler.background_opacity = flat.background_opacity;
    ruler.pattern = flat.pattern;
    ruler.pattern_spacing = flat.pattern_spacing;
    ruler.width = flat.width;
    ruler.height = flat.height;
    ruler.x = flat.x;
    ruler.y = flat.y;
    ruler.target_offset_x = flat.target_offset_x;
    ruler.target_offset_y = flat.target_offset_y;
    ruler.click_through = flat.click_through;
    ruler.edit_mode = flat.edit_mode;
    ruler.monitor_name = flat.monitor_name;
    ruler.target_window_id = flat.target_window_id;
    ruler.background_image_path = flat.background_image_path;

    Ok(AppSettings {
        active_ruler_id: ruler.id.clone(),
        shortcut: if flat.shortcut.trim().is_empty() {
            DEFAULT_SHORTCUT_LABEL.to_string()
        } else {
            flat.shortcut
        },
        rulers: vec![ruler],
    })
}

fn write_app_settings(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = settings_file_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let contents = serde_json::to_string_pretty(settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn backgrounds_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|directory| directory.join("backgrounds"))
        .map_err(|error| error.to_string())
}

fn config_relative_path(app: &AppHandle, relative_path: &str) -> Result<PathBuf, String> {
    if relative_path.contains("..") || relative_path.starts_with('/') {
        return Err("Invalid background image path.".to_string());
    }

    app.path()
        .app_config_dir()
        .map(|directory| directory.join(relative_path))
        .map_err(|error| error.to_string())
}

fn settings_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|directory| directory.join(SETTINGS_FILE_NAME))
        .map_err(|error| error.to_string())
}

fn store_background_image(
    app: &AppHandle,
    extension: &str,
    source_label: &str,
    bytes: &[u8],
) -> Result<String, String> {
    let directory = backgrounds_dir(app)?;
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_millis();
    let label = safe_file_stem(source_label);
    let file_name = format!("{timestamp}-{label}.{extension}");
    let path = directory.join(&file_name);
    fs::write(&path, bytes).map_err(|error| error.to_string())?;
    Ok(format!("backgrounds/{file_name}"))
}

fn safe_file_stem(label: &str) -> String {
    let stem: String = label
        .chars()
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() {
                Some(character.to_ascii_lowercase())
            } else if matches!(character, '-' | '_' | ' ') {
                Some('-')
            } else {
                None
            }
        })
        .take(36)
        .collect();

    if stem.trim_matches('-').is_empty() {
        "background".to_string()
    } else {
        stem.trim_matches('-').to_string()
    }
}

fn decode_image_data_url(data_url: &str) -> Result<(&'static str, Vec<u8>), String> {
    let (metadata, encoded) = data_url
        .split_once(',')
        .ok_or_else(|| "Image data is not a valid data URL.".to_string())?;
    if !metadata.ends_with(";base64") {
        return Err("Image data must be base64 encoded.".to_string());
    }

    let extension = match metadata {
        "data:image/png;base64" => "png",
        "data:image/jpeg;base64" | "data:image/jpg;base64" => "jpg",
        "data:image/webp;base64" => "webp",
        "data:image/gif;base64" => "gif",
        _ => return Err("Only PNG, JPEG, WebP, and GIF images are supported.".to_string()),
    };

    let bytes = base64_decode(encoded)?;
    Ok((extension, bytes))
}

fn mime_from_path(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "webp" => Some("image/webp"),
        "gif" => Some("image/gif"),
        _ => None,
    }
}

fn base64_decode(encoded: &str) -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity(encoded.len() * 3 / 4);
    let mut buffer: u32 = 0;
    let mut bits = 0;

    for byte in encoded.bytes().filter(|byte| !byte.is_ascii_whitespace()) {
        if byte == b'=' {
            break;
        }

        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return Err("Image data contains invalid base64 characters.".to_string()),
        } as u32;

        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xff) as u8);
        }
    }

    Ok(output)
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded = String::with_capacity(bytes.len().div_ceil(3) * 4);

    for chunk in bytes.chunks(3) {
        let first = chunk[0];
        let second = *chunk.get(1).unwrap_or(&0);
        let third = *chunk.get(2).unwrap_or(&0);
        let triple = ((first as u32) << 16) | ((second as u32) << 8) | third as u32;

        encoded.push(TABLE[((triple >> 18) & 0x3f) as usize] as char);
        encoded.push(TABLE[((triple >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            encoded.push(TABLE[((triple >> 6) & 0x3f) as usize] as char);
        } else {
            encoded.push('=');
        }
        if chunk.len() > 2 {
            encoded.push(TABLE[(triple & 0x3f) as usize] as char);
        } else {
            encoded.push('=');
        }
    }

    encoded
}

fn sync_overlay_windows(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    for ruler in &settings.rulers {
        sync_one_overlay(app, ruler)?;
    }
    Ok(())
}

fn sync_one_overlay(app: &AppHandle, ruler: &RulerSettings) -> Result<(), String> {
    let window = ensure_overlay_window(app, ruler)?;
    window
        .set_size(LogicalSize::new(ruler.width, ruler.height))
        .map_err(|error| error.to_string())?;
    window
        .set_position(LogicalPosition::new(ruler.x, ruler.y))
        .map_err(|error| error.to_string())?;
    window
        .set_always_on_top(true)
        .map_err(|error| error.to_string())?;
    window
        .set_ignore_cursor_events(ruler.click_through && !ruler.edit_mode)
        .map_err(|error| error.to_string())?;
    if ruler.visible {
        window.show().map_err(|error| error.to_string())?;
    } else {
        window.hide().map_err(|error| error.to_string())?;
    }
    emit_ruler_settings(app, ruler);
    Ok(())
}

fn ensure_overlay_window(
    app: &AppHandle,
    ruler: &RulerSettings,
) -> Result<tauri::WebviewWindow, String> {
    let label = overlay_label(&ruler.id);
    if let Some(window) = app.get_webview_window(&label) {
        return Ok(window);
    }

    WebviewWindowBuilder::new(app, label, WebviewUrl::App("overlay.html".into()))
        .title(format!("Reading Ruler - {}", ruler.name))
        .inner_size(ruler.width, ruler.height)
        .min_inner_size(120.0, 32.0)
        .position(ruler.x, ruler.y)
        .resizable(true)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .visible_on_all_workspaces(true)
        .skip_taskbar(true)
        .focused(false)
        .focusable(false)
        .visible(ruler.visible)
        .build()
        .map_err(|error| error.to_string())
}

fn hide_overlay_window(app: &AppHandle, ruler_id: &str) -> Result<(), String> {
    let label = overlay_label(ruler_id);
    let overlay = app
        .get_webview_window(&label)
        .ok_or_else(|| "Overlay window is not available.".to_string())?;
    overlay.hide().map_err(|error| error.to_string())
}

fn emit_ruler_settings(app: &AppHandle, ruler: &RulerSettings) {
    let payload = RulerEvent {
        ruler_id: ruler.id.clone(),
        settings: ruler.clone(),
    };
    let _ = app.emit_to(overlay_label(&ruler.id), "settings-changed", &payload);
    let _ = app.emit_to(MAIN_LABEL, "settings-changed", &payload);
}

fn emit_app_settings(app: &AppHandle, settings: &AppSettings) {
    let _ = app.emit_to(MAIN_LABEL, "app-settings-changed", settings);
}

fn register_shortcut(app: &AppHandle, shortcut: &str) -> Result<(), String> {
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                if let Ok(settings) = load_or_create_app_settings(&app_handle) {
                    let _ = toggle_ruler(app_handle.clone(), settings.active_ruler_id);
                }
            }
        })
        .map_err(|error| error.to_string())
}

fn show_main_window(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window(MAIN_LABEL)
        .ok_or_else(|| "Control panel window is not available.".to_string())?;
    window.show().map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())
}

fn show_help_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("help") {
        window.show().map_err(|error| error.to_string())?;
        return window.set_focus().map_err(|error| error.to_string());
    }

    WebviewWindowBuilder::new(app, "help", WebviewUrl::App("help.html".into()))
        .title("Reading Ruler Help")
        .inner_size(980.0, 760.0)
        .min_inner_size(520.0, 520.0)
        .center()
        .resizable(true)
        .decorations(true)
        .transparent(false)
        .build()
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn setup_app_menu(app: &App) -> tauri::Result<()> {
    let app_menu = SubmenuBuilder::new(app, "Reading Ruler")
        .about(None)
        .separator()
        .text(MENU_SHOW_PANEL, "Show Control Panel")
        .text(MENU_SHOW_RULER, "Show Active Ruler")
        .text(MENU_HIDE_RULER, "Hide Active Ruler")
        .text(MENU_TOGGLE_RULER, "Toggle Active Ruler")
        .separator()
        .text(MENU_RESET_SETTINGS, "Reset Active Ruler")
        .separator()
        .quit()
        .build()?;
    let edit_menu = SubmenuBuilder::new(app, "Edit")
        .undo()
        .separator()
        .cut()
        .copy()
        .paste()
        .separator()
        .select_all()
        .build()?;
    let help_menu = SubmenuBuilder::new(app, "Help")
        .text(MENU_SHOW_HELP, "Reading Ruler Help")
        .build()?;
    let menu = MenuBuilder::new(app)
        .item(&app_menu)
        .item(&edit_menu)
        .item(&help_menu)
        .build()?;
    app.set_menu(menu)?;
    Ok(())
}

fn handle_menu_event(app: &AppHandle, id: &str) {
    match id {
        MENU_SHOW_PANEL => {
            let _ = show_main_window(app);
        }
        MENU_SHOW_RULER => {
            if let Ok(settings) = load_or_create_app_settings(app) {
                let _ = show_ruler(app.clone(), settings.active_ruler_id);
            }
        }
        MENU_HIDE_RULER => {
            if let Ok(settings) = load_or_create_app_settings(app) {
                let _ = hide_ruler(app.clone(), settings.active_ruler_id);
            }
        }
        MENU_TOGGLE_RULER => {
            if let Ok(settings) = load_or_create_app_settings(app) {
                let _ = toggle_ruler(app.clone(), settings.active_ruler_id);
            }
        }
        MENU_RESET_SETTINGS => {
            let _ = reset_active_ruler(app.clone());
        }
        MENU_SHOW_HELP => {
            let _ = show_help_window(app);
        }
        _ => {}
    }
}

fn cf_key(key: CFStringRef) -> CFString {
    unsafe { CFString::wrap_under_get_rule(key) }
}

fn cf_string_value(dict: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<String> {
    dict.find(&cf_key(key))
        .and_then(|value| value.downcast::<CFString>())
        .map(|value| value.to_string())
}

fn cf_i32_value(dict: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<i32> {
    dict.find(&cf_key(key))
        .and_then(|value| value.downcast::<CFNumber>())
        .and_then(|value| value.to_i32())
}

fn cf_rect_value(dict: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<CGRect> {
    dict.find(&cf_key(key))
        .and_then(|value| value.downcast::<CFDictionary>())
        .and_then(|value| CGRect::from_dict_representation(&value))
}

fn ax_attribute(element: AXUIElementRef, attribute: &str) -> Option<CFType> {
    let attribute = CFString::new(attribute);
    let mut value: CFTypeRef = ptr::null();
    let status = unsafe {
        AXUIElementCopyAttributeValue(element, attribute.as_concrete_TypeRef(), &mut value)
    };

    if status == AX_ERROR_SUCCESS && !value.is_null() {
        Some(unsafe { CFType::wrap_under_create_rule(value) })
    } else {
        None
    }
}

fn ax_string_attribute(element: AXUIElementRef, attribute: &str) -> Option<String> {
    ax_attribute(element, attribute)
        .and_then(|value| value.downcast::<CFString>())
        .map(|value| value.to_string())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn ax_point_attribute(element: AXUIElementRef, attribute: &str) -> Option<CGPoint> {
    let value = ax_attribute(element, attribute)?;
    let mut point = CGPoint::new(0.0, 0.0);
    let ok = unsafe {
        AXValueGetValue(
            value.as_CFTypeRef(),
            AX_VALUE_CGPOINT_TYPE,
            &mut point as *mut CGPoint as *mut c_void,
        )
    };
    ok.then_some(point)
}

fn ax_size_attribute(element: AXUIElementRef, attribute: &str) -> Option<CGSize> {
    let value = ax_attribute(element, attribute)?;
    let mut size = CGSize::new(0.0, 0.0);
    let ok = unsafe {
        AXValueGetValue(
            value.as_CFTypeRef(),
            AX_VALUE_CGSIZE_TYPE,
            &mut size as *mut CGSize as *mut c_void,
        )
    };
    ok.then_some(size)
}

fn close_enough(left: f64, right: f64) -> bool {
    (left - right).abs() <= 8.0
}

fn ax_window_matches_bounds(element: AXUIElementRef, bounds: CGRect) -> bool {
    let Some(position) = ax_point_attribute(element, "AXPosition") else {
        return false;
    };
    let Some(size) = ax_size_attribute(element, "AXSize") else {
        return false;
    };

    close_enough(position.x, bounds.origin.x)
        && close_enough(position.y, bounds.origin.y)
        && close_enough(size.width, bounds.size.width)
        && close_enough(size.height, bounds.size.height)
}

fn accessibility_title_for_window(owner_pid: i32, bounds: CGRect) -> Option<String> {
    let app = unsafe { AXUIElementCreateApplication(owner_pid) };
    if app.is_null() {
        return None;
    }

    let windows = ax_attribute(app, "AXWindows")?.downcast::<CFArray>()?;
    windows.iter().find_map(|window| {
        let element = *window as AXUIElementRef;
        if ax_window_matches_bounds(element, bounds) {
            ax_string_attribute(element, "AXTitle")
        } else {
            None
        }
    })
}

fn target_windows() -> Vec<TargetWindow> {
    let options = kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;
    let array_ref = unsafe { CGWindowListCopyWindowInfo(options, kCGNullWindowID) };
    if array_ref.is_null() {
        return Vec::new();
    }

    let array: CFArray<CFDictionary<CFString, CFType>> =
        unsafe { TCFType::wrap_under_create_rule(array_ref) };

    array
        .iter()
        .filter_map(|entry| {
            let dict = &*entry;
            let layer = cf_i32_value(dict, unsafe { kCGWindowLayer })?;
            if layer != 0 {
                return None;
            }

            let id = cf_i32_value(dict, unsafe { kCGWindowNumber })? as u32;
            let owner_pid = cf_i32_value(dict, unsafe { kCGWindowOwnerPID })?;
            let owner_name = cf_string_value(dict, unsafe { kCGWindowOwnerName })?;
            if owner_name == "Reading Ruler" || owner_name == "reading-ruler" {
                return None;
            }

            let bounds = cf_rect_value(dict, unsafe { kCGWindowBounds })?;
            if bounds.size.width < 120.0 || bounds.size.height < 60.0 {
                return None;
            }
            let title = cf_string_value(dict, unsafe { kCGWindowName })
                .filter(|title| !title.trim().is_empty() && title != "Window")
                .or_else(|| accessibility_title_for_window(owner_pid, bounds))
                .unwrap_or_else(|| "Untitled window".to_string());

            Some(TargetWindow {
                id,
                owner_pid,
                owner_name,
                title,
                x: bounds.origin.x,
                y: bounds.origin.y,
                width: bounds.size.width,
                height: bounds.size.height,
            })
        })
        .take(80)
        .collect()
}

fn find_target_window(window_id: u32) -> Option<TargetWindow> {
    target_windows()
        .into_iter()
        .find(|window| window.id == window_id)
}

fn frontmost_target_window() -> Option<TargetWindow> {
    target_windows().into_iter().next()
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .on_menu_event(|app, event| {
            handle_menu_event(app, event.id().as_ref());
        })
        .on_window_event(|window, event| {
            if window.label() == MAIN_LABEL || window.label() == "help" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            load_app_settings,
            select_ruler,
            create_ruler,
            duplicate_ruler,
            delete_ruler,
            rename_ruler,
            save_ruler_settings,
            save_ruler_geometry,
            show_ruler,
            hide_ruler,
            toggle_ruler,
            active_ruler_visible,
            default_shortcut_label,
            set_shortcut,
            reset_active_ruler,
            show_control_panel,
            list_target_windows,
            save_background_image,
            clear_background_image,
            background_image_data_url,
            track_target_window,
            settings_path
        ])
        .setup(|app| {
            setup_app_menu(app)?;
            eprintln!(
                "Reading Ruler settings path: {}",
                settings_file_path(app.handle())?.display()
            );
            let settings = load_or_create_app_settings(app.handle())?;
            if let Err(error) = register_shortcut(app.handle(), &settings.shortcut) {
                eprintln!("Unable to register shortcut {}: {error}", settings.shortcut);
            }
            sync_overlay_windows(app.handle(), &settings)?;
            emit_app_settings(app.handle(), &settings);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run Reading Ruler");
}
