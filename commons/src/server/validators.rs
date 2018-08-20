use regex::{Regex, RegexBuilder};

use crate::error::AppResult;
use crate::error::{
    AppError, INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE,
    INPUT_VALIDATION_NOT_ENOUGH_CHARS_ERROR_CODE, INPUT_VALIDATION_NOT_ENOUGH_ELEMENTS_ERROR_CODE,
    INPUT_VALIDATION_TOO_MANY_CHARS_ERROR_CODE, INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE,
    INPUT_VALIDATION_WITHOUT_LETTERS_ERROR_CODE, INPUT_VALIDATION_WITHOUT_NUMBERS_ERROR_CODE,
    INPUT_VALIDATION_WITHOUT_SYMBOLS_ERROR_CODE,
};

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        RegexBuilder::new(r#"^[\w\.%+-]+@[\w-]+(?:\.[\w-]+)*\.[a-z]+$"#)
            .case_insensitive(true)
            .unicode(true)
            .build()
            .unwrap();
    static ref WEBHOOK_REGEX: Regex =
        RegexBuilder::new(r#"^https://(?:[\w-]+(?:\.[\w-]+)*\.[a-z]+)(?:/[^?#]*)$"#)
            .case_insensitive(true)
            .unicode(true)
            .build()
            .unwrap();
}

pub fn string_length_validator(
    value: &str,
    min: usize,
    max: usize,
    field: &'static str,
) -> AppResult<()> {
    let length = value.chars().count();

    if length < min {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_NOT_ENOUGH_CHARS_ERROR_CODE,
        )
        .message(format!("The length is less than {} characters", min).into())
        .param(field.into()));
    }

    if length > max {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_TOO_MANY_CHARS_ERROR_CODE,
        )
        .message(format!("The length is greater than {} characters", max).into())
        .param(field.into()));
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn code_2fa_validator(value: &str, field: &'static str) -> AppResult<()> {
    string_length_validator(value, 8, 8, field)?;

    for char in field.chars() {
        if matches!(char, '0'..='9') {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE,
            )
            .message(arcstr::literal!("The code is in an incorrect format"))
            .param(field.into()));
        }
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn length_validator(
    length: usize,
    min: usize,
    max: usize,
    field: &'static str,
) -> AppResult<()> {
    if length < min {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_NOT_ENOUGH_ELEMENTS_ERROR_CODE,
        )
        .message(format!("The length is less than {}", min).into())
        .param(field.into()));
    }

    if length > max {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE,
        )
        .message(format!("The length is greater than {}", max).into())
        .param(field.into()));
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn username_validator(username: &str, field: &'static str) -> AppResult<()> {
    string_length_validator(username, 3, 20, field)?;

    for char in username.chars() {
        if !char.is_ascii_alphanumeric() && char != '_' && char != '-' {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE,
            )
            .message(arcstr::literal!(
                "Can only contain simple characters [a..zA..Z_-]"
            ))
            .param(field.into()));
        }
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn email_validator(email: &str, field: &'static str) -> AppResult<()> {
    string_length_validator(email, 3, 60, field)?;

    if !EMAIL_REGEX.is_match(email) {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE,
        )
        .message(arcstr::literal!("Not an email"))
        .param(field.into()));
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn password_validator(password: &str, field: &'static str) -> AppResult<()> {
    string_length_validator(password, 8, 30, field)?;

    let mut has_numbers = false;
    let mut has_symbol = false;
    let mut has_letter = false;

    for char in password.chars() {
        if char.is_numeric() {
            has_numbers = true;
        } else if char.is_alphabetic() {
            has_letter = true;
        } else {
            has_symbol = true;
        }
    }

    if !has_numbers {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_WITHOUT_NUMBERS_ERROR_CODE,
        )
        .message(arcstr::literal!("Must contain at least one number"))
        .param(field.into()));
    }

    if !has_symbol {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_WITHOUT_SYMBOLS_ERROR_CODE,
        )
        .message(arcstr::literal!("Must contain at least one symbol"))
        .param(field.into()));
    }

    if !has_letter {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_WITHOUT_LETTERS_ERROR_CODE,
        )
        .message(arcstr::literal!("Must contain at least one letter"))
        .param(field.into()));
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn webhook_validator(webhook: &str, field: &'static str) -> AppResult<()> {
    string_length_validator(webhook, 12, 1000, field)?;

    if !WEBHOOK_REGEX.is_match(webhook) {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE,
        )
        .message(arcstr::literal!("Not a valid webhook url"))
        .param(field.into()));
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_email_validator() {
        let email = "this.is+%-an_email@of.wapads.app";
        assert!(email_validator(email, "email").is_ok(), "1");

        let email = " this.is-an_email@of.wapads.app ";
        assert!(email_validator(email, "email").is_err(), "2");
    }

    #[test]
    pub fn test_webhook_validator() {
        let url = "https://web.wapads.app/path/to/file";
        assert!(webhook_validator(url, "url").is_ok(), "1");

        let url = "https://web.wapads.app/path/to/file/";
        assert!(webhook_validator(url, "url").is_ok(), "2");

        let url = "https://web.wapads.app/path/to/file?param";
        assert!(webhook_validator(url, "url").is_err(), "3");

        let url = "https://web.wapads.app/path/to/file#hashtag";
        assert!(webhook_validator(url, "url").is_err(), "4");

        let url = "http://web.wapads.app/path/to/file";
        assert!(webhook_validator(url, "url").is_err(), "5");

        let url = "web.wapads.app/path/to/file";
        assert!(webhook_validator(url, "url").is_err(), "6");

        let url = " https://web.wapads.app/path/to/file ";
        assert!(webhook_validator(url, "url").is_err(), "7");
    }
}
