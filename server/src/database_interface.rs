use rusqlite::{Connection, Result};

// Struct representing game directory table row, simple two column table for the storage directory and the data/time of the most recent update
#[derive(Debug)]
struct Game {
    directory: String,
    date_time: i32, // the date/time is stored as unix time
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
