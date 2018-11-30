-- Your SQL goes here
CREATE TYPE plant_type AS ENUM ('annual', 'perennial');
CREATE TABLE plants (
            id INTEGER PRIMARY KEY,
            title VARCHAR NOT NULL,
            days_to_maturity INTEGER NOT NULL);