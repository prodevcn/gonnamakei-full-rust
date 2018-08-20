pub fn from_snake_case_to_pascal_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    let mut uppercase_next = true;
    for char in input.chars() {
        if char == '_' {
            uppercase_next = true;
            continue;
        }

        if uppercase_next {
            result.push_str(&char.to_uppercase().to_string());
            uppercase_next = false;
        } else {
            result.push(char);
        }
    }

    result
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn from_camel_or_pascal_case_to_snake_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    let mut previous_uppercase = true;
    for char in input.chars() {
        if char.is_uppercase() {
            if !previous_uppercase {
                result.push('_');
            }
            result.push_str(&char.to_lowercase().to_string());
            previous_uppercase = true;
        } else {
            result.push(char);
            previous_uppercase = false;
        }
    }

    result
}
