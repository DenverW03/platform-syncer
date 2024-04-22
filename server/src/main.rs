use actix_web::{App, HttpServer};

mod database_interface;
mod rest_api;
mod storage_interface;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Checking for saves directory, creating if nonexistent
    storage_interface::saves_dir_check();

    HttpServer::new(|| App::new().service(rest_api::upload))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
