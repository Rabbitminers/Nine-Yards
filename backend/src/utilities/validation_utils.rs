use validator::{ValidationErrors, ValidationErrorsKind};

pub fn validation_errors_to_string(errors: ValidationErrors, adder: Option<String>) -> String {
    let mut output = String::new();

    let map = errors.into_errors();

    let key_option = map.keys().next().copied();

    if let Some(field) = key_option {
        if let Some(error) = map.get(field) {
            return match error {
                ValidationErrorsKind::Struct(errors) => {
                    validation_errors_to_string(*errors.clone(), Some(format!("of item {field}")))
                }
                ValidationErrorsKind::List(list) => {
                    if let Some((index, errors)) = list.iter().next() {
                        output.push_str(&validation_errors_to_string(
                            *errors.clone(),
                            Some(format!("of list {field} with index {index}")),
                        ));
                    }

                    output
                }
                ValidationErrorsKind::Field(errors) => {
                    if let Some(error) = errors.get(0) {
                        if let Some(adder) = adder {
                            output.push_str(&format!(
                                "Field {} {} failed validation with error: {}",
                                field, adder, error.code
                            ));
                        } else {
                            output.push_str(&format!(
                                "Field {} failed validation with error: {}",
                                field, error.code
                            ));
                        }
                    }

                    output
                }
            };
        }
    }

    String::new()
}

pub fn validate_url(value: &str) -> Result<(), validator::ValidationError> {
    let url = url::Url::parse(value)
        .ok()
        .ok_or_else(|| validator::ValidationError::new("invalid URL"))?;
    
    if url.scheme() != "https" {
        return Err(validator::ValidationError::new("URL must be https"));
    }

    Ok(())
}

pub fn validate_name(value: &str) -> Result<(), validator::ValidationError> {
    if value.trim().is_empty() {
        return Err(validator::ValidationError::new(
            "Name cannot contain only whitespace.",
        ));
    }

    Ok(())
}