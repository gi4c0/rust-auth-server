use std::{borrow::Cow, collections::HashMap};

use validator::ValidationError;

use crate::domains::user::{Email, Password, Username};

pub fn validate_username(username: &Username) -> Result<(), ValidationError> {
    let username_len = username.as_ref().len();

    if username_len < 3 || username_len > 50 {
        return Err(ValidationError::new("invalid_username"));
    }

    Ok(())
}

pub fn validate_email(email: &Email) -> Result<(), ValidationError> {
    if !validator::validate_email(email.as_ref()) {
        return Err(ValidationError {
            code: Cow::from("invalid_email"),
            params: HashMap::new(),
            message: Some(Cow::from("Provided Invalid email")),
        });
    }

    Ok(())
}

pub fn validate_password(password: &Password) -> Result<(), ValidationError> {
    let len = password.as_ref().len();

    if len < 8 || len > 50 {
        return Err(ValidationError {
            code: Cow::from("invalid_password"),
            message: Some(Cow::from(
                "Password length expected to be more than 8 and less than 50 chars length",
            )),
            params: HashMap::new(),
        });
    }

    Ok(())
}
