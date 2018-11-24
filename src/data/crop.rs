use chrono::{Duration, NaiveDate};
use crate::data::plant::Plant;
use rusqlite::types::*;
use rusqlite::Connection;

/// Tracking the growth and harvest of a specific plant
pub struct Crop {
    /// Number of plants sown
    pub num_plants: u32,
    /// The date that the plants were planted
    pub date_planted: NaiveDate,
    /// The id of the plant sown
    pub plant_id: i64,
    /// Unique id for the crop
    pub id: i64,
}

impl Crop {
    /// Create a new crop instance
    fn new(conn: &Connection, plant_id: i64, num_plants: u32, date_planted: NaiveDate) -> Self {
        conn.execute(
            "INSERT INTO crops (num_plants, date_planted, plant_id) VALUES (?1, ?2, ?3)",
            &[&num_plants, &date_planted as &ToSql, &plant_id],
        )
        .unwrap();

        Crop {
            id: conn.last_insert_rowid(),
            plant_id,
            num_plants,
            date_planted,
        }
    }

    /// Provides the ideal harvest date based on planting date and time to maturity
    fn planned_harvest_date(&self, conn: &Connection) -> NaiveDate {
        let plant = Plant::get_plant_by_id(&conn, self.plant_id).unwrap();
        self.date_planted + Duration::days(plant.days_to_maturity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::data::plant::PlantType;
    use crate::data::DataMgr;

    #[test]
    fn new_crop() {
        let mgr = DataMgr::new(String::from("./db/green-thumb-test-new_crop.db"));
        let plant = Plant::new(&mgr.conn, String::from("Bean"), 75, PlantType::Annual);
        let crop = Crop::new(&mgr.conn, plant.id, 5, NaiveDate::from_ymd(2018, 5, 6));
        assert_eq!(5, crop.num_plants);
        assert_eq!(NaiveDate::from_ymd(2018, 5, 6), crop.date_planted);
        assert_eq!(plant.id, crop.plant_id);
    }

    #[test]
    fn harvest_date() {
        let mgr = DataMgr::new(String::from("./db/green-thumb-test-harvest_date.db"));
        let plant = Plant::new(&mgr.conn, String::from("Bean"), 75, PlantType::Annual);
        let crop = Crop::new(&mgr.conn, plant.id, 5, NaiveDate::from_ymd(2018, 5, 6));
        assert_eq!(
            NaiveDate::from_ymd(2018, 7, 20),
            crop.planned_harvest_date(&mgr.conn)
        );
    }
}
