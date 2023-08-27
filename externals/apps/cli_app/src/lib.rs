use anyhow::Result;
use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};

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
    Get,
}

fn parse_duration(date: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    let parse_from_str = NaiveDateTime::parse_from_str;
    let end_date = parse_from_str(date, "%Y-%m-%d %H:%M:%S");
    end_date
}

pub struct CliApp<'a> {
    add_todo_usecase_invoker: &'a mut dyn AddTodoUseCaseInvoker,
}

impl<'a> CliApp<'a> {
    pub fn new(add_todo_usecase_invoker: &'a mut impl AddTodoUseCaseInvoker) -> Self {
        Self {
            add_todo_usecase_invoker,
        }
    }

    pub fn run(self) -> Result<()> {
        let cli = Cli::parse();

        match &cli.command {
            Command::Todo(TodoCli { command }) => match command {
                TodoCommand::Add { title, end_date } => {
                    println!("{:?}", title);
                    println!("{:?}", end_date);
                    self.add_todo_usecase_invoker
                        .invoke(title.to_string(), *end_date);
                }
                TodoCommand::Get => {
                    todo!()
                } //                TodoCommand::Get => {
                  //                    let database_url = "/tmp/test_plannable_events.db";
                  //                    let mut clieventstore = CliEventStore::new(database_url);
                  //                    let usecase = GetTodoUsecase::new(&mut clieventstore);
                  //                    let result = usecase.execute().unwrap();
                  //                    println!("{:?}", result);
                  //                }
            },
        }
        Ok(())
    }
}

pub trait AddTodoUseCaseInvoker {
    fn invoke(&mut self, title: String, end_date: Option<NaiveDateTime>);
}