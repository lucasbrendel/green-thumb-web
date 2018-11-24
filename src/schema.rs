table! {
    plants (id) {
        id -> Int4,
        name -> Text,
        days_to_maturity -> Nullable<Int4>,
        notes -> Nullable<Text>,
        zones -> Nullable<Array<Int4>>,
        plant_type -> Text,
    }
}
