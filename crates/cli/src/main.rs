use clap::Parser;
use cli::{CliArgs, SmritiCli};
use database::{create_conn, display_by_type, display_commands, insert_command};
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn: Connection = create_conn()?;

    let cli = SmritiCli::parse();

    match cli.command {
        CliArgs::Add(add_command) => {
            println!("Adding command: {:?}", add_command);
            match insert_command(
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
        CliArgs::View(view_command) => {
            if view_command.all {
                display_commands(&conn)?;
            } else if view_command.alias {
                display_by_type(&conn, "alias".into())?;
            } else if view_command.service {
                display_by_type(&conn, "service".into())?;
            } else {
                println!("No valid options provided for view command");
            }
        }
        _ => {
            println!("Command not implemented yet");
            // default logic or placeholder for other commands
        }
    }

    Ok(())
}
