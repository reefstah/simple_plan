use crate::models::PlannableEventRow;
use crate::schema::plannable_events::dsl::*;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::SqliteConnection;
use std::io::Error;
use std::io::ErrorKind;
struct PlannableEventsRepository {
    sql_connection: SqliteConnection,
}

impl PlannableEventsRepository {
    fn initialize(database_url: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            sql_connection: establish_connection(database_url)?,
        })
    }
    fn save(&mut self, eventrow: Vec<PlannableEventRow>) -> Result<(), diesel::result::Error> {
        insert_into(plannable_events)
            .values(&eventrow)
            .execute(&mut self.sql_connection)
            .map(|size| ())
    }
    fn reset(&mut self) -> Result<(), diesel::result::Error> {
        delete(plannable_events)
            .execute(&mut self.sql_connection)
            .map(|size| ())
    }
    fn drop_table(&mut self) -> Result<(), diesel::result::Error> {
        diesel::sql_query("DROP TABLE IF EXISTS plannable_events")
            .execute(&mut self.sql_connection)
            .map(|size| ())
    }

    fn create_table(&mut self) -> Result<(), diesel::result::Error> {
        diesel::sql_query(
            "CREATE TABLE plannable_events (
             event_id BLOB PRIMARY KEY NOT NULL,
             plannable_id TEXT NOT NULL,
             sequence INTEGER NOT NULL,
             body BLOB NOT NULL,
             UNIQUE(plannable_id, sequence) 
             ON CONFLICT ROLLBACK)",
        )
        .execute(&mut self.sql_connection)
        .map(|size| ())
    }
    fn get_plannable_id(
        mut self,
        id: &String,
    ) -> Result<Vec<PlannableEventRow>, diesel::result::Error> {
        plannable_events
            .filter(plannable_id.eq(id))
            .select(PlannableEventRow::as_select())
            .load(&mut self.sql_connection)
    }
}
fn establish_connection(database_url: &str) -> Result<SqliteConnection, Error> {
    dotenvy::dotenv().map_err(|_error| Error::new(ErrorKind::Other, "error"))?;
    Ok(SqliteConnection::establish(&database_url)
        .map_err(|_error| Error::new(ErrorKind::Other, "error"))?)
}
#[cfg(test)]
mod tests {

    use crate::models::PlannableEventRow;

    use super::PlannableEventsRepository;

    #[test]
    fn initialize() {
        let database_url = "/tmp/simple_plan_initialize.db";
        let repository = PlannableEventsRepository::initialize(database_url);
        assert!(repository.is_ok());
    }

    #[test]
    fn save() {
        let database_url = "/tmp/simple_plan_save.db";
        let mut repository = PlannableEventsRepository::initialize(database_url).unwrap();
        repository.drop_table().unwrap();
        repository.create_table().unwrap();

        let plannables = vec![PlannableEventRow {
            event_id: String::from("7fe8b15d-1a3e-461d-9057-99ef10459a0e")
                .as_bytes()
                .to_vec(),
            plannable_id: String::from("1"),
            sequence: 0,
            body: String::from("").as_bytes().to_vec(),
        }];
        let result = repository.save(plannables);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn read() {
        let database_url = "/tmp/simple_plan_read.db";
        let mut repository = PlannableEventsRepository::initialize(database_url).unwrap();
        repository.drop_table().unwrap();
        repository.create_table().unwrap();

        let plannables = vec![PlannableEventRow {
            event_id: String::from("7fe8b15d-1a3e-461d-9057-99ef10459a0e")
                .as_bytes()
                .to_vec(),
            plannable_id: String::from("1"),
            sequence: 0,
            body: String::from("").as_bytes().to_vec(),
        }];
        repository.save(plannables.clone()).unwrap();
        let result = repository
            .get_plannable_id(&plannables[0].plannable_id)
            .unwrap();
        assert_eq!(result, plannables);
    }
}
