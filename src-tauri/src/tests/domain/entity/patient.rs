use celes::Country;
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::{
    entity::patient::Patient,
    value_object::{
        contact_information::ContactInformation, personal_name::PersonalName,
        postal_address::PostalAddress,
    },
};

#[cfg(test)]

pub fn valid_patient_test_helper() -> Patient {
    Patient {
        birthdate: Some(NaiveDate::parse_from_str("1978-01-09", "%Y-%m-%d").unwrap()),
        created_at: Utc::now(),
        contact_information: ContactInformation {
            email: Some("jeremy.jousse@gmail".into()),
            mobile_phone: Some("0650805080".into()),
            phone: Some("032005500550".into()),
        },
        diabetic: false,
        id: Uuid::new_v4(),
        long_duration_disease: false,
        national_insurance_number: Some("017859000123".into()),
        notes: "My best patient".into(),
        personal_name: PersonalName {
            first_name: "Jérémy".into(),
            last_name: "Jousse".into(),
            title: None,
        },
        profession: Some("Developer".into()),
        postal_address: PostalAddress {
            city: Some("Lille".into()),
            country: Some(Country::france()),
            postal_code: Some("59000".into()),
            street: Some("1 rue de Paris".into()),
        },
        updated_at: Utc::now(),
    }
}
