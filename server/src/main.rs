use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/upload/{filename:.+}")]
async fn upload(
    path_game_name: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        println!("File RECEIVED!");

        let path_temp = path_game_name.clone();

        // Getting the directory path to save to, the files parent directory, so can confirm that it exists
        let filename = f.file_name.unwrap();
        let file_path = PathBuf::from("./saves/".to_owned() + &path_temp.to_string() + &filename);
        let dir = file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        if !&dir.exists() {
            std::fs::create_dir(&dir).expect("Failed to create game save directory");
        }

        // Writing the file to the appropriate path
        f.file.persist(file_path).unwrap();
    }

    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Checking for saves directory, creating if nonexistent
    if !PathBuf::from("./saves/").exists() {
        std::fs::create_dir(PathBuf::from("./saves/")).expect("Failed to create saves directory");
    }

    HttpServer::new(|| App::new().service(hello).service(upload))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
