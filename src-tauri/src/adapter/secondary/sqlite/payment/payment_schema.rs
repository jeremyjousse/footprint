use diesel::table;

table! {
    payments(id) {
        amount -> Double,
        consultation_id -> Text,
        created_at -> Timestamp,
        id -> Text,
        payed_at -> Timestamp,
        payment_method -> Text,
        status -> Text,
        updated_at -> Timestamp,
    }
}
