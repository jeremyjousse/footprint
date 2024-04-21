use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    entity::patient::Patient,
    helper::data_time::option_date_serializer,
    value_object::{
        contact_information::ContactInformation, personal_name::PersonalName,
        postal_address::PostalAddress,
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientDto {
    #[serde(with = "option_date_serializer")]
    pub birthdate: Option<NaiveDate>,
    pub contact_information: ContactInformation,

    pub diabetic: bool,
    pub id: Option<String>,

    pub long_duration_disease: bool,
    pub national_insurance_number: Option<String>,
    pub notes: String, // TODO Option<String>
    pub personal_name: PersonalName,

    pub postal_address: PostalAddress,
    pub profession: Option<String>,
}

impl From<PatientDto> for Patient {
    fn from(dto: PatientDto) -> Self {
        // TODO remove
        let fake_date = None;

        Self {
            birthdate: dto.birthdate,
            contact_information: dto.contact_information,
            created_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
            diabetic: dto.diabetic,
            id: match dto.id {
                Some(uuid) => Uuid::parse_str(&uuid).unwrap_or(Uuid::new_v4()), // TODO do we really want to create a new uuid?
                None => Uuid::new_v4(),
            },
            long_duration_disease: dto.long_duration_disease,
            national_insurance_number: dto.national_insurance_number,
            notes: dto.notes,
            personal_name: dto.personal_name,
            postal_address: dto.postal_address,
            profession: dto.profession,
            updated_at: match fake_date {
                Some(updated_at) => updated_at,
                None => Utc::now(),
            },
        }
    }
}

impl From<Patient> for PatientDto {
    fn from(value: Patient) -> Self {
        Self {
            birthdate: value.birthdate,
            contact_information: value.contact_information,
            diabetic: value.diabetic,
            id: Some(value.id.to_string()),
            long_duration_disease: value.long_duration_disease,
            national_insurance_number: value.national_insurance_number,
            notes: value.notes,
            personal_name: value.personal_name,
            postal_address: value.postal_address,
            profession: value.profession,
        }
    }
}
