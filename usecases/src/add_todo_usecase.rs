use chrono::NaiveDateTime;
use entities::todo_events::TodoCreatedEvent;
use uuid::Uuid;
struct AddTodoUsecase<'a> {
    store_todo_events: &'a mut dyn StoreTodoEvents,
}

impl AddTodoUsecase<'_> {
    fn execute(self, title: String, end_date: Option<NaiveDateTime>) -> Result<(), std::io::Error> {
        let todo = vec![TodoCreatedEvent {
            title,
            end_date,
            todo_id: Uuid::new_v4(),
            event_id: Uuid::new_v4(),
            sequence: 0,
        }];
        self.store_todo_events.store(todo)
    }
}

pub trait StoreTodoEvents {
    fn store(&mut self, todo_events: Vec<TodoCreatedEvent>) -> Result<(), std::io::Error>;
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::AddTodoUsecase;
    use crate::add_todo_usecase::StoreTodoEvents;
    use chrono::NaiveDate;
    use entities::todo_events::TodoCreatedEvent;
    use event_store::plannable_event_store::TodoEventStore;
    use uuid::Uuid;

    //Used for testing to call store functions
    struct TestEventStore {
        real_event_store: TodoEventStore,
    }

    impl TestEventStore {
        fn new(name: String) -> Self {
            let database_url = format!("/tmp/{}.db", name);
            let eventstore = TodoEventStore::clean(&database_url).unwrap();
            Self {
                real_event_store: eventstore,
            }
        }
        fn get_all_todos(&mut self) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
            self.real_event_store.get_all()
        }
    }

    impl StoreTodoEvents for TestEventStore {
        fn store(&mut self, todo_events: Vec<TodoCreatedEvent>) -> Result<(), std::io::Error> {
            self.real_event_store.save(todo_events)
        }
    }

    #[test]
    fn given_todoevent_when_stored_then_ok() {
        let mut eventstore = TestEventStore::new(String::from("testtodousecase"));
        let usecase = AddTodoUsecase {
            store_todo_events: &mut eventstore,
        };
        let end_date = Some(
            NaiveDate::from_ymd_opt(2023, 9, 29)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap(),
        );
        let title = String::from("Read rust book");
        let result = usecase.execute(title, end_date);
        assert!(result.is_ok());
        let result = eventstore.get_all_todos();
        assert!(result.is_ok());
    }
}
