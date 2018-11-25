//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.

use crate::data::DataMgr;
use diesel::prelude::*;
use diesel::serialize::*;
use crate::schema::plants::dsl::*;
use crate::schema::plants;
use diesel_derive_enum::*;

/// Standard type to define all things to grow
#[derive(Debug, Queryable)]
pub struct Plant {
    pub id: i64,
    /// Name of a plant.
    pub name: String,
    /// Seasonal type of plant
    // pub plant_type: PlantType,
    /// Growing zones defined by USDA that a plant can survive in. This is limited to first 10 zones.
    pub zones: Vec<u8>,
    /// Any description or textual things to track about the plant.
    pub notes: String,
    /// Number of days from planting until germination occurs
    pub days_to_maturity: i64,
}

#[derive(Insertable)]
#[table_name="plants"]
pub struct NewPlant<'a> {
    pub name: &'a str,
    // pub plant_type: &'a PlantType,
}

/// Seasonal variety types of plants
#[derive(DbEnum, Debug)]
pub enum PlantType {
    /// Plant has one growing season and needs to be replanted every year
    Annual,
    /// Plant survives over multiple growing seasons.
    Perennial,
}

// impl Plant {
//     /// Create a new plant type
//     pub fn new(
//         mgr: &DataMgr,
//         name: String,
//         days_to_maturity: i64,
//         plant_type: PlantType,
//     ) -> Self {

//         Plant {
//             id: 0,
//             name,
//             plant_type,
//             notes: String::from(""),
//             zones: Vec::new(),
//             days_to_maturity,
//         }
//     }

//     /// Access all defined plants
//     pub fn get_plants(conn: &DataMgr) -> Result<Vec<Plant>> {
        
//     }

//     /// Obtain a plant based on the database id provided
//     pub fn get_plant_by_id(conn: &DataMgr, uid: i64) -> Result<Plant> {
        
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::DataMgr;

    #[test]
    fn new_plant() {
        let mgr = DataMgr::new(String::from("./db/green-thumb-test-new_plant.db"));
        let plant = Plant::new(&mgr.conn, String::from("Tomato"), 45, PlantType::Annual);
        assert_eq!("Tomato", plant.name);
        assert_eq!(45, plant.days_to_maturity);
        assert_eq!(PlantType::Annual, plant.plant_type);
    }

    #[test]
    fn get_plant_by_id() {
        let mgr = DataMgr::new(String::from("./db/green-thumb-test-get_plant_by_id.db"));
        let _t = Plant::new(&mgr.conn, String::from("Tomato"), 45, PlantType::Annual);
        let plant = Plant::get_plant_by_id(&mgr.conn, 1);
        assert_eq!(45, plant.unwrap().days_to_maturity);
    }

    #[test]
    fn get_plants() {
        let mgr = DataMgr::new(String::from("./db/green-thumb-test-get_plants.db"));
        let tomato = Plant::new(&mgr.conn, String::from("Tomato"), 45, PlantType::Annual);
        let beans = Plant::new(&mgr.conn, String::from("Beans"), 52, PlantType::Perennial);
        let cucumber = Plant::new(&mgr.conn, String::from("Cucumber"), 31, PlantType::Annual);
        let plants = Plant::get_plants(&mgr.conn).unwrap();
        assert_eq!(tomato.name, plants[0].name);
        assert_eq!(beans.name, plants[1].name);
        assert_eq!(cucumber.name, plants[2].name);
    }
}
