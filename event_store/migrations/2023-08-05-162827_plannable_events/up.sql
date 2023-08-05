CREATE TABLE plannable_events (
    event_id BLOB PRIMARY KEY NOT NULL,
    plannable_id TEXT NOT NULL,
    sequence INTEGER NOT NULL,
    body BLOB NOT NULL,
    UNIQUE(plannable_id, sequence) ON CONFLICT ROLLBACK
);
