use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    helper::validation::validate_sanitized_string,
    value_object::{
        consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
    },
};

use super::{consultation_type::ConsultationType, patient::Patient};

// TODO add validations
#[derive(Clone, Debug, Validate)]
pub struct Consultation {
    pub appointment_date_time: DateTime<Utc>,
    pub consultation_type: String,
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
    pub location: ConsultationLocation,
    pub note: Option<String>,
    pub patient_id: Uuid,
    pub price: f64,
    pub status: ConsultationStatus,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod consultation_tests {

    use crate::tests::domain::entity::consultation::valid_consultation_test_helper;

    use super::*;

    #[test]
    fn consultation_is_verified() {
        assert!(valid_consultation_test_helper().validate().is_ok());
    }
}
