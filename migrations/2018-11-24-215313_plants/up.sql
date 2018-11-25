-- Your SQL goes here
CREATE TYPE plant_type AS ENUM ('annual', 'perennial');
CREATE TABLE IF NOT EXISTS plants (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            days_to_maturity SERIAL,
            notes TEXT,
            zones SERIAL [],
            plant_type plant_type NOT NULL
            );