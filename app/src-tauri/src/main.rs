// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use futures::stream::TryStreamExt;
use reqwest::{Body};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

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

            // Broadcasting that the file has been found to the frontend
            app_handle.emit("folder-selected", result.clone()).unwrap();

            // Sending the file to the server
            let _ = send_file(result, "127.0.0.1:8080");
        }
    });
}

#[tokio::main]
async fn send_file(file_path: String, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(format!("{}/this.txt", file_path)).await?;

    let client = reqwest::Client::new();
    let _res = client
        .post(format!("{}/post", url))
        .body(file_to_body(file))
        .send()
        .await?;

    Ok(())
}

fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![select_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
