use actix_web::{App, HttpServer};
use database_interface::check_database;

mod database_interface;
mod rest_api;
mod storage_interface;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Checking for saves directory, creating if nonexistent
    storage_interface::saves_dir_check();

    // Database presence + setup checking
    check_database().expect("Failed to setup the database");

    HttpServer::new(|| {
        App::new()
            .service(rest_api::upload)
            .service(rest_api::last_modified)
            .service(rest_api::get_sync)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
