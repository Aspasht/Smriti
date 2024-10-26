use crate::models::Command;
use rusqlite::{params, Connection, Result};

pub fn insert_user(
    conn: &Connection,
    command: String,
    alias: String,
    info: String,
    service: String,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO commands (command, alias, info, service) VALUES (?1, ?2, ?3, ?4)",
        params![command, alias, info, service], // Use params! macro for better readability
    )?;
    Ok(())
}

pub fn display_commands(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut data = conn.prepare("SELECT * FROM commands")?;
    let data_iter = data.query_map([], |row| {
        Ok(Command {
            id: row.get(0)?,
            command: row.get(1)?,
            alias: row.get(2)?,
            info: row.get(2)?,
            service: row.get(2)?,
        })
    })?;

    for person in data_iter {
        println!("{:?}", person.unwrap());
    }
    Ok(())
}
