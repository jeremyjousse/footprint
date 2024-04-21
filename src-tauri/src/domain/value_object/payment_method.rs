use serde::Deserialize;
use strum_macros::EnumString;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, EnumString, Eq, PartialEq, strum_macros::Display)]
// #[derive(Clone, Debug, Deserialize, EnumString, EnumDiscriminants, Eq, PartialEq, strum_macros::Display)]
// #[strum_discriminants(derive(EnumString, EnumMessage))]
pub enum PaymentMethod {
    BankTransfer,
    // #[strum_discriminants(strum(message = "BankCheque"))]
    // BankCheque(BankCheque),
    Card,
    Cash,
}

// TODO
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Validate)]
pub struct BankCheque {
    pub account_owner: String,
    pub bank_name: String,
    pub check_number: String,
    pub bank_deposit_id: Option<Uuid>,
}
