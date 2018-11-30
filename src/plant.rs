//! The design behind the plant module is to act as the main container of data for any
//! type of plant that could be tracked and maintained in the garden or orchard.
//! This item should contain the necessary data so that work can be done for the use to alert
//! when work is needed to be performed.



pub fn create_plant<'a>(
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
