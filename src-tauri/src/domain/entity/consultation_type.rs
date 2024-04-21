use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

use crate::domain::helper::validation::validate_sanitized_string;

#[derive(Clone, Debug, Validate)]
pub struct ConsultationType {
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
    #[validate(
        length(
            min = 2,
            max = 255,
            message = "Name must be at least 2 characters long."
        ),
        custom(function = "validate_sanitized_string")
    )]
    pub name: String,
    pub price: f64,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod consultation_type_tests {

    use super::*;

    fn valid_consultation_type_test_helper() -> ConsultationType {
        ConsultationType {
            created_at: Utc::now(),
            id: Uuid::new_v4(),
            name: String::from("Hello Jean-Michel"),
            price: 120.55,
            updated_at: Utc::now(),
        }
    }

    fn none_valid_consultation_type_test_helper() -> ConsultationType {
        ConsultationType {
            created_at: Utc::now(),
            id: Uuid::new_v4(),
            name: String::from("Hello <b>Jean-Michel</b>"),
            price: 120.25,
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_consultation_type_is_verified() {
        let valid_consultation_type = valid_consultation_type_test_helper();

        let none_valid_consultation_type = none_valid_consultation_type_test_helper();

        assert!(valid_consultation_type.validate().is_ok());
        assert!(none_valid_consultation_type.validate().is_err());
    }
}
