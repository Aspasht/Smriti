use clap::Parser;
use cli::{CliArgs, SmritiCli};
use database::{
    create_conn, delete_by_alias, delete_by_service, display_by_type, display_commands,
    insert_command, rename_alias, retrieve_command, retrieve_command_by_alias,
    retrieve_commands_by_service, update_command_by_alias, update_info_by_alias,
    update_service_by_alias,
};
use rusqlite::{Connection, Result};
mod shell_executor;
use cli_table::{Cell, CellStruct, Style, Table, TableDisplay};
use colored::Colorize;

#[cfg(test)]
mod main_test;

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
                    eprintln!("{}", format!("Error executing command: {}", e).red());
                }
            }
            Err(err) => eprintln!("{}", format!("Error retrieving command: {}", err).red()),
        },

        CliArgs::Add(add_command) => {
            let add_args = &add_command;
            match insert_command(
                &conn,
                &add_args.command,
                &add_args.alias,
                add_args.info.as_ref().unwrap(),
                &add_args.service,
            ) {
                Ok(()) => {
                    match retrieve_command_by_alias(&conn, &add_args.alias) {
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
                            eprintln!("{}",format!(
                                "Error retrieving command: {} \nNo command associated with alias: {:?}",
                                e, &add_args.alias).red()
                            );
                        }
                    }
                    println!("Saved successfully!")
                }
                Err(err) => eprintln!(
                    "Error inserting data: {} ",
                    format!("{} \n Note: Command and Alias must be unique", err).red()
                ),
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
                    Err(err) => eprintln!("{}", format!("{}", err).red()),
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
                    Err(err) => eprintln!("{}", format!("{}", err).red()),
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
                            eprintln!("{}",format!(
                                "Error retrieving command: {} \nNo command associated with alias: {:?}",
                                e, alias).red()
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

        CliArgs::Delete(delete_command) => {
            if delete_command.alias.is_some() || delete_command.service.is_some() {
                if let Some(ref alias) = delete_command.alias {
                    match delete_by_alias(&conn, alias) {
                        Ok(()) => {
                            println!(
                                "Commands associated with {} alias is  deleted from the database",
                                alias
                            )
                        }
                        Err(e) => {
                            println!(
                                "{}",
                                format!("{}\n Couldn't delete command with alias {}", e, alias)
                                    .red()
                            );
                        }
                    }
                }

                if let Some(ref service) = delete_command.service {
                    match delete_by_service(&conn, service) {
                        Ok(()) => {
                            println!(
                                "Commands associated with {} service were deleted from the database",
                                service
                            )
                        }
                        Err(e) => {
                            println!(
                                "{}",
                                format!("{}\n Couldn't delete command with {} service", e, service)
                                    .red()
                            );
                        }
                    }
                }
            }
        }

        CliArgs::Update(update_command) => {
            if !update_command.alias.is_empty() {
                if update_command.command.is_some()
                    || update_command.info.is_some()
                    || update_command.service.is_some()
                {
                    if let Some(ref command) = update_command.command {
                        match update_command_by_alias(&conn, &update_command.alias, command) {
                            Ok(()) => match retrieve_command_by_alias(&conn, &update_command.alias)
                            {
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
                                    eprintln!("{}",format!(
                                            "Error retrieving command: {} \nNo command associated with alias: {:?}",
                                            e, &update_command.alias).red()
                                        );
                                }
                            },
                            Err(e) => {
                                println!(
                                    "{}",
                                    format!("{}\n Couldn't update at the moment", e).red()
                                );
                            }
                        }
                    }

                    if let Some(ref service) = update_command.service {
                        match update_service_by_alias(&conn, &update_command.alias, service) {
                            Ok(()) => {
                                match retrieve_command_by_alias(&conn, &update_command.alias) {
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
                                            e, &update_command.alias
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}",
                                    format!("{}\n Couldn't update at the moment", e).red()
                                );
                            }
                        }
                    }

                    if let Some(ref info) = update_command.info {
                        match update_info_by_alias(&conn, &update_command.alias, info) {
                            Ok(()) => {
                                match retrieve_command_by_alias(&conn, &update_command.alias) {
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
                                            e, &update_command.alias
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}",
                                    format!("{}\n Couldn't update at the moment", e).red()
                                );
                            }
                        }
                    }
                }
            } else {
                println!("{}", format!("Required -a flag not provided").red());
            }
        }

        CliArgs::Rename(rename_command) => {
            match rename_alias(&conn, &rename_command.alias, &rename_command.new_alias) {
                Ok(()) => match retrieve_command_by_alias(&conn, &rename_command.new_alias) {
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
                            "{}",
                            format!(
                                "Error retrieving alias: {} \nNo alias found with the name: {:?}",
                                e, &rename_command.new_alias
                            )
                            .red()
                        );
                    }
                },
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "{}\n Unable to complete the rename process. Please verify your commands.",
                            e
                        )
                        .red()
                    );
                }
            }
        } // _ => {
          //     println!("{}", format!("Command not implemented yet").red());
          //     // default logic or placeholder for other commands}
          // }
    }

    Ok(())
}
