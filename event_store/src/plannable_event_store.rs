use crate::models::PlannableEventRow;
use crate::plannable_events_repository::PlannableEventsRepository;
use entities::todo_events::TodoCreatedEvent;
use std::io::Error;
use std::io::ErrorKind;
use uuid::Uuid;

pub struct TodoEventStore {
    repository: PlannableEventsRepository,
}

impl TodoEventStore {
    pub fn new(database_url: &str) -> Result<Self, std::io::Error> {
        let repository = PlannableEventsRepository::initialize(database_url)?;
        Ok(Self { repository })
    }

    //#[cfg(test)]
    pub fn clean(database_url: &str) -> Result<Self, std::io::Error> {
        let mut instance = Self::new(database_url)?;
        instance
            .repository
            .drop_table()
            .map_err(|_error| Error::new(ErrorKind::Other, "Drop Table failed"))?;
        instance
            .repository
            .create_table()
            .map_err(|_error| Error::new(ErrorKind::Other, "Create table failed"))?;
        Ok(instance)
    }

    pub fn save(
        &mut self,
        todocreatedsevents: Vec<TodoCreatedEvent>,
    ) -> Result<(), std::io::Error> {
        let rows: Vec<PlannableEventRow> = todocreatedsevents
            .into_iter()
            .map(|event| event.into())
            .collect();
        self.repository
            .save(rows)
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))
    }

    pub fn read(&mut self, todo_id: Uuid) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
        let rows = self
            .repository
            .read(&todo_id.to_string())
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    pub fn get_all(&mut self) -> Result<Vec<TodoCreatedEvent>, std::io::Error> {
        let rows = self
            .repository
            .get_all()
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}

#[cfg(test)]
mod tests {

    use crate::plannable_event_store::TodoEventStore;
    use chrono::NaiveDate;
    use entities::todo_events::TodoCreatedEvent;
    use uuid::Uuid;

    #[test]
    fn given_todocreatedevent_when_savetorepository_then_repositoryhasoneentry() {
        let database_url = "/tmp/simple_plan_savetodoevent.db";
        let mut eventstore = TodoEventStore::clean(database_url).unwrap();

        let plannables = vec![TodoCreatedEvent {
            event_id: Uuid::new_v4(),
            todo_id: Uuid::new_v4(),
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
        let result = eventstore.get_all().unwrap();
        assert_eq!(result, plannables);
    }

    #[test]
    fn given_todoid_when_read_then_returnalleventsforthetodoid() {
        let database_url = "/tmp/simple_plan_readtodoevent.db";
        let mut eventstore = TodoEventStore::clean(database_url).unwrap();
        let todo_id = Uuid::new_v4();
        let todo_created = TodoCreatedEvent {
            event_id: Uuid::new_v4(),
            todo_id: todo_id.clone(),
            sequence: 0,
            title: String::from("Buy rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        };
        let todo_changed = TodoCreatedEvent {
            event_id: Uuid::new_v4(),
            todo_id: todo_id.clone(),
            sequence: 1,
            title: String::from("Read rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        };
        let plannables = vec![todo_created, todo_changed];
        eventstore.save(plannables.clone()).unwrap();
        let result = eventstore.read(todo_id).unwrap();
        assert_eq!(result, plannables);
    }

    #[test]
    fn given_todoid_when_doesnotexist_then_returnnoevents() {
        let database_url = "/tmp/simple_plan_readnoevent.db";
        let mut eventstore = TodoEventStore::clean(database_url).unwrap();
        let todo_id = Uuid::new_v4();
        let result = eventstore.read(todo_id).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn given_existingtodoid_when_savemultiplewithduplicate_then_rollback() {
        //GIVEN initial event
        let database_url = "/tmp/simple_plan_adduplicaterollback.db";
        let mut eventstore = TodoEventStore::clean(database_url).unwrap();
        let todo_id = Uuid::new_v4();
        let todo_created = vec![TodoCreatedEvent {
            event_id: Uuid::new_v4(),
            todo_id: todo_id.clone(),
            sequence: 0,
            title: String::from("Buy rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        }];
        eventstore.save(todo_created.clone()).unwrap();
        let todo_changed = vec![
            TodoCreatedEvent {
                event_id: Uuid::new_v4(),
                todo_id: todo_id.clone(),
                sequence: 1,
                title: String::from("Read rust book"),
                end_date: Some(
                    NaiveDate::from_ymd_opt(2023, 9, 29)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            },
            TodoCreatedEvent {
                event_id: Uuid::new_v4(),
                todo_id: todo_id.clone(),
                sequence: 0,
                title: String::from("Sell rust book"),
                end_date: Some(
                    NaiveDate::from_ymd_opt(2023, 9, 29)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            },
        ];

        //WHEN Save duplicate, the combination of sequence and todo_id has to be unique, DB
        //constraint
        let result = eventstore.save(todo_changed);
        //THEN error
        assert!(result.is_err());
        let read_todo_created = eventstore.read(todo_id).unwrap();
        //Test rollback of THEN by comparing the output to the original DB insertion to test rollback
        assert_eq!(read_todo_created, todo_created);
    }
}
