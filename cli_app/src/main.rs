use anyhow::Result;
use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use usecases::add_todo_usecase::AddTodoUsecase;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Todo(TodoCli),
}

#[derive(Parser, Debug)]
struct TodoCli {
    #[command(subcommand)]
    command: TodoCommand,
}

#[derive(Subcommand, Debug)]
enum TodoCommand {
    Add {
        title: String,
        #[arg(value_parser = parse_duration)]
        end_date: Option<NaiveDateTime>,
    },
}

fn parse_duration(date: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    let parse_from_str = NaiveDateTime::parse_from_str;
    let end_date = parse_from_str(date, "%Y-%m-%d %H:%M:%S");
    end_date
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Todo(TodoCli { command }) => match command {
            TodoCommand::Add { title, end_date } => {
                println!("{:?}", title);
                println!("{:?}", end_date);
            }
        },
    }
    Ok(())
}
