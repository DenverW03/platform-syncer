// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_dialog::DialogExt;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn select_file(app_handle: tauri::AppHandle) {
    // Using the app handler to start a file picking dialog
    app_handle.dialog().file().pick_file(move |file_path| {
        // return a file_path `Option`, or `None` if the user closes the dialog
        let result = match file_path {
            Some(file_response) => file_response.path.into_os_string().into_string().unwrap(),
            None => "".to_string(),
        };

        // Handling the cases that the result can be in
        if result == "" {
            println!("Failed to find file");
        }
        else {
            println!("The file path: {}", result);
            app_handle.emit("file-selected", result).unwrap();
        }
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
