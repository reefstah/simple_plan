use chrono::NaiveDateTime;
use uuid::Uuid;

type TaskId = Uuid;
type TaskCreatedEventId = Uuid;
#[derive(Debug, PartialEq, Clone)]
pub struct TaskCreatedEvent {
    pub title: String,
    pub task_id: TaskId,
    pub event_id: TaskCreatedEventId,
    pub sequence: i32,
    pub end_date: Option<NaiveDateTime>,
}
