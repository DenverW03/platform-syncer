use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use crate::database_interface;
use crate::storage_interface;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/upload/{game_name:.+}")]
async fn upload(
    game_name: web::Path<String>, // The string given after upload/ in the upload URL, depicted above
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        println!("File RECEIVED!");

        let game_name_dir = game_name.clone();

        // Calling the file storage handling function
        storage_interface::store_file(f, game_name_dir);
    }

    // Getting the directory for the SQL table insertion after file storage
    let dir = PathBuf::from("./saves/").join(game_name.clone());

    // Adding to database after file saved
    database_interface::insert_directory(dir).expect("Failed to update saved records"); // doing nothing with the returned result atm but will later. TODO i guess lmao

    Ok(HttpResponse::Ok())
}

#[get("/last_modified/{game_name:.+}")]
async fn last_modified(game_name: web::Path<String>) -> Result<impl Responder, Error> {
    let date: i32 = database_interface::get_last_modified(game_name.clone()).unwrap();

    // Converting to a string to return in the response body
    let response: String = date.to_string();
    Ok(HttpResponse::Ok().body(response))
}

// Rather than just the game name, this endpoint works via the specific save file
#[get("/get_sync/{game_file_path:.+}")]
async fn get_sync(game_name: web::Path<String>) -> impl Responder {
    println!("Syncing server side");

    let file_paths = fs::read_dir(format!("./saves/{}", game_name))?;

    Ok()
}
