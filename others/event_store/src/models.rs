use chrono::NaiveDateTime;
use diesel::prelude::*;
use entities::todo_events::TodoCreatedEvent;
use serde::{Deserialize, Serialize};
use std::str;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::plannable_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlannableEventRow {
    pub event_id: Vec<u8>,
    pub plannable_id: String,
    pub sequence: i32,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoBody {
    title: String,
    end_date: Option<NaiveDateTime>,
}

impl From<TodoCreatedEvent> for PlannableEventRow {
    fn from(todo_created_events: TodoCreatedEvent) -> Self {
        let body = TodoBody {
            title: todo_created_events.title,
            end_date: todo_created_events.end_date,
        };
        PlannableEventRow {
            //event_id: (*todo_created_events.event_id.as_bytes()).to_vec(),
            event_id: todo_created_events.event_id.to_string().into(),
            plannable_id: todo_created_events.todo_id.to_string(),
            sequence: todo_created_events.sequence,
            body: serde_json::to_string(&body).unwrap().into(),
        }
    }
}

impl From<PlannableEventRow> for TodoCreatedEvent {
    fn from(row: PlannableEventRow) -> Self {
        let deserialized: TodoBody =
            serde_json::from_str(str::from_utf8(&row.body).unwrap()).unwrap();
        TodoCreatedEvent {
            event_id: Uuid::parse_str(str::from_utf8(&row.event_id).unwrap()).unwrap(),
            todo_id: Uuid::parse_str(&row.plannable_id).unwrap(),
            sequence: row.sequence,
            title: deserialized.title,
            end_date: deserialized.end_date,
        }
    }
}
