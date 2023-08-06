use crate::models::PlannableEventRow;
use crate::plannable_events_repository::PlannableEventsRepository;
use entities::TodoCreatedEvent;
use std::io::Error;
use std::io::ErrorKind;

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
}

#[cfg(test)]
mod tests {

    use super::PlannableEventsRepository;
    use crate::{models::PlannableEventRow, plannable_event_store::TodoEventStore};
    use entities::TodoCreatedEvent;
    use std::result;
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
        }];
        let result = eventstore.save(plannables);
        assert!(result.is_ok());
    }
}
