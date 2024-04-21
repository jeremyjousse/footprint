use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_object::{
    contact_information::ContactInformation, personal_name::PersonalName,
    postal_address::PostalAddress,
};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Patient {
    pub birthdate: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    #[validate(nested)]
    pub contact_information: ContactInformation,
    pub diabetic: bool,
    pub id: Uuid,
    pub long_duration_disease: bool,
    pub national_insurance_number: Option<String>,
    pub notes: String, // TODO Option<String>
    #[validate(nested)]
    pub personal_name: PersonalName,
    pub profession: Option<String>,
    #[validate(nested)]
    pub postal_address: PostalAddress,
    pub updated_at: DateTime<Utc>,
}

/*
   t.integer  "family_doctor_id"
   t.integer  "physical_therapists_id"
*/

#[cfg(test)]
mod patient_tests {

    use crate::tests::domain::entity::patient::valid_patient_test_helper;

    use super::*;

    #[test]
    fn patient_is_verified_test() {
        let patient = valid_patient_test_helper();
        assert!(patient.validate().is_ok());
    }
}
