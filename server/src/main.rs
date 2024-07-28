use std::fs;

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
    match check_database() {
        Ok(_) => println!("Database is set up"),
        Err(_) => println!("Failed to setup the database"),
    }

    // Loading the URL from the "./url.txt" file
    let url = match fs::read_to_string("./url.txt") {
        Ok(url) => url.replace("\n", ""),
        Err(_) => {
            println!("Failed to read file string");
            return Ok(());
        }
    };

    HttpServer::new(|| {
        App::new()
            .service(rest_api::upload)
            .service(rest_api::last_modified)
            .service(rest_api::get_sync)
    })
    .bind(url)?
    .run()
    .await
}
