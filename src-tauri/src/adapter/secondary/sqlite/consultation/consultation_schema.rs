use diesel::table;

table! {
    consultations (id) {
        appointment_date_time -> Timestamp,
        consultation_type -> Text,
        created_at -> Timestamp,
        id -> Text,
        location -> Text,
        note -> Nullable<Text>,
        patient_id -> Text,
        price -> Double,
        status -> Text,
        updated_at -> Timestamp
    }
}
