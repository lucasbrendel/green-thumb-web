pub mod crop;
pub mod plant;
pub mod task;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

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
                "",
                NO_PARAMS,
            )
            .unwrap();
        data.conn
            .execute(
                "",
                NO_PARAMS,
            )
            .unwrap();
        data
    }
}
