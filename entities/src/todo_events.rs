use chrono::NaiveDateTime;
use uuid::Uuid;

type TodoId = Uuid;
type TodoCreatedEventId = Uuid;
#[derive(Debug, PartialEq, Clone)]
pub struct TodoCreatedEvent {
    pub title: String,
    pub todo_id: TodoId,
    pub event_id: TodoCreatedEventId,
    pub sequence: i32,
    pub end_date: Option<NaiveDateTime>,
}
