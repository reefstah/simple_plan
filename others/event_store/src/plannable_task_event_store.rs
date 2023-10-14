use crate::models::PlannableEventRow;
use crate::plannable_events_repository::PlannableEventsRepository;
use entities::task_events::TaskCreatedEvent;
use std::io::Error;
use std::io::ErrorKind;
use uuid::Uuid;

pub struct TaskEventStore {
    repository: PlannableEventsRepository,
}

impl TaskEventStore {
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
        taskcreatedsevents: Vec<TaskCreatedEvent>,
    ) -> Result<(), std::io::Error> {
        let rows: Vec<PlannableEventRow> = taskcreatedsevents
            .into_iter()
            .map(|event| event.into())
            .collect();
        self.repository
            .save(rows)
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))
    }

    pub fn read(&mut self, task_id: Uuid) -> Result<Vec<TaskCreatedEvent>, std::io::Error> {
        let rows = self
            .repository
            .read(&task_id.to_string())
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    pub fn get_all(&mut self) -> Result<Vec<TaskCreatedEvent>, std::io::Error> {
        let rows = self
            .repository
            .get_all()
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))?;

        Ok(rows.into_iter().map(|row| row.into()).collect())
    }
}

#[cfg(test)]
mod tests {

    use crate::plannable_task_event_store::TaskEventStore;
    use chrono::NaiveDate;
    use entities::task_events::TaskCreatedEvent;
    use uuid::Uuid;

    #[test]
    fn given_taskcreatedevent_when_savetorepository_then_repositoryhasoneentry() {
        let database_url = "/tmp/simple_plan_task_saveevent.db";
        let mut eventstore = TaskEventStore::clean(database_url).unwrap();

        let plannables = vec![TaskCreatedEvent {
            event_id: Uuid::new_v4(),
            task_id: Uuid::new_v4(),
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
    fn given_taskid_when_read_then_returnalleventsforthetaskid() {
        let database_url = "/tmp/simple_plan_task_readevent.db";
        let mut eventstore = TaskEventStore::clean(database_url).unwrap();
        let task_id = Uuid::new_v4();
        let task_created = TaskCreatedEvent {
            event_id: Uuid::new_v4(),
            task_id: task_id.clone(),
            sequence: 0,
            title: String::from("Buy rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        };
        let task_changed = TaskCreatedEvent {
            event_id: Uuid::new_v4(),
            task_id: task_id.clone(),
            sequence: 1,
            title: String::from("Read rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        };
        let plannables = vec![task_created, task_changed];
        eventstore.save(plannables.clone()).unwrap();
        let result = eventstore.read(task_id).unwrap();
        assert_eq!(result, plannables);
    }

    #[test]
    fn given_taskid_when_doesnotexist_then_returnnoevents() {
        let database_url = "/tmp/simple_plan_task_readnoevent.db";
        let mut eventstore = TaskEventStore::clean(database_url).unwrap();
        let task_id = Uuid::new_v4();
        let result = eventstore.read(task_id).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn given_taskidexisting_when_savemultiplewithduplicate_then_rollback() {
        //GIVEN initial event
        let database_url = "/tmp/simple_plan_task_addduplicaterollback.db";
        let mut eventstore = TaskEventStore::clean(database_url).unwrap();
        let task_id = Uuid::new_v4();
        let task_created = vec![TaskCreatedEvent {
            event_id: Uuid::new_v4(),
            task_id: task_id.clone(),
            sequence: 0,
            title: String::from("Buy rust book"),
            end_date: Some(
                NaiveDate::from_ymd_opt(2023, 9, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        }];
        eventstore.save(task_created.clone()).unwrap();
        let task_changed = vec![
            TaskCreatedEvent {
                event_id: Uuid::new_v4(),
                task_id: task_id.clone(),
                sequence: 1,
                title: String::from("Read rust book"),
                end_date: Some(
                    NaiveDate::from_ymd_opt(2023, 9, 29)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            },
            TaskCreatedEvent {
                event_id: Uuid::new_v4(),
                task_id: task_id.clone(),
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
        let result = eventstore.save(task_changed);
        //THEN error
        assert!(result.is_err());
        let read_task_created = eventstore.read(task_id).unwrap();
        //Test rollback of THEN by comparing the output to the original DB insertion to test rollback
        assert_eq!(read_task_created, task_created);
    }
}
