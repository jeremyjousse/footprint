use celes::Country;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::domain::{
    entity::patient::Patient,
    helper::data_time::{naive_date_time_to_naive_date, naive_date_to_naive_date_time},
    value_object::{
        contact_information::ContactInformation, personal_name::PersonalName,
        postal_address::PostalAddress,
    },
};

use super::patient_schema::patients;

#[derive(
    Clone, Debug, Deserialize, Identifiable, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
#[diesel(table_name = patients)]
#[diesel(primary_key(id))]
pub struct DbPatient {
    pub birthdate: Option<NaiveDateTime>,
    pub city: Option<String>,
    pub created_at: NaiveDateTime,
    pub country: Option<String>,
    pub diabetic: bool,
    pub email: Option<String>,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub long_duration_disease: bool,
    pub national_insurance_number: Option<String>,
    pub mobile_phone: Option<String>,
    pub notes: String,
    pub phone: Option<String>,
    pub postal_code: Option<String>,
    pub profession: Option<String>,
    pub street: Option<String>,
    pub title: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl From<Patient> for DbPatient {
    fn from(value: Patient) -> Self {
        Self {
            birthdate: value.birthdate.map(naive_date_to_naive_date_time),
            city: value.postal_address.city,
            created_at: value.created_at.naive_utc(),
            country: value
                .postal_address
                .country
                .map(|country| country.long_name.to_string()),
            diabetic: value.diabetic,
            email: value.contact_information.email,
            first_name: value.personal_name.first_name,
            id: value.id.to_string(),
            last_name: value.personal_name.last_name,
            long_duration_disease: value.long_duration_disease,
            mobile_phone: value.contact_information.mobile_phone,
            national_insurance_number: value.national_insurance_number,
            notes: value.notes,
            phone: value.contact_information.phone,
            profession: value.profession,
            postal_code: value.postal_address.postal_code,
            street: value.postal_address.street,
            title: value.personal_name.title,
            updated_at: value.updated_at.naive_utc(),
        }
    }
}

impl From<DbPatient> for Patient {
    fn from(value: DbPatient) -> Self {
        let contact_information = ContactInformation {
            email: value.email,
            mobile_phone: value.mobile_phone,
            phone: value.phone,
        };

        let personal_name = PersonalName {
            first_name: value.first_name,
            last_name: value.last_name,
            title: value.title,
        };

        let postal_address = PostalAddress {
            city: value.city,
            country: match value.country {
                Some(country) => Country::from_name(country).ok(),
                None => None,
            },

            postal_code: value.postal_code,
            street: value.street,
        };

        Self {
            birthdate: value.birthdate.map(naive_date_time_to_naive_date),
            created_at: value.created_at.and_utc(),
            contact_information,
            diabetic: value.diabetic,
            id: uuid::Uuid::parse_str(&value.id).unwrap(),
            long_duration_disease: value.long_duration_disease,
            national_insurance_number: value.national_insurance_number,
            notes: value.notes,
            personal_name,
            postal_address,
            updated_at: value.updated_at.and_utc(),
            profession: value.profession,
        }
    }
}
