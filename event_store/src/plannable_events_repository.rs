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
    fn initialize() -> Result<Self, std::io::Error> {
        Ok(Self {
            sql_connection: establish_connection()?,
        })
    }
    fn save(mut self, eventrow: Vec<PlannableEventRow>) -> Result<(), diesel::result::Error> {
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
}
fn establish_connection() -> Result<SqliteConnection, Error> {
    dotenvy::dotenv().map_err(|_error| Error::new(ErrorKind::Other, "error"))?;
    let url =
        ::std::env::var("DATABASE_URL").map_err(|_error| Error::new(ErrorKind::Other, "error"))?;

    Ok(
        SqliteConnection::establish(&url)
            .map_err(|_error| Error::new(ErrorKind::Other, "error"))?,
    )
}
#[cfg(test)]
mod tests {
    use diesel::SqliteConnection;

    use crate::models::PlannableEventRow;

    use super::PlannableEventsRepository;

    #[test]
    fn initialize() {
        let repository = PlannableEventsRepository::initialize();
        assert!(repository.is_ok());
    }

    #[test]
    fn save() {
        let mut repository = PlannableEventsRepository::initialize().unwrap();
        repository.reset().unwrap();
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
    fn save1() {
        let mut repository = PlannableEventsRepository::initialize().unwrap();
        repository.reset().unwrap();
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
    fn save2() {
        let mut repository = PlannableEventsRepository::initialize().unwrap();
        repository.reset().unwrap();
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
    fn save3() {
        let mut repository = PlannableEventsRepository::initialize().unwrap();
        repository.reset().unwrap();
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
}
