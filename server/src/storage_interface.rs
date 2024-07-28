use actix_multipart::form::tempfile::TempFile;
use std::path::PathBuf;

// Function checks for the existence of a save directory, creates one if nonexistent
pub fn saves_dir_check() {
    // Checking for saves directory, creating if nonexistent
    if !PathBuf::from("./saves/").exists() {
        match std::fs::create_dir(PathBuf::from("./saves/")) {
            Ok(_) => println!("Saves directory created!"),
            Err(_) => println!("Failed to create the saves directory"),
        }
    }
}

// Function that writes a file to the provided subdir in the saves directory
pub fn store_file(file: TempFile, game_name: String) {
    // Getting the directory path to save to, the files parent directory, so can confirm that it exists
    let filename = match file.file_name {
        Some(filename) => filename,
        None => {
            println!("Failed to get the filename from the file");
            return; // can't continue without the filename
        }
    };
    let file_path = PathBuf::from("./saves/".to_owned() + &game_name.to_string() + &filename);
    let dir = match file_path.parent() {
        Some(dir) => dir.to_path_buf(),
        None => {
            println!("Failed to get the parent directory from file path");
            return;
        }
    };

    // If the game directory does not already exist, create it.
    if !&dir.exists() {
        std::fs::create_dir(&dir).expect("Failed to create game save directory");
    }

    // Writing the file to the appropriate path
    match file.file.persist(file_path) {
        Ok(_) => println!("File saved successfully!"),
        Err(_) => println!("Failed to save file"),
    }
}
