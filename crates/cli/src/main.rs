use clap::Parser;
use cli::{CliArgs, SmritiCli};
use database::{create_conn, display_by_type, display_commands, insert_command, retrieve_command};
use rusqlite::{Connection, Result};
mod shell_executor;
use cli_table::{Cell, Style, Table};

fn main() -> Result<()> {
    let conn: Connection = create_conn()?;

    let cli = SmritiCli::parse();

    match cli.command {
        CliArgs::Run(arg) => match retrieve_command(&conn, &arg.command) {
            Ok(command) => {
                println!("{}", command);
                if let Err(e) = shell_executor::execute_command(&command) {
                    eprintln!("Error executing command: {}", e);
                }
            }
            Err(e) => eprintln!("Error retrieving command: {}", e),
        },

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
                match display_commands(&conn) {
                    Ok(commands) => {
                        let mut table = vec![];
                        for command in commands {
                            table.push(vec![
                                command.id.cell(),
                                command.command.cell(),
                                command.alias.cell(),
                                command.info.cell(),
                                command.service.cell(),
                            ]);
                        }

                        let table_display = table
                            .table()
                            .title(vec![
                                "Id".cell().bold(true),
                                "Command".cell().bold(true),
                                "Alias".cell().bold(true),
                                "Info".cell().bold(true),
                                "Service".cell().bold(true),
                            ])
                            .display()
                            .unwrap();

                        println!("{}", table_display);
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else if view_command.alias {
                display_by_type(&conn, "alias")?;
            } else if view_command.service {
                display_by_type(&conn, "service")?;
            } else {
                println!("No valid options provided for view command");
            }
        }
        _ => {
            println!("Command not implemented yet");
            // default logic or placeholder for other commands}
        }
    }

    Ok(())
}
