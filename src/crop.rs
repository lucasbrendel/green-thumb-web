use crate::data::plant::Plant;
use chrono::{Duration, NaiveDate};
use rusqlite::types::*;
use rusqlite::Connection;

pub fn create_crop<'a>(
    mgr: &DataMgr,
    title: &'a str,
    days_to_maturity: i64,
    plant_type: PlantType,
) -> Plant {
    let new_plant = NewPlant { title: title };
    diesel::insert_into(plants::table)
        .values(&new_plant)
        .get_result(mgr.conn)
        .expect("Error creating new plant")
}

impl Crop {

    /// Provides the ideal harvest date based on planting date and time to maturity
    fn planned_harvest_date(&self, conn: &Connection) -> NaiveDate {
        let plant = Plant::get_plant_by_id(&conn, self.plant_id).unwrap();
        self.date_planted + Duration::days(plant.days_to_maturity)
    }
}
