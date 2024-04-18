// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use reqwest::Body;
use tokio_util::codec::{BytesCodec, FramedRead};
use std::env;
use lazy_static::lazy_static;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use serde_json::{json, Value};

// Using a global var for the app settings path and initialising the path safely
lazy_static! {
    static ref DATA_DIR: PathBuf = {
        // Matching the operating system with the app data path
        let mut data_dir = match std::env::consts::OS {
            "windows" => PathBuf::from(std::env::var_os("APPDATA").unwrap()).join("ROAMING\\syncer\\"),
            "macos" => PathBuf::from(std::env::var_os("HOME").unwrap()).join("Library/Application Support/syncer/"),
            "linux" => PathBuf::from(std::env::var_os("HOME").unwrap()).join(".config/syncer/"),
            _ => PathBuf::from("."),
        };

        data_dir.push("my_app");
        data_dir
    };
}

#[tauri::command]
fn select_folder(game_name: String, app_handle: tauri::AppHandle) {
    // Using the app handler to start a file picking dialog
    app_handle.dialog().file().pick_folder(move |folder_path| {
        // Return a file_path `Option`, or `None` if the user closes the dialog
        let result = match folder_path {
            Some(file_response) => file_response.into_os_string().into_string().unwrap(),
            None => "".to_string(),
        };

        // Handling the cases that the result can be in
        if result == "" {
            println!("Failed to find folder");
        } else {
            println!("The folder path: {}", result);

            write_folder_to_json(game_name, result.clone());

            // Broadcasting that the file has been found to the frontend
            app_handle.emit("folder-selected", result.clone()).unwrap();

            // Sending the file to the server
            let _ = send_folder_contents(result, "http://127.0.0.1:8080");
        }
    });
}

fn write_folder_to_json(game_name: String, path: String) {
    let file_path: &str = &DATA_DIR.join("games.json").into_os_string().into_string().unwrap();

    // Reading the JSON from with serde
    let file = fs::File::open(file_path)
        .expect("file should open read only");
    let mut json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    // Add a new value to the JSON
    json[game_name] = json!(path);

    // Write the modified JSON back to the file
    let new_json_data = json.to_string();
    fs::write(PathBuf::from(file_path), new_json_data)
        .expect("Unable to write file");
}

// Sending the file over HTTP
#[tokio::main]
async fn send_folder_contents(directory: String, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Getting all the files in the directory
    let paths = fs::read_dir(directory)?;

    // Looping through all the files and sending them
    for path in paths {
        let path = path?;
        let file_path = path.path();
        let filename = path.file_name().to_string_lossy().to_string();

        let file = tokio::fs::File::open(file_path).await?;
        let client = reqwest::Client::new();

        // Uploading the file
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::stream(file).file_name(filename.clone()));

        let _res = client
            .post(format!("{}/upload", url)) // "{}/upload/Lies Of P/{}" url, filename
            .multipart(form)
            .send()
            .await?;
    }

    Ok(())
}

// Converting the file to a streamable frame
fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}

#[tokio::main]
async fn setup_games_json() {
    // The saved game folders to sync are in a JSON file
    let games_to_sync = DATA_DIR.join("games.json");

    // Create the parent directory if it doesn't exist
    if let Some(parent) = games_to_sync.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create parent directory");
    }

    // If the games JSON does not exist then create it! (Typically first time app is run)
    // Currently the games list JSON will not be synced to cloud
    if !games_to_sync.exists() {
        println!("File does not exist, creating a new one...");
        let mut file = File::create(&games_to_sync)
            .await
            .expect("Failed to create file");
        let initial_data = r#"[]"#;
        file.write_all(initial_data.as_bytes())
            .await
            .expect("Failed to write initial data to file");
        println!("File created successfully!");
    }
    else {
            println!("File already exists.");
    }
}

#[tauri::command]
fn get_games_list(_app_handle: tauri::AppHandle) -> Result<String, String> {
    let file_path: &str = &DATA_DIR.join("games.json").into_os_string().into_string().unwrap();

    let file_contents = fs::read_to_string(file_path)
        .map_err(|err| err.to_string())?;

    let json_data: Value = serde_json::from_str(&file_contents)
        .map_err(|err| err.to_string())?;

    Ok(json!(json_data).to_string())
}

fn main() {
    setup_games_json();

    // Setting up the Tauri app frontend
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![select_folder, get_games_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
