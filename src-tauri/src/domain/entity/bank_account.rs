use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::domain::helper::validation::validate_sanitized_string;

#[derive(Clone, Debug, Validate)]
pub struct BankAccount {
    pub id: Uuid,
    #[validate(
        length(
            min = 2,
            max = 255,
            message = "Bank name must be at least 2 characters long."
        ),
        custom(function = "validate_sanitized_string")
    )]
    pub bank_name: String,
    #[validate(
        length(
            min = 2,
            max = 255,
            message = "Account number must be at least 2 characters long."
        ),
        custom(function = "validate_sanitized_string")
    )]
    pub account_number: String, // TODO only allow numbers and 0
    pub bank_cheque_deposit_number: u32,
}

#[cfg(test)]
mod bank_account_tests {

    use super::*;

    fn valid_bank_account_test_helper() -> BankAccount {
        BankAccount {
            id: Uuid::new_v4(),
            bank_name: "Banque du Nord".into(),
            account_number: "00 345 055 334".into(),
            bank_cheque_deposit_number: 45678,
        }
    }

    fn none_valid_bank_account_test_helper() -> BankAccount {
        BankAccount {
            id: Uuid::new_v4(),
            bank_name: "<b>Banque du Sud</b>".into(),
            account_number: "bad number".into(),
            bank_cheque_deposit_number: 0,
        }
    }

    #[test]
    fn test_bank_account_is_verified() {
        let valid_bank_account = valid_bank_account_test_helper();
        let none_valid_bank_account = none_valid_bank_account_test_helper();

        assert!(valid_bank_account.validate().is_ok());
        assert!(none_valid_bank_account.validate().is_err());
    }
}
