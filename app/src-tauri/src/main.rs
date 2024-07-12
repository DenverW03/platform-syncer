// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

            write_folder_to_json(game_name.clone(), result.clone());

            // Broadcasting that the file has been found to the frontend
            app_handle.emit("folder-selected", result.clone()).unwrap();

            // Sending the file to the server
            let _ = send_folder_contents(result, "http://127.0.0.1:8080", game_name.clone());
        }
    });
}

fn write_folder_to_json(game_name: String, path: String) {
    let file_path: &str = &DATA_DIR
        .join("games.json")
        .into_os_string()
        .into_string()
        .unwrap();

    // Reading the JSON from with serde
    let file = fs::File::open(file_path).expect("file should open read only");
    let mut json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    // Add a new value to the JSON
    json[game_name] = json!(path);

    // Write the modified JSON back to the file
    let new_json_data = json.to_string();
    fs::write(PathBuf::from(file_path), new_json_data).expect("Unable to write file");
}

// Sending the file over HTTP
#[tokio::main]
async fn send_folder_contents(
    directory: String,
    url: &str,
    game_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::stream(file).file_name(filename.clone()),
        );

        let _result = client
            .post(format!("{}/upload/{}/", url, game_name)) // upload location depends on game
            .multipart(form)
            .send()
            .await?;
    }

    Ok(())
}

#[tauri::command]
async fn sync_game(game_name: String, path: String, _app_handle: tauri::AppHandle) {
    // Check when the file was last modified
    let local_date = date_modified_local(path.clone()).await;

    // Check when the server record was last updated
    let server_date = date_modified_server(game_name.clone())
        .await
        .expect("TODO REASON");

    // If record newer then download, if local newer then upload
    if local_date > server_date {
        // Sync the server to match the local
        local_sync(game_name, path).await;
    } else {
        // Sync the local to match the server
        server_sync(game_name, path).await;
    }
}

// This function is used to sync the server to the local gamefiles
async fn local_sync(_game_name: String, path: String) {
    println!("Local sync. The path is: {}", path);
}

// This function is used to sync the local gamefiles to the server
async fn server_sync(game_name: String, path: String) {
    println!("Server sync. The path is: {}", path);

    // Request the file from the server and handle receipt
    // Dealing with multipart get request not happening so request each individually
    // Files are named the same server side as client side\
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Failed to read directory");
            return; // Exit the function if reading the directory fails
        }
    };

    for path in paths {
        // Match on the result of the iterator item
        let path = match path {
            Ok(path) => path,
            Err(_) => {
                println!("Failed to read path");
                continue; // Skip this iteration if reading the path fails
            }
        };

        // Getting the file name as a proper string
        let file_name = match path.path().file_name() {
            Some(file_name) => file_name.to_owned(),
            None => {
                println!("Failed to get file name");
                continue; // Skip this iteration if getting the file name fails
            }
        };
        let file_name2 = match file_name.into_string() {
            Ok(file_name2) => file_name2,
            Err(_) => {
                println!("Failed to get file name 2");
                continue;
            }
        };

        // Building the overall request location
        let request_location = format!("{}/{}", game_name, file_name2);

        // Requesting the file from the server
        let client = reqwest::Client::new();
        let result = client
            .get(format!(
                "http://127.0.0.1:8080/get_sync/{}/",
                request_location
            ))
            .send()
            .await;

        // Overwrite local file with one from server
    }
}

// Requests the date modified entry from the server using the game name
async fn date_modified_server(game_name: String) -> Result<i32, Box<dyn std::error::Error>> {
    // Sending a get request
    let client = reqwest::Client::new();
    let result = client
        .get(format!(
            "http://127.0.0.1:8080/last_modified/{}/",
            game_name
        ))
        .send()
        .await?;

    // Convert the response bytes to a string
    let string_response = String::from_utf8(result.bytes().await?.to_vec())?;

    // Parse the string into an i32
    let date = string_response.parse::<i32>()?;

    Ok(date)
}

// Uses the path to find the date that the folder was last modified
// Returns the date as unix time
async fn date_modified_local(dir: String) -> i32 {
    // Getting all files in the directory
    let paths = fs::read_dir(dir).unwrap();

    // Counter to keep the most recent modified date
    let mut counter: i32 = 0;
    for path in paths {
        let path = path.unwrap();
        let file_path = path.path();

        // Getting the date last modified of the file
        let metadata = tokio::fs::metadata(file_path).await.unwrap();
        let time = metadata.modified().unwrap();
        let unix_time = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;

        // Comparing with and updating counter if necessary
        if unix_time > counter {
            counter = unix_time
        }
    }

    // The counter should now hold the unix time of the most recently modified file, being the overall most recent version of the game save
    counter
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
    } else {
        println!("File already exists.");
    }
}

#[tauri::command]
fn get_games_list(_app_handle: tauri::AppHandle) -> Result<String, String> {
    let file_path: &str = &DATA_DIR
        .join("games.json")
        .into_os_string()
        .into_string()
        .unwrap();

    let file_contents = fs::read_to_string(file_path).map_err(|err| err.to_string())?;

    let json_data: Value = serde_json::from_str(&file_contents).map_err(|err| err.to_string())?;

    Ok(json!(json_data).to_string())
}

fn main() {
    setup_games_json();

    // Setting up the Tauri app frontend
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            select_folder,
            get_games_list,
            sync_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
