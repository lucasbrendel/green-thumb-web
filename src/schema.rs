table! {
    crops (id) {
        id -> Int4,
        num_plants -> Int4,
        date_planted -> Timestamp,
        plant_id -> Int4,
    }
}

table! {
    plants (id) {
        id -> Int4,
        title -> Text,
        days_to_maturity -> Nullable<Int4>,
        notes -> Nullable<Text>,
        zones -> Nullable<Array<Int4>>,
        plant_type -> Text,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        text -> Text,
        is_completed -> Bool,
        completed_date -> Nullable<Text>,
    }
}

joinable!(crops -> plants (plant_id));

allow_tables_to_appear_in_same_query!(
    crops,
    plants,
    tasks,
);
