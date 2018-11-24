pub mod crop;
pub mod plant;
pub mod task;

use rusqlite::{Connection, NO_PARAMS};

pub struct DataMgr {
    pub conn: Connection,
}

impl DataMgr {
    pub fn new(db: String) -> Self {
        let data = DataMgr {
            conn: Connection::open(db).unwrap(),
        };
        data.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS plants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            days_to_maturity INTEGER,
            notes TEXT,
            zones BLOB,
            plant_type TEXT NOT NULL
            );",
                NO_PARAMS,
            )
            .unwrap();
        data.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS crops (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                num_plants INTEGER NOT NULL,
                date_planted TEXT NOT NULL,
                plant_id INTEGER NOT NULL,
                FOREIGN KEY(plant_id) REFERENCES plants(id));",
                NO_PARAMS,
            )
            .unwrap();
        data.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                is_completed BOOL NOT NULL,
                completed_date TEXT);",
                NO_PARAMS,
            )
            .unwrap();
        data
    }
}
