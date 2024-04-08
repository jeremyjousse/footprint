use chrono::{DateTime, NaiveDate, Utc};

use serde::Serialize;
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
