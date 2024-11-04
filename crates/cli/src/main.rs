use clap::Parser;
use cli::{CliArgs, SmritiCli};
use database::{
    create_conn, display_by_type, display_commands, insert_command, retrieve_command,
    retrieve_command_by_alias, retrieve_commands_by_service,
};
use rusqlite::{Connection, Result};
mod shell_executor;
use cli_table::{Cell, CellStruct, Style, Table, TableDisplay};
use colored::Colorize;

pub fn create_table_header(table: Vec<Vec<CellStruct>>) -> TableDisplay {
    table
        .table()
        .title(vec![
            "Id".cyan().cell().bold(true),
            "Alias".cyan().cell().bold(true),
            "Command".cyan().cell().bold(true),
            "Info".cyan().cell().bold(true),
            "Service".cyan().cell().bold(true),
        ])
        .display()
        .unwrap()
}

fn main() -> Result<()> {
    let conn: Connection = create_conn()?;

    let cli = SmritiCli::parse();

    match cli.command {
        CliArgs::Run(arg) => match retrieve_command(&conn, &arg.alias) {
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
                                command.alias.cell(),
                                command.command.cell(),
                                command.info.cell(),
                                command.service.cell(),
                            ]);
                        }

                        let table_display = create_table_header(table);

                        println!("{}", table_display);
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else if view_command.alias {
                match display_by_type(&conn, "alias") {
                    Ok(aliases) => {
                        let mut table = vec![];
                        for alias in aliases {
                            table.push(vec![alias.cell()]);
                        }

                        let table_display = table
                            .table()
                            .title(vec!["Aliases"
                                .cyan()
                                .cell()
                                .bold(true)
                                .justify(cli_table::format::Justify::Center)])
                            .display()
                            .unwrap();

                        println!("{}", table_display);
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else if view_command.service {
                match display_by_type(&conn, "service") {
                    Ok(aliases) => {
                        let mut table = vec![];
                        for alias in aliases {
                            table.push(vec![alias.cell()]);
                        }

                        let table_display = table
                            .table()
                            .title(vec!["Services"
                                .red()
                                .cell()
                                .bold(true)
                                .justify(cli_table::format::Justify::Center)])
                            .display()
                            .unwrap();

                        println!("{}", table_display);
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else {
                println!("No valid options provided for view command");
            }
        }

        CliArgs::Search(show_command) => {
            // Check if at least one of the options is provided
            if show_command.alias.is_some() || show_command.service.is_some() {
                // Handle alias if provided
                if let Some(ref alias) = show_command.alias {
                    match retrieve_command_by_alias(&conn, alias) {
                        Ok(command) => {
                            let mut table = vec![];
                            table.push(vec![
                                command.id.cell(),
                                command.alias.cell(),
                                command.command.cell(),
                                command.info.cell(),
                                command.service.cell(),
                            ]);
                            let table_display = create_table_header(table);
                            println!("{}", table_display);
                        }
                        Err(e) => {
                            eprintln!(
                                "Error retrieving command: {} \nNo command associated with alias: {:?}",
                                e, alias
                            );
                        }
                    }
                }

                // Handle service if provided
                if let Some(ref service) = show_command.service {
                    match retrieve_commands_by_service(&conn, service) {
                        Ok(commands) => {
                            let mut table = vec![];
                            for command in commands {
                                table.push(vec![
                                    command.id.cell(),
                                    command.alias.cell(),
                                    command.command.cell(),
                                    command.info.cell(),
                                    command.service.cell(),
                                ]);
                            }
                            // let table_display = create_table_header(table);
                            // If we have results in the table, display them
                            if !table.is_empty() {
                                let table_display = create_table_header(table);
                                println!("\n{}", table_display);
                            } else {
                                println!(
                                    "{}",
                                    format!(
                                        "No commands found for the provided '{}' service.",
                                        service
                                    )
                                    .red()
                                );
                            }
                            // println!("{}", table_display);
                        }
                        Err(e) => {
                            eprintln!("{}",
                                format!("Error retrieving command: {} \nNo command associated with service: {:?}",
                                e, service
                            ).red());
                        }
                    }
                }

                // // Create the table with the collected data
                // let table_display = create_table_header(table);
                // println!("\n{}", table_display);
            } else {
                println!(
                    "{}",
                    format!("No valid options provided for view command. Please provide either --alias or --service.").red()
                );
            }
        }

        _ => {
            println!("{}", format!("Command not implemented yet").red());
            // default logic or placeholder for other commands}
        }
    }

    Ok(())
}
