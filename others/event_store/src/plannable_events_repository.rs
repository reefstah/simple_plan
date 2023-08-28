use crate::models::PlannableEventRow;
use crate::schema::plannable_events::dsl::*;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::SqliteConnection;
use std::io::Error;
use std::io::ErrorKind;

pub struct PlannableEventsRepository {
    sql_connection: SqliteConnection,
}

impl PlannableEventsRepository {
    pub fn initialize(database_url: &str) -> Result<Self, std::io::Error> {
        Ok(Self {
            sql_connection: establish_connection(database_url)?,
        })
    }
    pub fn save(&mut self, eventrow: Vec<PlannableEventRow>) -> Result<(), diesel::result::Error> {
        insert_into(plannable_events)
            .values(&eventrow)
            .execute(&mut self.sql_connection)
            .map(|_size| ())
    }
    fn reset(&mut self) -> Result<(), diesel::result::Error> {
        delete(plannable_events)
            .execute(&mut self.sql_connection)
            .map(|_size| ())
    }
    pub fn drop_table(&mut self) -> Result<(), diesel::result::Error> {
        diesel::sql_query("DROP TABLE IF EXISTS plannable_events")
            .execute(&mut self.sql_connection)
            .map(|_size| ())
    }
    pub fn create_table(&mut self) -> Result<(), diesel::result::Error> {
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
        .map(|_size| ())
    }
    pub fn read(&mut self, id: &String) -> Result<Vec<PlannableEventRow>, diesel::result::Error> {
        plannable_events
            .filter(plannable_id.eq(id))
            .select(PlannableEventRow::as_select())
            .load(&mut self.sql_connection)
    }
    pub fn get_all(&mut self) -> Result<Vec<PlannableEventRow>, diesel::result::Error> {
        plannable_events
            .select(PlannableEventRow::as_select())
            .load(&mut self.sql_connection)
    }
}

fn establish_connection(database_url: &str) -> Result<SqliteConnection, Error> {
    Ok(SqliteConnection::establish(&database_url)
        .map_err(|_error| Error::new(ErrorKind::Other, "SqliteConnection"))?)
}
