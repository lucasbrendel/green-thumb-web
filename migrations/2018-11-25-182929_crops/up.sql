-- Your SQL goes here
CREATE TABLE  crops (
    id SERIAL PRIMARY KEY,
    num_plants SERIAL NOT NULL,
    date_planted TIMESTAMP NOT NULL,
    plant_id SERIAL NOT NULL,
    FOREIGN KEY(plant_id) REFERENCES plants(id));