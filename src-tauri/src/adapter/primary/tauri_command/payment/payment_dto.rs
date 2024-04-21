use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    entity::payment::Payment,
    helper::data_time::date_time_serializer,
    value_object::{payment_method::PaymentMethod, payment_status::PaymentStatus},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDto {
    pub id: Option<String>,
    pub consultation_id: Uuid,
    #[serde(with = "date_time_serializer")]
    pub payed_at: NaiveDateTime,
    pub payment_method: String,
    pub payment_bank_cheque: Option<BankChequeDto>,
    pub amount: f64,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankChequeDto {
    pub account_owner: String,
    pub bank_name: String,
    pub check_number: String,
    pub bank_deposit_id: Option<String>,
}

impl From<PaymentDto> for Payment {
    fn from(value: PaymentDto) -> Self {
        let fake_date = None;

        // TODO

        Payment {
            created_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
            consultation_id: value.consultation_id,
            id: match value.id {
                // TODO refactor as multiple usage
                Some(uuid) => Uuid::parse_str(&uuid).unwrap_or(Uuid::new_v4()), // TODO do we really want to create a new uuid?
                None => Uuid::new_v4(),
            },
            payed_at: value.payed_at.and_utc(),
            payment_method: PaymentMethod::Cash, // TODO
            amount: value.amount,
            updated_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
            status: PaymentStatus::from_str(&value.status).unwrap(),
        }
    }
}

impl From<Payment> for PaymentDto {
    fn from(value: Payment) -> Self {
        PaymentDto {
            id: Some(value.id.to_string()),
            consultation_id: value.consultation_id,
            payed_at: value.payed_at.naive_utc(),
            payment_method: value.payment_method.to_string(),
            payment_bank_cheque: None,
            amount: value.amount,
            status: value.status.to_string(),
        }
    }
}
