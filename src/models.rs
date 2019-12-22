use crate::schema::plants;

/// Standard type to define all things to grow
#[derive(Debug, Queryable)]
pub struct Plant {
    pub id: i32,
    /// Name of a plant.
    pub title: String,
    /// Number of days from planting until germination occurs
    pub days_to_maturity: i32,
}

#[derive(Insertable)]
#[table_name = "plants"]
pub struct NewPlant<'a> {
    pub title: &'a str,
    pub days_to_maturity: i32,
}
