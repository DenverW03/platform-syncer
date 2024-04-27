use actix_multipart::form::tempfile::TempFile;
use std::path::{Path, PathBuf};

// Function checks for the existence of a save directory, creates one if nonexistent
pub fn saves_dir_check() {
    // Checking for saves directory, creating if nonexistent
    if !PathBuf::from("./saves/").exists() {
        std::fs::create_dir(PathBuf::from("./saves/")).expect("Failed to create saves directory");
    }
}

// Function that writes a file to the provided subdir in the saves directory
pub fn store_file(file: TempFile, game_name: String) {
    // Getting the directory path to save to, the files parent directory, so can confirm that it exists
    let filename = file.file_name.unwrap();
    let file_path = PathBuf::from("./saves/".to_owned() + &game_name.to_string() + &filename);
    let dir = file_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    // If the game directory does not already exist, create it.
    if !&dir.exists() {
        std::fs::create_dir(&dir).expect("Failed to create game save directory");
    }

    // Writing the file to the appropriate path
    file.file.persist(file_path).unwrap();
}
