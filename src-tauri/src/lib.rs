mod metrics;
mod processes;
mod widget;

use metrics::{fetch_system_metrics, SystemMetrics};
use processes::{fetch_process_details, get_process_list, kill_target_process, ProcessDetails, ProcessItem};
use std::sync::Mutex;
use sysinfo::System;
use tauri::State;
use widget::{close_widget_window, drag_window, spawn_widget_window, window_close, window_maximize, window_minimize};

pub struct AppState {
    sys: Mutex<System>,
}

#[tauri::command]
fn get_system_metrics(state: State<'_, AppState>) -> SystemMetrics {
    let mut sys = state.sys.lock().unwrap();
    fetch_system_metrics(&mut sys)
}

#[tauri::command]
fn get_processes(state: State<'_, AppState>) -> Vec<ProcessItem> {
    let mut sys = state.sys.lock().unwrap();
    get_process_list(&mut sys)
}

#[tauri::command]
fn get_process_info(state: State<'_, AppState>, pid: u32) -> Option<ProcessDetails> {
    let mut sys = state.sys.lock().unwrap();
    fetch_process_details(&mut sys, pid)
}

#[tauri::command]
fn kill_process(state: State<'_, AppState>, pid: u32) -> bool {
    let mut sys = state.sys.lock().unwrap();
    kill_target_process(&mut sys, pid)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut sys = System::new_all();
    sys.refresh_all();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            sys: Mutex::new(sys),
        })
        .invoke_handler(tauri::generate_handler![
            get_system_metrics,
            get_processes,
            get_process_info,
            kill_process,
            spawn_widget_window,
            close_widget_window,
            drag_window,
            window_minimize,
            window_maximize,
            window_close
        ])
        .run(tauri::generate_context!())
        .expect("error while running Cacao Performance application");
}
