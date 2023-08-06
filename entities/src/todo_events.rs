use uuid::Uuid;

type TodoId = Uuid;
type TodoCreatedEventId = Uuid;
pub struct TodoCreatedEvent {
    pub title: String,
    pub todo_id: TodoId,
    pub event_id: TodoCreatedEventId,
    pub sequence: i32,
}
