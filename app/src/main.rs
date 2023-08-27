use anyhow::Result;
use chrono::NaiveDateTime;
use cli_app::AddTodoUseCaseInvoker;
use cli_app::GetTodoUseCaseInvoker;
use entities::todo_events::TodoCreatedEvent;
use event_store::plannable_event_store::TodoEventStore;
use usecases::add_todo_usecase::AddTodoUsecase;
use usecases::add_todo_usecase::StoreTodoEvents;
use usecases::get_todo_usecase::GetTodoEvents;

use cli_app::CliApp;
use usecases::get_todo_usecase::GetTodoUsecase;

struct AppEventStore {
    real_event_store: TodoEventStore,
}

impl AppEventStore {
    fn new(database_url: &str) -> Self {
        let eventstore = TodoEventStore::new(database_url).unwrap();
        Self {
            real_event_store: eventstore,
        }
    }
}
impl StoreTodoEvents for AppEventStore {
    fn save(&mut self, todo_events: Vec<TodoCreatedEvent>) -> Result<(), std::io::Error> {
        self.real_event_store.save(todo_events)
    }
}

impl GetTodoEvents for AppEventStore {
    fn get_all(&mut self) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
        self.real_event_store.get_all()
    }
}

fn main() -> Result<()> {
    App::new().run()
}

struct App {
    app_event_store: AppEventStore,
}

impl App {
    pub fn new() -> Self {
        let database_url = "/tmp/test_plannable_events.db";
        let app_event_store = AppEventStore::new(database_url);

        Self { app_event_store }
    }

    pub fn run(&mut self) -> Result<()> {
        CliApp::new(self).run()
    }
}

impl AddTodoUseCaseInvoker for App {
    fn invoke_add_todo_usecase(&mut self, title: String, end_date: Option<NaiveDateTime>) {
        AddTodoUsecase::new(&mut self.app_event_store)
            .execute(title, end_date)
            .unwrap();
    }
}
impl GetTodoUseCaseInvoker for App {
    fn invoke_get_todo_usecase(&mut self) {
        GetTodoUsecase::new(&mut self.app_event_store)
            .execute()
            .unwrap();
    }
}
