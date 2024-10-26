use clap::Parser;
use cli::{CliArgs, SmritiCli};
use database::{create_conn, display_commands, insert_user};
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn: Connection = create_conn()?;

    let cli = SmritiCli::parse();

    match cli.command {
        CliArgs::Add(add_command) => {
            println!("Adding command: {:?}", add_command);
            match insert_user(
                &conn,
                add_command.command,
                add_command.alias,
                add_command.info.unwrap(),
                add_command.service,
            ) {
                Ok(()) => println!("Saved successfully!"),
                Err(err) => eprintln!("Error inserting data: {}", err),
            }
        }
        CliArgs::View(_view_command) => {
            display_commands(&conn)?;
        }
        _ => {
            println!("Command not implemented yet");
            // default logic or placeholder for other commands
        }
    }

    Ok(())
}
