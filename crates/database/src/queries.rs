use crate::models::Command;
use rusqlite::{params, Connection, Result};

pub fn insert_command(
    conn: &Connection,
    command: String,
    alias: String,
    info: String,
    service: String,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO commands (command, alias, info, service) VALUES (?1, ?2, ?3, ?4)",
        params![command, alias, info, service],
    )?;
    Ok(())
}

pub fn display_commands(conn: &Connection) -> Result<Vec<Command>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM commands")?;

    let data_iter = stmt.query_map([], |row| {
        Ok(Command {
            id: row.get(0)?, // Adjust indices to match database column order
            command: row.get(1)?,
            alias: row.get(2)?,
            info: row.get(3)?,
            service: row.get(4)?,
        })
    })?;

    // Collect the iterator into a Vec<Command>
    let commands: Vec<Command> = data_iter.filter_map(Result::ok).collect();

    Ok(commands)
}

pub fn display_by_type(conn: &Connection, search_type: &str) -> Result<(), rusqlite::Error> {
    let query = format!("SELECT {} FROM commands", search_type);
    let mut stmt = conn.prepare(&query)?;
    let data_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for data in data_iter {
        println!("{}", data.unwrap());
    }
    Ok(())
}

pub fn retrieve_command(conn: &Connection, alias: &str) -> Result<String, rusqlite::Error> {
    // Use a parameterized query to safely retrieve the command by alias
    let query = "SELECT command FROM commands WHERE alias = ?";
    let mut stmt = conn.prepare(query)?;

    // Use query_row to retrieve a single value based on the alias
    stmt.query_row([alias], |row| row.get::<_, String>(0))
        .map_err(|e| e) // Map the error to the Result's type
}
