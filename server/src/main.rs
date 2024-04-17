use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::Error;
use std::io::Write;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/upload/{dir:.+}")]
async fn upload(path: web::Path<String>, body: web::Bytes) -> impl Responder {
    println!("File RECEIVED!");

    let dir = path.into_inner().to_string();

    println!("{}", dir);

    // Saving the file to local storage
    // let file_bytes = body.clone();
    // let file_path: String = format!("./{}", dir);
    // if let Err(err) = save_file(file_bytes, file_path).await {
    //     println!("Error saving file: {}", err);
    // } else {
    //     println!("File saved successfully");
    // }

    HttpResponse::Ok().body(format!("Received {} bytes", body.len()))
}

async fn save_file(file_bytes: web::Bytes, file_path: String) -> Result<(), Error> {
    let mut file = File::create(file_path)?;
    file.write_all(&file_bytes)?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(upload))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
