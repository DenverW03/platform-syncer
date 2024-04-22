use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use std::io::Error;

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

        // Calling the file storage handling function
        let game_name_dir = game_name.clone();
        storage_interface::store_file(f, game_name_dir);
    }

    Ok(HttpResponse::Ok())
}
