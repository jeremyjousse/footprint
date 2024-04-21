use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_object::{
    consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
};

// TODO add validations
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
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
