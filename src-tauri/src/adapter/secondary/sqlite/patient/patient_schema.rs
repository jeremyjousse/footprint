use diesel::table;

table! {
    patients (id) {
        birthdate -> Nullable<Timestamp>, // Timestamp??
        city -> Nullable<Text>,
        created_at -> Timestamp,
        country -> Nullable<Text>,
        diabetic -> Bool,
        email -> Nullable<Text>,
        first_name -> Text,
        id -> Text,
        last_name -> Text,
        long_duration_disease -> Bool,
        mobile_phone -> Nullable<Text>,
        national_insurance_number -> Nullable<Text>,
        notes -> Text,
        phone -> Nullable<Text>,
        postal_code -> Nullable<Text>,
        profession -> Nullable<Text>,
        street -> Nullable<Text>,
        title -> Nullable<Text>,
        updated_at -> Timestamp,
    }
}
