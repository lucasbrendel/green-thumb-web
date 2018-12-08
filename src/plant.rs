//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.
use crate::data::DataMgr;
use crate::models::{NewPlant, Plant, PlantType};
use crate::schema::plants;
use diesel;
use diesel::RunQueryDsl;

/// Create a plant object and store in the database
pub fn create_plant<'a>(mgr: &DataMgr, title: &'a str, days_to_maturity: i32) -> Plant {
    let new_plant = NewPlant {
        title: title,
        days_to_maturity: days_to_maturity,
    };
    diesel::insert_into(plants::table)
        .values(&new_plant)
        .get_result(&mgr.conn)
        .expect("Error creating new plant")
}

#[cfg(test)]
mod test{
    use super::*;
    use std::env;
    use crate::data;
    use diesel::prelude::*;
    use dotenv::dotenv;
    
    #[test]
    fn test_create_plant() {
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mgr = data::DataMgr::new(url);
        mgr.conn.begin_test_transaction().unwrap();
        create_plant(&mgr, "Tomato", 45);
    }
}
