use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Window, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub fn drag_window(window: Window) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn window_minimize(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn window_maximize(window: Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn window_close(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetConfig {
    pub id: String,
    pub title: String,
    // Parameter toggles
    pub show_cpu: bool,
    pub show_gpu: bool,
    pub show_ram: bool,
    pub show_disk: bool,
    pub show_time: bool,
    pub show_date: bool,
    pub show_process_count: bool,
    pub temp_unit: String, // "C" or "F"

    // Style properties
    pub bg_color: String,
    pub text_color: String,
    pub bg_image: String,
    pub bg_image_size: String,
    pub bg_image_position: String,
    pub bg_image_opacity: f32,
    pub widget_size: String, // "small", "medium", "large", "custom"
    pub custom_width: u32,
    pub custom_height: u32,
    pub shape: String, // "square", "rounded", "capsule"
    pub font_size: u32, // in px
    pub font_family: String,
    pub font_weight: String, // "300", "400", "500", "600", "700"
    pub widget_opacity: f32, // 0.1 to 1.0
    pub has_border: bool,
    pub border_color: String,
    pub border_opacity: f32, // 0.0 to 1.0
}

#[tauri::command]
pub fn spawn_widget_window(app: AppHandle, config: WidgetConfig) -> Result<String, String> {
    let window_label = format!("widget_{}", config.id);

    // If window already exists, focus it
    if let Some(existing_window) = app.get_webview_window(&window_label) {
        let _ = existing_window.set_focus();
        return Ok(window_label);
    }

    let (width, height) = match config.widget_size.as_str() {
        "small" => (200.0, 120.0),
        "medium" => (280.0, 180.0),
        "large" => (360.0, 260.0),
        "custom" => (
            config.custom_width.max(80) as f64,
            config.custom_height.max(30) as f64,
        ),
        _ => (280.0, 180.0),
    };

    let encoded_id = config.id.clone();
    let url = format!("index.html#/widget?id={}", encoded_id);

    let builder = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::App(url.into()),
    )
    .title(&config.title)
    .inner_size(width, height)
    .min_inner_size(40.0, 30.0)
    .resizable(true)
    .decorations(false)
    .transparent(true)
    .shadow(false)
    .always_on_top(true);

    match builder.build() {
        Ok(_) => Ok(window_label),
        Err(e) => Err(format!("Failed to launch floating widget: {}", e)),
    }
}

#[tauri::command]
pub fn close_widget_window(app: AppHandle, widget_id: String) -> Result<(), String> {
    let window_label = format!("widget_{}", widget_id);
    if let Some(window) = app.get_webview_window(&window_label) {
        let _ = window.close();
    }
    Ok(())
}
