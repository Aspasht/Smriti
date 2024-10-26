use rusqlite::{Connection, Result};

pub fn create_conn() -> Result<Connection> {
    let conn = Connection::open("smriti.db")?;

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
