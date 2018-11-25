-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    is_completed BOOL NOT NULL,
    completed_date TEXT);