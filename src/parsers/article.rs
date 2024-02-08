use std::{borrow::Cow, collections::HashMap};

use validator::ValidationError;

pub fn validate_order_by(sorting_direction: &str) -> Result<(), ValidationError> {
    if sorting_direction != "ASC" && sorting_direction != "DESC" {
        return Err(ValidationError {
            code: Cow::from("invalid_sorting_direction"),
            message: Some(Cow::from("Invalid Sorting Direction")),
            params: HashMap::new(),
        });
    }

    Ok(())
}
