use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PersonalName {
    #[validate(length(
        min = 2,
        max = 255,
        message = "First name must be at least 2 characters long."
    ))]
    pub first_name: String,

    #[validate(length(
        min = 2,
        max = 255,
        message = "Last name must be at least 2 characters long."
    ))]
    pub last_name: String,

    pub title: Option<String>,
}
