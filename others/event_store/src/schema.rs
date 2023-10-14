// @generated automatically by Diesel CLI.
// the schema is the same for all entities, tasks, todos etc. The difference is what goes in the
// body

diesel::table! {
    plannable_events (event_id) {
        event_id -> Binary,
        plannable_id -> Text,
        sequence -> Integer,
        body -> Binary,
    }
}
