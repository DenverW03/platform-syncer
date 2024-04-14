use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/post")]
async fn upload(body: web::Bytes) -> impl Responder {
    // TODO: Implement file upload logic
    println!("File RECEIVED!");

    HttpResponse::Ok().body(format!("Received {} bytes", body.len()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(upload))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
