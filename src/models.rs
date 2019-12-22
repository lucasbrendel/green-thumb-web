use crate::schema::plants;

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
