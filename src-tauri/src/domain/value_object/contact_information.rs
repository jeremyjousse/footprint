use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactInformation {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(custom(function = "validate_phone_number"))]
    pub mobile_phone: Option<String>,

    #[validate(custom(function = "validate_phone_number"))]
    pub phone: Option<String>,
}

fn validate_phone_number(phone: &Option<String>) -> Result<(), ValidationError> {
    if phone.is_some() {
        let number = phonenumber::parse(None, phone.as_ref().unwrap()).unwrap();
        if !phonenumber::is_valid(&number) {
            return Err(ValidationError::new("phone_number"));
        }
    }

    Ok(())
}
