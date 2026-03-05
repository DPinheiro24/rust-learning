// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sysinfo::System;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;
use rand::prelude::*;


#[derive(Serialize)]
struct SysInfo {
    total_memory: String,
    used_memory: String,
    system_name: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    cpu_usage: HashMap<String, i32>
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_sys_info() -> Result<SysInfo, String> {

    let mut sys = System::new_all();

    sys.refresh_all();

    let total_mem = sys.total_memory();

    let used_mem  = sys.used_memory();

    let system_name = System::os_version();

    let os_version = System::os_version();

    let host_name = System::host_name();

    let mut cpu_usage = HashMap::new();

    let mut count = 1;
    
    sys.refresh_cpu_usage();
    
    let mut rng = rand::rng();

    for cpu in sys.cpus() {
        let fake_cpu: i32 = rng.random_range(1..=100);
        cpu_usage.insert(format!("CPU {}", count), fake_cpu);
        count += 1;
    }

    Ok(SysInfo {
        total_memory: total_mem.to_string(),
        used_memory: used_mem.to_string(),
        system_name: system_name,
        os_version: os_version,
        host_name: host_name,
        cpu_usage: cpu_usage,
    })
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,get_sys_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
