use std::error::Error;

use clap::Parser;

use manager::{find_default_journal_file, TasksManager};

use crate::cli::{Cli, Commands, TaskCommands, TasksSubcommand};

mod cli;
mod manager;
mod task;

fn handle_tasks_subcommands(task_commands: TasksSubcommand) -> Result<(), Box<dyn Error>> {
    let file = task_commands
        .file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file.");

    let mut manager = TasksManager::new(file);

    match task_commands.command {
        Some(TaskCommands::Create(args)) => manager.create_task(args),
        Some(TaskCommands::List(_)) => manager.list_tasks(),
        Some(TaskCommands::Delete(args)) => manager.delete_task(args.id),
        Some(TaskCommands::Update(args)) => manager.update_task(args),
        _ => Ok(()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Tasks(task_commands)) => handle_tasks_subcommands(task_commands),
        _ => Ok(()),
    }
}
