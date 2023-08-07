use crate::models::PlannableEventRow;
use crate::plannable_events_repository::PlannableEventsRepository;
use entities::TodoCreatedEvent;
use std::io::Error;
use std::io::ErrorKind;
use uuid::Uuid;

pub struct TodoEventStore {
    repository: PlannableEventsRepository,
}

impl TodoEventStore {
    pub fn new(database_url: &str) -> Result<Self, std::io::Error> {
        let repository = PlannableEventsRepository::initialize(database_url)?;
        Ok(Self::new_internal(repository))
    }

    fn new_internal(repository: PlannableEventsRepository) -> Self {
        Self { repository }
    }

    fn save(&mut self, todocreatedsevents: Vec<TodoCreatedEvent>) -> Result<(), std::io::Error> {
        let rows: Vec<PlannableEventRow> = todocreatedsevents
            .into_iter()
            .map(|event| event.into())
            .collect();
        self.repository
            .save(rows)
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))
    }

    fn read(&mut self, todo_id: Uuid) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
        let rows = self
            .repository
            .read(&todo_id.to_string())
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}

#[cfg(test)]
mod tests {

    use super::PlannableEventsRepository;
    use crate::plannable_event_store::TodoEventStore;
    use entities::TodoCreatedEvent;
    use uuid::uuid;

    #[test]
    fn given_todocreatedevent_when_savetorepository_then_repositoryhasoneentry() {
        let database_url = "/tmp/simple_plan_savetodoevent.db";
        let mut repository = PlannableEventsRepository::initialize(database_url).unwrap();
        repository.drop_table().unwrap();
        repository.create_table().unwrap();

        let mut eventstore = TodoEventStore::new_internal(repository);

        let plannables = vec![TodoCreatedEvent {
            event_id: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            todo_id: uuid!("57e55044-10b1-426f-9247-bb680e5fe0c8"),
            sequence: 0,
            title: String::from(""),
            end_date: None,
        }];
        let result = eventstore.save(plannables);
        assert!(result.is_ok());
    }

    #[test]
    fn given_todoid_when_read_then_returnalleventsforthetodoid() {
        let database_url = "/tmp/simple_plan_readtodoevent.db";
        let mut repository = PlannableEventsRepository::initialize(database_url).unwrap();
        repository.drop_table().unwrap();
        repository.create_table().unwrap();

        let mut eventstore = TodoEventStore::new_internal(repository);

        let plannables = vec![TodoCreatedEvent {
            event_id: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            todo_id: uuid!("57e55044-10b1-426f-9247-bb680e5fe0c8"),
            sequence: 0,
            title: String::from("some title"),
            end_date: None,
        }];
        eventstore.save(plannables.clone());
        let todo_id = uuid!("57e55044-10b1-426f-9247-bb680e5fe0c8");
        let result = eventstore.read(todo_id).unwrap();
        assert_eq!(result, plannables);
    }
}
