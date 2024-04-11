// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use reqwest::blocking::Client;

#[tauri::command]
fn select_file(app_handle: tauri::AppHandle) {
    // Using the app handler to start a file picking dialog
    app_handle.dialog().file().pick_folder(move |folder_path| {
        // return a file_path `Option`, or `None` if the user closes the dialog
        let result = match folder_path {
            Some(file_response) => file_response.into_os_string().into_string().unwrap(),
            None => "".to_string(),
        };

        // Handling the cases that the result can be in
        if result == "" {
            println!("Failed to find folder");
        } else {
            println!("The folder path: {}", result);
            app_handle.emit("file-selected", result).unwrap();
        }
    });
}

async fn send_file(file_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("from_a_file.txt")?;
    let client = Client::new();
    let res = client.post("http://httpbin.org/post")
        .body(file)
        .send()?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
