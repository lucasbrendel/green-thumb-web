use crate::data::plant::Plant;
use chrono::{Duration, NaiveDate};
use rusqlite::types::*;
use rusqlite::Connection;

impl Crop {

    /// Provides the ideal harvest date based on planting date and time to maturity
    fn planned_harvest_date(&self, conn: &Connection) -> NaiveDate {
        let plant = Plant::get_plant_by_id(&conn, self.plant_id).unwrap();
        self.date_planted + Duration::days(plant.days_to_maturity)
    }
}

