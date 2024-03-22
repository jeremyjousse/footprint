use chrono::{DateTime, NaiveDate, Utc};

use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_object::{
    contact_information::ContactInformation, personal_name::PersonalName,
    postal_address::PostalAddress,
};

#[derive(Clone, Debug, Serialize, Validate)]
pub struct Patient {
    pub birthdate: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    #[validate]
    pub contact_information: ContactInformation,
    pub diabetic: bool,
    pub id: Uuid,
    pub long_duration_disease: bool,
    pub national_insurance_number: Option<String>,
    pub notes: String,
    #[validate]
    pub personal_name: PersonalName,
    pub profession: Option<String>,
    #[validate]
    pub postal_address: PostalAddress,
    pub updated_at: DateTime<Utc>,
}

/*
   t.integer  "family_doctor_id"
   t.integer  "physical_therapists_id"
*/

#[cfg(test)]
mod patient_tests {

    use log::debug;

    use super::*;

    #[test]
    fn test_patient() {
        // let patient = Patient {
        //     birthdate: None,
        //     contact_information: todo!(),
        //     created_at: todo!(),
        //     id: todo!(),
        //     note: todo!(),
        //     personal_name: todo!(),
        //     postal_address: todo!(),
        //     updated_at: todo!(),
        // };

        // debug!("{:?}", patient.personal_name);
    }
}
