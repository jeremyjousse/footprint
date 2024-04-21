use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_object::{payment_method::PaymentMethod, payment_status::PaymentStatus};

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct Payment {
    pub created_at: DateTime<Utc>,
    pub consultation_id: Uuid,
    pub id: Uuid,
    pub payed_at: DateTime<Utc>,
    pub payment_method: PaymentMethod,
    pub amount: f64,
    pub updated_at: DateTime<Utc>,
    pub status: PaymentStatus,
}

#[cfg(test)]
mod payment_tests {
    use crate::tests::domain::entity::payment::valid_payment_test_helper;

    use super::*;

    #[test]
    fn payment_is_valid_test() {
        let payment = valid_payment_test_helper();
        assert!(payment.validate().is_ok());
    }
}
