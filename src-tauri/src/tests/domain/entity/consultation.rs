use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    entity::consultation::Consultation,
    value_object::{
        consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
    },
};

#[cfg(test)]

pub fn valid_consultation_test_helper() -> Consultation {
    Consultation {
        appointment_date_time: Utc::now(),
        consultation_type: "POD".into(),
        created_at: Utc::now(),
        id: Uuid::new_v4(),
        location: ConsultationLocation::Clinic,
        note: Some("As usual, the best patient ever.".into()),
        patient_id: Uuid::new_v4(),
        price: 30.05,
        status: ConsultationStatus::Done,
        updated_at: Utc::now(),
    }
}
