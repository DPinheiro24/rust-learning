// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sysinfo::System;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};
use rand::prelude::*;
use std::thread;
use std::time::Duration;


#[derive(Serialize, Clone)]
struct SysInfo {
    total_memory: u64,
    used_memory: u64,
    system_name: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    cpu_usage: HashMap<String, i32>
}

struct AppState {
    is_paused: bool,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn switch_pause(state: State<Arc<Mutex<AppState>>>) -> Result<String, String> {

    let mut stats = state.lock().unwrap();

    let message = if stats.is_paused {
        stats.is_paused = false;
        "falso"
    } else {
        stats.is_paused = true;
        "verdadeiro"
    };

    let notification = format!("Estado de procura alterado para {}", message);

    Ok(notification)
}

#[tauri::command]
fn get_sys_info() -> Result<SysInfo, String> {

    let mut sys = System::new_all();

    sys.refresh_all();

    let total_mem = sys.total_memory();

    let used_mem  = sys.used_memory();

    let system_name = System::name();

    let os_version = System::os_version();

    let host_name = System::host_name();

    let mut cpu_usage = HashMap::new();

    let mut count = 1;
    
    sys.refresh_cpu_usage();
    
    let mut rng = rand::rng();

    for _cpu in sys.cpus() {
        let fake_cpu: i32 = rng.random_range(1..=100);
        cpu_usage.insert(format!("CPU {}", count), fake_cpu);
        count += 1;
    }

    Ok(SysInfo {
        total_memory: total_mem,
        used_memory: used_mem,
        system_name: system_name,
        os_version: os_version,
        host_name: host_name,
        cpu_usage: cpu_usage,
    })
    
}

fn start_polling(app: AppHandle, state: Arc<Mutex<AppState>>) {
    thread::spawn(move || {
        loop {
            let paused = state.lock().unwrap().is_paused;
            if !paused {
                if let Ok(info) = get_sys_info() {
                    let _ = app.emit("sys-info", info);
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(AppState { is_paused: true })))
        .setup(|app| {
            let state = app.state::<Arc<Mutex<AppState>>>().inner().clone();
            start_polling(app.handle().clone(), state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_sys_info, switch_pause])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
