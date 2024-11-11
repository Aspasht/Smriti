use colored::Colorize;
use regex::Regex;
use std::error::Error;
use std::{collections::HashMap, process};
use subprocess::{Exec, Redirection};

pub fn replace_placeholder(command: &str, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    let pattern = r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}"; // match placeholders starts with {}
    let re = Regex::new(pattern).unwrap();

    let mut placeholder_map: HashMap<String, String> = HashMap::new();

    // collect placeholders from the command
    let mut placeholders: Vec<String> = Vec::new();
    for cap in re.captures_iter(command) {
        let placeholder = cap[0].to_string();
        if !placeholders.contains(&placeholder) {
            placeholders.push(placeholder); // add unique placeholders
        }
    }

    // if there are no placeholders, return the original command
    if placeholders.is_empty() {
        return Ok(command.to_string());
    }

    if placeholders.len() > args.len() {
        return Err(
            "Not enough arguments to replace all placeholders.\nuse smriti search -a youralias"
                .into(),
        );
    }

    // mapping each placeholder to an argument
    for (i, placeholder) in placeholders.iter().enumerate() {
        placeholder_map.insert(placeholder.clone(), args[i].clone());
    }

    // replace placeholders in the command with their corresponding arguments
    let mut modified_command = command.to_string();
    for (placeholder, replacement) in placeholder_map {
        modified_command = modified_command.replace(&placeholder, &replacement);
    }

    Ok(modified_command)
}

#[allow(dead_code)]
pub fn execute_command(command: String) -> Result<(), Box<dyn Error>> {
    println!("{}", format!("{}", command).green());
    let result = Exec::shell(command)
        .stdout(Redirection::Pipe) // Pipe stdout to capture output
        .stderr(Redirection::Pipe) // Pipe stderr to capture errors
        .capture()?; // Capture the output and wait for the command to complete

    // Print stdout and stderr
    if !result.stdout.is_empty() {
        println!("Output:\n{}", String::from_utf8_lossy(&result.stdout));
    }
    if !result.stderr.is_empty() {
        eprintln!("Error:\n{}", String::from_utf8_lossy(&result.stderr).red());
    }

    // Check if the command was successful
    if result.success() {
        // println!("Command executed successfully");
        process::exit(0);
    } else {
        eprintln!(
            "{}",
            format!("Command failed with status: {:?}", result.exit_status).red()
        );
    }

    Ok(())
}
