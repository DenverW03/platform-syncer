// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Write;

use tauri_plugin_dialog::{DialogExt, FileResponse};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn select_file(app_handle: tauri::AppHandle) -> String {
    let mut return_var = String::new();
    // Using the app handler to start a file picking dialog
    app_handle.dialog().file().pick_file(move |file_path| {
        // return a file_path `Option`, or `None` if the user closes the dialog
        match file_path {
            Some(path) => match path.name {
                Some(name) => return_var.write_str(&name),
                None => return_var.write_str("Failed to find file"),
            },
            None => return_var.write_str("Failed to find file"),
        };
    });

    return return_var;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
