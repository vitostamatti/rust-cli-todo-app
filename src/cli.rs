use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// This subcommands allows to create, delete, update and list the Tasks.
    Tasks(TasksSubcommand),
}

#[derive(Args, Debug)]
pub struct TasksSubcommand {
    /// Path to file to save and load task from. If not provided
    /// a tasks.json file is created in the current directory.
    pub file: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<TaskCommands>,
}

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// List Tasks with the all its information in stdout.
    List(ListTasksArgs),
    /// Create new Tasks providing name, text, and status.
    Create(CreateTaskArgs),
    /// Update existing Tasks providing new name, text or status.
    Update(UpdateTaskArgs),
    /// Delete existing Task by providing its index.
    Delete(DeleteTaskArgs),
}

#[derive(Args, Debug)]
pub struct CreateTaskArgs {
    /// The name of the Task
    pub name: String,
    #[arg(short, long)]
    /// The text description of the Task
    pub text: Option<String>,
    /// The Status of the Task
    #[arg(short, long, value_enum)]
    pub status: Option<StatusArg>,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StatusArg {
    Doing,
    Done,
    Pending,
}

#[derive(Args, Debug)]
pub struct UpdateTaskArgs {
    /// The index of the task to be deleted
    pub id: usize,
    #[arg(short, long)]
    /// New task name to be updated
    pub name: Option<String>,
    #[arg(short, long)]
    /// New task text to be updated
    pub text: Option<String>,
    #[arg(short, long, value_enum)]
    /// New task status to be updated
    pub status: Option<StatusArg>,
}

#[derive(Args, Debug)]
pub struct DeleteTaskArgs {
    /// The index of the task to be deleted
    pub id: usize,
}

#[derive(Args, Debug)]
pub struct ListTasksArgs {
    /// Maximum number of list to show
    #[arg(short, long)]
    pub number: Option<usize>,
}
