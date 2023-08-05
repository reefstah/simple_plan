use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::plannable_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlannableEventRow {
    pub event_id: Vec<u8>,
    pub plannable_id: String,
    pub sequence: i32,
    pub body: Vec<u8>,
}
