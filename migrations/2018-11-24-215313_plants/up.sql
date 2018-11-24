-- Your SQL goes here
CREATE TABLE IF NOT EXISTS plants (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            days_to_maturity SERIAL,
            notes TEXT,
            zones INTEGER [],
            plant_type TEXT NOT NULL
            );