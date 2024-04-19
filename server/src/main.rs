use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
    Multipart,
};
use actix_web::http::StatusCode;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use futures::TryStream;
use futures::{StreamExt, TryStreamExt};
use std::fs::File;
use std::io::{Error, Write};
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

// endpoint /upload/path+filename
// #[post("/upload/{filename:.+}")]
// async fn upload(path: web::Path<String>, body: web::Bytes) -> impl Responder {
//     println!("File RECEIVED!");

//     // Getting the path to save to including file name
//     let file_path = PathBuf::from("./saves/".to_owned() + &path.into_inner().to_string());

//     // Getting the directory path to save to, the files parent directory
//     let dir = file_path
//         .parent()
//         .unwrap_or_else(|| Path::new("."))
//         .to_path_buf();

//     if !&dir.exists() {
//         std::fs::create_dir(&dir).expect("Failed to create game save directory");
//     }

//     // Saving the file to local storage
//     let file_bytes = body.clone();
//     if let Err(err) = save_file(file_bytes, file_path.to_str().unwrap().to_string()).await {
//         println!("Error saving file: {}", err);
//     } else {
//         println!("File saved successfully");
//     }

//     HttpResponse::Ok().body(format!("Received {} bytes", body.len()))
// }

// async fn save_file(file_bytes: web::Bytes, file_path: String) -> Result<(), Error> {
//     let mut file = File::create(file_path)?;
//     file.write_all(&file_bytes)?;
//     Ok(())
// }

// #[post("/upload")]
// pub async fn upload(mut payload: web::Payload, file_path: web::Path<String>) -> impl Responder {
//     let filepath = format!("./saves/{}", file_path.into_inner());

//     let mut f = web::block(|| std::fs::File::create(filepath))
//         .await
//         .map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
//         })?;

//     while let Some(chunk) = payload.next().await {
//         let bytes = chunk.map_err(|e| {
//             eprintln!("{}", e);
//             HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
//         })?;

//         web::block(move || f.unwrap().write_all(&bytes))
//             .await
//             .map_err(|e| {
//                 eprintln!("{}", e);
//                 HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
//             })?;
//     }

//     Ok::<HttpResponse>(HttpResponse::Ok().body("File uploaded successfully"))
// }

#[post("/upload")]
async fn upload(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    for f in form.files {
        // Each call is a received file technically I think
        println!("File RECEIVED!");

        let path = format!("./saves/{}", f.file_name.unwrap());
        // println!("saving to {path}");
        // f.file.persist(path).unwrap();

        // Getting the necessary paths
        // let file_path = PathBuf::from("./saves/".to_owned() + &path.into_inner().to_string());
        // Getting the directory path to save to, the files parent directory, so can confirm that it exists
        let file_path = PathBuf::from(path);
        let dir = file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        if !&dir.exists() {
            std::fs::create_dir(&dir).expect("Failed to create game save directory");
        }

        // Writing the file to the appropriate path
        // let mut file = File::create(file_path)?;
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
