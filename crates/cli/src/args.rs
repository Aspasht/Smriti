use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "smriti")]
#[clap(about = "Keep track of the commands your memory missed.")]
#[clap(version = "1.0")]

pub struct SmritiCli {
    #[clap(subcommand)]
    pub command: CliArgs,
}

#[derive(Debug, Subcommand)]
pub enum CliArgs {
    /// Execute a saved command.
    Run(RunCommand),
    /// Add a new command to the saved list.
    Add(AddCommand),
    /// Remove an existing command, alias, or group.
    Delete(DeleteCommand),
    /// Modify an existing command, alias, or group.
    Update(UpdateCommand),
    /// Display a list of all saved commands, aliases, or groups.
    View(ViewCommand),
    /// Find the command associated with a specific alias or service.
    Search(SearchCommand),
}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(value_name = "ALIAS")]
    pub alias: String,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// Command to save
    #[arg(short, long)]
    pub command: String,
    /// Command alias
    #[arg(short, long)]
    pub alias: String,
    /// Command description
    #[arg(short, long)]
    pub info: Option<String>, // Optional, update only if provided
    /// Groups command according to different type or stack
    #[arg(short, long)]
    pub service: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    /// Deletes existing command
    #[arg(short, long)]
    pub command: String,
    /// Deletes existing command by their alias
    #[arg(short, long)]
    pub alias: String,
    /// Deletes serviceed command
    #[arg(short, long)]
    pub service: String,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// Update existing commands
    #[arg(short, long)]
    pub command: String,
    /// Updating existing command alias
    #[arg(short, long)]
    pub alias: String,
    /// Update existing command description
    #[arg(short, long)]
    pub info: Option<String>, // Optional, update only if provided
    /// Update command service
    #[arg(short, long)]
    pub service: String,
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    /// Dispalys all saved commands
    #[arg(long)]
    pub all: bool,
    /// Display all available aliases
    #[arg(short, long)]
    pub alias: bool,
    /// Display saved commands by their service
    #[arg(short, long)]
    pub service: bool,
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    /// Search by alias
    #[arg(long, short)]
    pub alias: Option<String>,
    /// Search by service
    #[arg(long, short)]
    pub service: Option<String>,
}
