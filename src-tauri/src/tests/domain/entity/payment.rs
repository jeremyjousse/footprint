use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    entity::payment::Payment,
    value_object::{
        payment_method::{BankCheque, PaymentMethod},
        payment_status::PaymentStatus,
    },
};
#[cfg(test)]
pub fn valid_payment_test_helper() -> Payment {
    let bank_cheque = BankCheque {
        account_owner: "Jérémy Jousse".into(),
        bank_name: "Credit de Développeur".into(),
        check_number: "123456789".into(),
        bank_deposit_id: None,
    };
    Payment {
        created_at: Utc::now(),
        consultation_id: Uuid::new_v4(),
        id: Uuid::new_v4(),
        payed_at: Utc::now(),
        // payment_method: PaymentMethod::BankCheque(bank_cheque),
        payment_method: PaymentMethod::Cash,
        amount: 10.0,
        updated_at: Utc::now(),
        status: PaymentStatus::Payed,
    }
}
