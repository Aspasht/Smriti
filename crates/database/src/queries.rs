use crate::models::Command;
use rusqlite::{params, Connection, Result};

pub fn insert_command(
    conn: &Connection,
    command: &str,
    alias: &str,
    info: &str,
    service: &str,
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
            id: row.get(0)?,
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

pub fn display_by_type(
    conn: &Connection,
    search_type: &str,
) -> Result<Vec<String>, rusqlite::Error> {
    let query = format!("SELECT {} FROM commands", search_type);
    let mut stmt = conn.prepare(&query)?;
    let data_iter = stmt.query_map([], |row| row.get(0))?;

    // Collect results into a Vec<String>
    let results: Vec<String> = data_iter.filter_map(Result::ok).collect();

    Ok(results)
}

pub fn retrieve_command(conn: &Connection, alias: &str) -> Result<String, rusqlite::Error> {
    let query = "SELECT command FROM commands WHERE alias = ?1";
    let mut stmt = conn.prepare(query)?;

    // Use `query_row` with `optional` to return `Option<String>`
    let result = stmt.query_row([alias], |row| row.get(0))?;

    Ok(result) // Returns Some(command) if found, or None if no match
}

pub fn retrieve_command_by_alias(
    conn: &Connection,
    alias: &str,
) -> Result<Command, rusqlite::Error> {
    let query = "SELECT id, command, alias, info, service FROM commands WHERE alias = ?1";
    let mut stmt = conn.prepare(query)?;
    let command = stmt.query_row([alias], |row| {
        Ok(Command {
            id: row.get(0)?,
            command: row.get(1)?,
            alias: row.get(2)?,
            info: row.get(3)?,
            service: row.get(4)?,
        })
    })?;

    Ok(command)
}

pub fn retrieve_commands_by_service(
    conn: &Connection,
    service: &str,
) -> Result<Vec<Command>, rusqlite::Error> {
    let query = "SELECT id, command, alias, info, service FROM commands WHERE service = ?1";
    let mut stmt = conn.prepare(query)?;

    let data_iter = stmt.query_map([service], |row| {
        Ok(Command {
            id: row.get(0)?,
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

pub fn delete_by_alias(conn: &Connection, alias: &str) -> Result<(), rusqlite::Error> {
    let query = "DELETE FROM commands WHERE alias = ?1";
    conn.execute(query, &[alias])?;
    Ok(())
}

pub fn delete_by_service(conn: &Connection, service: &str) -> Result<(), rusqlite::Error> {
    let query = "DELETE FROM commands WHERE service = ?1";
    conn.execute(query, &[service])?;
    Ok(())
}

pub fn update_command_by_alias(
    conn: &Connection,
    alias: &str,
    value: &str,
) -> Result<(), rusqlite::Error> {
    let query = "UPDATE commands SET command = ?2 WHERE alias = ?1";
    conn.execute(query, &[alias, value])?;
    Ok(())
}

pub fn update_service_by_alias(
    conn: &Connection,
    alias: &str,
    value: &str,
) -> Result<(), rusqlite::Error> {
    let query = "UPDATE commands SET service = ?2 WHERE alias = ?1";
    conn.execute(query, &[alias, value])?;
    Ok(())
}

pub fn update_info_by_alias(
    conn: &Connection,
    alias: &str,
    value: &str,
) -> Result<(), rusqlite::Error> {
    let query = "UPDATE commands SET info = ?2 WHERE alias = ?1";
    conn.execute(query, &[alias, value])?;
    Ok(())
}
