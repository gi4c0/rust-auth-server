use validator::{ValidationErrors, ValidationErrorsKind};

pub fn format_errors(validation_errs: &ValidationErrors) -> Vec<String> {
    validation_errs
        .errors()
        .iter()
        .map(|(field, error_kind)| match error_kind {
            ValidationErrorsKind::Field(errors) => {
                let errors: String = errors
                    .iter()
                    .map(|e| e.message.as_ref().unwrap_or(&e.code).to_string())
                    .collect::<Vec<String>>()
                    .join("; ");

                format!("{field}: {}", errors)
            }

            ValidationErrorsKind::Struct(validated_struct) => {
                let errors: String = format_errors(validated_struct).join("; ");
                format!("{field}: {}", errors)
            }

            ValidationErrorsKind::List(list) => {
                let errors: String = list
                    .values()
                    .map(|e| format_errors(e).join("; "))
                    .collect::<Vec<String>>()
                    .join("; ");

                format!("{field}: {}", errors)
            }
        })
        .collect()
}
