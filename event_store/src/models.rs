use crate::schema::plannable_events::plannable_id;
use diesel::prelude::*;
use entities::TodoCreatedEvent;
#[derive(Debug, PartialEq, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::plannable_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlannableEventRow {
    pub event_id: Vec<u8>,
    pub plannable_id: String,
    pub sequence: i32,
    pub body: Vec<u8>,
}

impl From<TodoCreatedEvent> for PlannableEventRow {
    fn from(todo_created_events: TodoCreatedEvent) -> Self {
        PlannableEventRow {
            event_id: (*todo_created_events.event_id.as_bytes()).to_vec(),
            plannable_id: todo_created_events.todo_id.to_string(),
            sequence: todo_created_events.sequence,
            body: todo_created_events.title.into(),
        }
    }
}
