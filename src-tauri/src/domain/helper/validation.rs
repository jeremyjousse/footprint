use validator::ValidationError;

pub fn validate_sanitized_string(input: &str) -> Result<(), ValidationError> {
    let forbidden_chars: Vec<char> = input
        .chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace() && *c != '-')
        .collect();

    if !forbidden_chars.is_empty() {
        return Err(ValidationError::new("not_special_char"));
    }

    Ok(())
}

pub fn validate_sanitized_option_string(input: String) -> Result<(), ValidationError> {
    let forbidden_chars: Vec<char> = input
        .chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace() && *c != '-')
        .collect();

    if !forbidden_chars.is_empty() {
        return Err(ValidationError::new("not_special_char"));
    }

    Ok(())
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn validate_sanitized_string_test() {
        let validation_error = validate_sanitized_string("🧰Hello <b>Bob</b>");
        let validation_ok = validate_sanitized_string("Hello Jean-Michel");

        assert!(validation_error.is_err());
        assert!(validation_ok.is_ok());
    }
}
