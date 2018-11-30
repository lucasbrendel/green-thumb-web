use crate::schema::plants;
use diesel::prelude::Insertable;
use diesel_derive_enum::*;

/// Standard type to define all things to grow
#[derive(Debug, Queryable)]
pub struct Plant {
    pub id: i32,
    /// Name of a plant.
    pub title: String,
    /// Seasonal type of plant
    // pub plant_type: PlantType,
    /// Growing zones defined by USDA that a plant can survive in. This is limited to first 10 zones.
    // pub zones: Vec<u8>,
    /// Any description or textual things to track about the plant.
    // pub notes: String,
    /// Number of days from planting until germination occurs
    pub days_to_maturity: i32,
}

#[derive(Insertable)]
#[table_name = "plants"]
pub struct NewPlant<'a> {
    pub title: &'a str,
    // pub plant_type: &'a PlantType,
    // pub zones: Vec<u8>,
    // pub notes: &'a str,
    pub days_to_maturity: i32,
}

/// Seasonal variety types of plants
#[derive(DbEnum, Debug, PartialEq, Eq)]
pub enum PlantType {
    /// Plant has one growing season and needs to be replanted every year
    Annual,
    /// Plant survives over multiple growing seasons.
    Perennial,
}

/// Tracking the growth and harvest of a specific plant
#[derive(Debug, Queryable)]
pub struct Crop {
    /// Number of plants sown
    pub num_plants: u32,
    /// The date that the plants were planted
    // pub date_planted: NaiveDate,
    /// The id of the plant sown
    pub plant_id: i64,
    /// Unique id for the crop
    pub id: i64,
}
