use entities::todo_events::TodoCreatedEvent;

pub struct GetTodoUsecase<'a> {
    get_todo_events: &'a mut dyn GetTodoEvents,
}

impl<'a> GetTodoUsecase<'a> {
    pub fn execute(self) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
        self.get_todo_events.get_all()
    }
    pub fn new(get_todo_events: &'a mut dyn GetTodoEvents) -> Self {
        Self { get_todo_events }
    }
}

pub trait GetTodoEvents {
    fn get_all(&mut self) -> Result<Vec<TodoCreatedEvent>, std::io::Error>;
}

#[cfg(test)]
mod tests {
    use super::GetTodoUsecase;
    use crate::get_todo_usecase::GetTodoEvents;
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
        fn save(&mut self, todo_events: Vec<TodoCreatedEvent>) -> Result<(), std::io::Error> {
            self.real_event_store.save(todo_events)
        }
    }

    impl GetTodoEvents for TestEventStore {
        fn get_all(&mut self) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
            self.real_event_store.get_all()
        }
    }

    #[test]
    fn given_todoevent_when_stored_then_ok() {
        let mut eventstore = TestEventStore::new(String::from("simple_plan_get_todo_usecase"));
        let todo_id = Uuid::new_v4();
        let plannables = vec![TodoCreatedEvent {
            event_id: Uuid::new_v4(),
            todo_id,
            sequence: 0,
            title: String::from("Read rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(9, 10, 11)
                    .unwrap(),
            ),
        }];
        let result = eventstore.save(plannables.clone());
        assert!(result.is_ok());
        let usecase = GetTodoUsecase {
            get_todo_events: &mut eventstore,
        };
        let result = usecase.execute().unwrap();
        assert_eq!(result, plannables);
    }
}
