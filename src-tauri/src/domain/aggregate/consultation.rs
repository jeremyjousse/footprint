use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    adapter::primary::tauri_command::{
        consultation::consultation_dto::ConsultationDto, payment::payment_dto::PaymentDto,
    },
    domain::{
        entity::payment::Payment,
        value_object::{
            consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
        },
    },
};

// TODO add validations
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct ConsultationAggregate {
    pub consultation: ConsultationDto,
    pub payments: Vec<PaymentDto>,
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
