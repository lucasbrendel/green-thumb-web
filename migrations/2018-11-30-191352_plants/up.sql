-- Your SQL goes here
CREATE TYPE plant_type AS ENUM ('annual', 'perennial');
CREATE TABLE plants (
            id SERIAL PRIMARY KEY,
            title VARCHAR NOT NULL,
            days_to_maturity SERIAL NOT NULL);