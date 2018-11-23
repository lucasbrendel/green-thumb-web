//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.

use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{Connection, Result, NO_PARAMS};
use std::str::FromStr;

/// Standard type to define all things to grow
#[derive(Debug)]
pub struct Plant {
    pub id: i64,
    /// Name of a plant.
    pub name: String,
    /// Seasonal type of plant
    pub plant_type: PlantType,
    /// Growing zones defined by USDA that a plant can survive in. This is limited to first 10 zones.
    pub zones: Vec<u8>,
    /// Any description or textual things to track about the plant.
    pub notes: String,
    /// Number of days from planting until germination occurs
    pub days_to_maturity: i64,
}

/// Seasonal variety types of plants
#[derive(Debug, PartialEq, Serialize, Deserialize, Display, EnumString, AsStaticStr)]
pub enum PlantType {
    /// Plant has one growing season and needs to be replanted every year
    Annual,
    /// Plant survives over multiple growing seasons.
    Perennial,
}

impl ToSql for PlantType {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        let ty = self.to_string();
        Ok(ToSqlOutput::from(ty))
    }
}

impl FromSql for PlantType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match PlantType::from_str(value.as_str().unwrap()).unwrap() {
            PlantType::Annual => Ok(PlantType::Annual),
            PlantType::Perennial => Ok(PlantType::Perennial),
            // _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl Plant {
    /// Create a new plant type
    pub fn new(
        conn: &Connection,
        name: String,
        days_to_maturity: i64,
        plant_type: PlantType,
    ) -> Self {
        conn.execute(
            "INSERT INTO plants (name, days_to_maturity, plant_type, zones, notes)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &name as &ToSql,
                &days_to_maturity,
                &plant_type,
                &Vec::new(),
                &String::from(""),
            ],
        )
        .unwrap();

        Plant {
            id: conn.last_insert_rowid(),
            name,
            plant_type,
            notes: String::from(""),
            zones: Vec::new(),
            days_to_maturity,
        }
    }

    /// Access all defined plants
    pub fn get_plants(conn: &Connection) -> Result<Vec<Plant>> {
        let mut plants: Vec<Plant> = Vec::new();
        let mut stmt = conn
            .prepare("SELECT id, name, days_to_maturity, zones, notes, plant_type FROM plants")?;
        info!("Get Plants: {:?}", stmt);
        let map_plants = stmt.query_map(NO_PARAMS, |row| Plant {
            id: row.get(0),
            name: row.get(1),
            days_to_maturity: row.get(2),
            zones: row.get(3),
            notes: row.get(4),
            plant_type: row.get(5),
        })?;
        for plant in map_plants {
            info!("Accessing {:?}", plant);
            plants.push(plant.unwrap());
        }
        Ok(plants)
    }

    /// Obtain a plant based on the database id provided
    pub fn get_plant_by_id(conn: &Connection, uid: i64) -> Result<Plant> {
        let mut stmt = conn.prepare(
            "SELECT name, days_to_maturity, zones, notes, plant_type FROM plants WHERE id = :uid",
        )?;
        info!("Get Plant by ID: {:?}", stmt);
        let plant = stmt.query_map(&[&uid], |row| Plant {
            id: uid,
            name: row.get(0),
            days_to_maturity: row.get(1),
            zones: row.get(2),
            notes: row.get(3),
            plant_type: row.get(4),
        })?;
        Ok(plant.last().unwrap().unwrap())
    }
}

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
