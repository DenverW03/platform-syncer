use rusqlite::{Connection, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Game {
    directory: String,
    date_time: i32,
}

// Function used to check whether the database file exists, if not, creates it
pub fn check_database() -> Result<(), rusqlite::Error> {
    // Will create the database if it does not exist
    let conn = Connection::open("database.db")?;

    // Creating the table if it does not already exist, typically if the database did not already exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Games (directory TEXT NOT NULL, date_time INTEGER NOT NULL, PRIMARY KEY (directory))",
        (),
    )?;

    Ok(())
}

// Function used to insert a row of data into the database
pub fn insert_directory(directory: PathBuf) -> Result<(), rusqlite::Error> {
    // Connect to the database
    let conn = Connection::open("database.db")?;

    // TODO: Update the database entry (check for existence and if not create entry, otherwise update)

    // Creating a struct instance of the data so that rusqlite can handle it
    let game: Game = Game {
        directory: directory.into_os_string().into_string().unwrap(),
        date_time: 0, // Throwaway placeholder for now, isn't inserted into the table anyway
    };

    let statement = format!(
        "INSERT INTO Games (directory, date_time) VALUES ({}, strftime('%s', 'now'))",
        game.directory
    );
    conn.execute(&statement, ())?;

    Ok(())
}
