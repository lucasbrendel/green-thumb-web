use diesel::prelude::*;

pub struct DataMgr {
    pub conn: PgConnection,
}

impl DataMgr {
    pub fn new(db: String) -> Self {
        DataMgr {
            conn: PgConnection::establish(&db).expect(&format!("Error connecting to {}", db)),
        }
    }
}
