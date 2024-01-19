use std::{borrow::Cow, collections::HashMap};

use validator::ValidationError;

use crate::domains::user::{Email, Username};

pub fn validate_username(username: &Username) -> Result<(), ValidationError> {
    let username = username.as_ref();
    let username_len = username.len();

    if username_len < 1 || username_len > 50 {
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
