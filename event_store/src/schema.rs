// @generated automatically by Diesel CLI.

diesel::table! {
    plannable_events (event_id) {
        event_id -> Binary,
        plannable_id -> Text,
        sequence -> Integer,
        body -> Binary,
    }
}
