use tauri::{AppHandle, Manager};
use std::fs;
use std::path::PathBuf;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_note(app: AppHandle, title: String, content: String) -> Result<String, String> {
    // Get app data directory
    let app_dir = app.path().app_data_dir().map_err(|e: tauri::Error| e.to_string())?;
    
    // Create notes directory if it doesn't exist
    let notes_dir = app_dir.join("notes");
    fs::create_dir_all(&notes_dir).map_err(|e: std::io::Error| e.to_string())?;
    
    // Create file path (sanitize title for filename)
    let filename = format!("{}.txt", title.replace("/", "_"));
    let file_path = notes_dir.join(filename);
    
    // Write content to file
    fs::write(&file_path, content).map_err(|e: std::io::Error| e.to_string())?;
    
    Ok(format!("Note saved: {}", title))
}

#[tauri::command]
fn get_notes(app: AppHandle) -> Result<Vec<String>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e: tauri::Error| e.to_string())?;
    let notes_dir = app_dir.join("notes");
    
    // Return empty list if directory doesn't exist yet
    if !notes_dir.exists() {
        return Ok(vec![]);
    }
    
    // Read all .txt files
    let entries = fs::read_dir(notes_dir).map_err(|e: std::io::Error| e.to_string())?;
    let mut notes = Vec::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".txt") {
                    notes.push(filename.replace(".txt", ""));
                }
            }
        }
    }
    
    Ok(notes)
}

#[tauri::command]
fn read_note(app: AppHandle, title: String) -> Result<String, String> {
    let app_dir = app.path().app_data_dir()
        .map_err(|e: tauri::Error| e.to_string())?;
    let notes_dir = app_dir.join("notes");
    let filename = format!("{}.txt", title.replace("/", "_"));
    let file_path = notes_dir.join(filename);
    
    fs::read_to_string(file_path).map_err(|e: std::io::Error| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_note,
            get_notes,
            read_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}