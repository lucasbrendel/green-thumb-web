//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.
use crate::schema::plants;
use crate::models::{NewPlant, Plant, PlantType};
use crate::data::DataMgr;
use diesel::RunQueryDsl;
use diesel;

/// Create a plant object and store in the database
pub fn create_plant<'a>(
    mgr: &DataMgr,
    title: &'a str,
    days_to_maturity: i32,
    plant_type: PlantType
    ) -> Plant {
    let new_plant = NewPlant { title: title, days_to_maturity: days_to_maturity };
    diesel::insert_into(plants::table)
        .values(&new_plant)
        .get_result(&mgr.conn)
        .expect("Error creating new plant")
}