use anyhow::Result;
use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use entities::todo_events::TodoCreatedEvent;
use event_store::plannable_event_store::TodoEventStore;
use usecases::add_todo_usecase::AddTodoUsecase;
use usecases::add_todo_usecase::StoreTodoEvents;
use usecases::get_todo_usecase::GetTodoEvents;
use usecases::get_todo_usecase::GetTodoUsecase;

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

struct CliEventStore {
    real_event_store: TodoEventStore,
}

impl CliEventStore {
    fn new(database_url: &str) -> Self {
        let eventstore = TodoEventStore::new(database_url).unwrap();
        Self {
            real_event_store: eventstore,
        }
    }
}
impl StoreTodoEvents for CliEventStore {
    fn save(&mut self, todo_events: Vec<TodoCreatedEvent>) -> Result<(), std::io::Error> {
        self.real_event_store.save(todo_events)
    }
}

impl GetTodoEvents for CliEventStore {
    fn get_all(&mut self) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
        self.real_event_store.get_all()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Todo(TodoCli { command }) => match command {
            TodoCommand::Add { title, end_date } => {
                println!("{:?}", title);
                println!("{:?}", end_date);
                let database_url = "/tmp/test_plannable_events.db";
                let mut clieventstore = CliEventStore::new(database_url);
                let usecase = AddTodoUsecase::new(&mut clieventstore);
                let result = usecase.execute(title.to_string(), *end_date);
                if result.is_ok() {
                    println!("Todo Add Complete");
                }
            }
            TodoCommand::Get => {
                let database_url = "/tmp/test_plannable_events.db";
                let mut clieventstore = CliEventStore::new(database_url);
                let usecase = GetTodoUsecase::new(&mut clieventstore);
                let result = usecase.execute().unwrap();
                println!("{:?}", result);
            }
        },
    }
    Ok(())
}
