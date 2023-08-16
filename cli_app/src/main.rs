use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Todo(TodoCli),
}

#[derive(Parser)]
struct TodoCli {
    #[command(subcommand)]
    command: TodoCommand,
}

#[derive(Subcommand)]
enum TodoCommand {
    Add { title: String },
}

fn main() -> Result<()> {
    let _args = Cli::parse();
    Ok(())
}
