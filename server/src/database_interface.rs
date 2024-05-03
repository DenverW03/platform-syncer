use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Game {
    directory: String,
    date_time: i32,
}

// Function used to get the last modified date of a game entry in the database
pub fn get_last_modified(game_name: String) -> Result<i32> {
    // Connect to the database
    let conn = Connection::open("database.db")?;

    // Request the row from the database
    let mut stmt = conn.prepare("SELECT * FROM Games WHERE directory=./saves/?/")?;
    let entry = stmt.query_row(params![game_name], |row| {
        Ok(Game {
            directory: row.get(0)?,
            date_time: row.get(1)?,
        })
    })?;

    // Returning the unix time value wrapped in a Result
    Ok(entry.date_time)
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
    let directory_string = directory.into_os_string().into_string().unwrap();
    let game: Game = Game {
        directory: directory_string,
        date_time: 0, // Throwaway placeholder for now, isn't inserted into the table anyway
    };

    // Check if the directory entry already exists
    let mut stmt = conn.prepare("SELECT 1 FROM Games WHERE directory = ?")?;
    let exists = stmt.exists([game.directory.clone()])?;
    if exists {
        // Update the existing row
        let mut stmt = conn.prepare("UPDATE Games SET date_time = cast(strftime('%s', 'now') as integer) WHERE directory = ?")?;
        stmt.execute([game.directory.clone()])?;
    } else {
        // Insert a new row
        let mut stmt = conn.prepare("INSERT INTO Games (directory, date_time) VALUES (?, cast(strftime('%s', 'now') as integer))")?;
        stmt.execute([game.directory.clone()])?;
    }

    Ok(())
}
