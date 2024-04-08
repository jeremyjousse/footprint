use diesel::table;

table! {
    consultation_types (id) {
        created_at -> Timestamp,
        id -> Text,
        name -> Text,
        price -> Double,
        updated_at -> Timestamp,
    }
}
