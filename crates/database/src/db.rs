use dirs::home_dir;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn create_conn() -> Result<Connection> {
    // get user home directory
    let mut db_path = home_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push(".smriti.db");

    let conn = Connection::open(db_path)?;

    // Create the necessary tables if they do not exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
             id INTEGER PRIMARY KEY,
             command TEXT NOT NULL UNIQUE,
             alias TEXT UNIQUE,
             info TEXT,
             service TEXT
        )",
        [],
    )?;

    Ok(conn) // Return the connection
}
