use std::str::FromStr;

use chrono::{NaiveDateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    entity::payment::Payment,
    value_object::{payment_method::PaymentMethod, payment_status::PaymentStatus},
};

use super::payment_schema::payments;

#[derive(
    Clone, Debug, Deserialize, Identifiable, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
#[diesel(table_name = payments)]
#[diesel(primary_key(id))]
pub struct DbPayment {
    pub amount: f64,
    pub consultation_id: String,
    pub created_at: NaiveDateTime,
    pub id: String,
    pub payed_at: NaiveDateTime,
    pub payment_method: String,
    pub status: String,
    pub updated_at: NaiveDateTime,
}

impl From<Payment> for DbPayment {
    fn from(value: Payment) -> DbPayment {
        DbPayment {
            created_at: Utc::now().naive_utc(), // TODO ???: use the provided timestamp
            id: value.id.to_string(),
            updated_at: Utc::now().naive_utc(),
            amount: value.amount,
            consultation_id: value.consultation_id.to_string(),
            payed_at: value.payed_at.naive_utc(),
            payment_method: value.payment_method.to_string(),
            status: value.status.to_string(),
        }
    }
}

impl From<DbPayment> for Payment {
    fn from(value: DbPayment) -> Payment {
        Payment {
            id: uuid::Uuid::parse_str(&value.id).unwrap(),
            created_at: value.created_at.and_utc(),
            consultation_id: Uuid::from_str(&value.consultation_id).unwrap(),
            payed_at: value.payed_at.and_utc(),
            payment_method: PaymentMethod::from_str(&value.payment_method).unwrap(),
            amount: value.amount,
            updated_at: value.updated_at.and_utc(),
            status: PaymentStatus::from_str(&value.status).unwrap(),
        }
    }
}

#[cfg(test)]
mod db_payment_tests {

    use crate::tests::domain::entity::payment::valid_payment_test_helper;

    use super::*;
    #[test]
    fn it_parses_db_payment_from_payment() {
        let payment = valid_payment_test_helper();
        let db_payment: DbPayment = payment.into();

        println!("{:?}", db_payment);
    }
}
