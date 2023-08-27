use chrono::NaiveDateTime;
use cli_app::AddTodoUseCaseInvoker;
use entities::todo_events::TodoCreatedEvent;
use event_store::plannable_event_store::TodoEventStore;
use usecases::add_todo_usecase::AddTodoUsecase;
use usecases::add_todo_usecase::StoreTodoEvents;
use usecases::get_todo_usecase::GetTodoEvents;
use usecases::get_todo_usecase::GetTodoUsecase;

use cli_app::CliApp;

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

fn main() {
    App::new().run();
}

struct App {
    app_event_store: AppEventStore,
}

impl App {
    pub fn new() -> Self {
        let database_url = "/tmp/test_plannable_events.db";
        let mut app_event_store = AppEventStore::new(database_url);

        Self { app_event_store }
    }

    pub fn run(&mut self) {
        CliApp::new(self).run();
    }
}

impl AddTodoUseCaseInvoker for App {
    fn invoke(&mut self, title: String, end_date: Option<NaiveDateTime>) {
        AddTodoUsecase::new(&mut self.app_event_store).execute(title, end_date);
    }
}
