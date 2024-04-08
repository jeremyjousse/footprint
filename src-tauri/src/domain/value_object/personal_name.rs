use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::helper::validation::validate_sanitized_string;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PersonalName {
    #[validate(
        length(
            min = 2,
            max = 255,
            message = "First name must be at least 2 characters long."
        ),
        custom(function = "validate_sanitized_string")
    )]
    pub first_name: String,

    #[validate(
        length(
            min = 2,
            max = 255,
            message = "Last name must be at least 2 characters long."
        ),
        custom(function = "validate_sanitized_string")
    )]
    pub last_name: String,

    pub title: Option<String>,
}
