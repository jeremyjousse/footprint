use chrono::{serde::ts_seconds_option, DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_object::{
    contact_information::ContactInformation, personal_name::PersonalName,
    postal_address::PostalAddress,
};

#[derive(Clone, Debug, Validate)]
pub struct Patient {
    pub birthdate: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,

    #[validate]
    pub contact_information: ContactInformation,

    pub id: Uuid,

    pub note: String,

    #[validate]
    pub personal_name: PersonalName,

    #[validate]
    pub postal_address: Option<PostalAddress>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientDto {
    #[serde(with = "ts_seconds_option")]
    pub birthdate: Option<DateTime<Utc>>,
    pub contact_information: ContactInformation,

    pub id: Option<String>,

    pub note: String,
    pub personal_name: PersonalName,

    pub postal_address: Option<PostalAddress>,
}

impl From<PatientDto> for Patient {
    fn from(dto: PatientDto) -> Self {
        // TODO remove
        let fake_date = None;

        Self {
            birthdate: dto
                .birthdate
                .map(|birthdate_datetime| birthdate_datetime.date_naive()),
            contact_information: dto.contact_information,
            created_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
            id: match dto.id {
                Some(uuid) => Uuid::parse_str(&uuid).unwrap_or(Uuid::new_v4()), // TODO do we really want to create a new uuid?
                None => Uuid::new_v4(),
            },
            note: dto.note,
            personal_name: dto.personal_name,
            postal_address: dto.postal_address,
            updated_at: match fake_date {
                Some(updated_at) => updated_at,
                None => Utc::now(),
            },
        }
    }
}

/*
   t.string   "profession"
   t.string   "national_insurance_number"
   t.integer  "family_doctor_id"
   t.integer  "physical_therapists_id"
   t.integer  "osteopath_id"
   t.integer  "legal_guardian_id"
   t.boolean  "diabetic"
   t.boolean  "universal_healthcare_coverage"
   t.boolean  "long_duration_disease"
   t.integer  "diabete_network_id"
*/

#[cfg(test)]
mod patient_tests {

    use log::debug;

    use super::*;

    #[test]
    fn test_patient() {
        let patient = Patient {
            birthdate: None,
            contact_information: todo!(),
            created_at: todo!(),
            id: todo!(),
            note: todo!(),
            personal_name: todo!(),
            postal_address: todo!(),
            updated_at: todo!(),
        };

        debug!("{:?}", patient.personal_name);
    }
}
