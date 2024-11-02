use std::error::Error;
use subprocess::{Exec, Redirection};

pub fn execute_command(command: &str) -> Result<(), Box<dyn Error>> {
    // Execute the command using subprocess
    let result = Exec::shell(command)
        .stdout(Redirection::Pipe) // Pipe stdout to capture output
        .stderr(Redirection::Pipe) // Pipe stderr to capture errors
        .capture()?; // Capture the output and wait for the command to complete

    // Print stdout and stderr
    if !result.stdout.is_empty() {
        println!("Output:\n{}", String::from_utf8_lossy(&result.stdout));
    }
    if !result.stderr.is_empty() {
        eprintln!("Error:\n{}", String::from_utf8_lossy(&result.stderr));
    }

    // Check if the command was successful
    if result.success() {
        println!("Command executed successfully");
    } else {
        eprintln!("Command failed with status: {:?}", result.exit_status);
    }

    Ok(())
}
