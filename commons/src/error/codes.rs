use arcstr::ArcStr;

// THIS CLASS MUST MATCH THE SAME IN WEB-CLIENT: APIErrorCodes.ts

// INTERNAL -------------------------------------------------------------------
pub const INTERNAL_CANNOT_WRITE_ERROR_CODE: ArcStr = arcstr::literal!("cannot_write");
pub const INTERNAL_INCOMPLETE_ERROR_CODE: ArcStr = arcstr::literal!("incomplete");
pub const INTERNAL_UNREACHABLE_ERROR_CODE: ArcStr = arcstr::literal!("unreachable");
pub const INTERNAL_INCORRECT_MESSAGE_ERROR_CODE: ArcStr = arcstr::literal!("incorrect_message");
pub const INTERNAL_CANNOT_LOCK_ERROR_CODE: ArcStr = arcstr::literal!("cannot_lock");
pub const INTERNAL_AQL_MAX_RETRIES_REACHED_ERROR_CODE: ArcStr =
    arcstr::literal!("max_retries_reached");
pub const INTERNAL_DB_ERROR_CODE: ArcStr = arcstr::literal!("error_id_db");
pub const INTERNAL_NOT_ENOUGH_CREDITS_ERROR_CODE: ArcStr = arcstr::literal!("not_enough_credits");

// AUTHORIZATION --------------------------------------------------------------
pub const AUTHORIZATION_MISSING_HEADER_ERROR_CODE: ArcStr = arcstr::literal!("missing_header");
pub const AUTHORIZATION_INCORRECT_HEADER_FORMAT_ERROR_CODE: ArcStr =
    arcstr::literal!("incorrect_header_format");
pub const AUTHORIZATION_INCORRECT_TOKEN_TYPE_ERROR_CODE: ArcStr =
    arcstr::literal!("incorrect_token_type");
pub const AUTHORIZATION_INVALID_PERMISSIONS_ERROR_CODE: ArcStr =
    arcstr::literal!("invalid_permissions");
pub const AUTHORIZATION_TOO_MANY_REQUESTS_ERROR_CODE: ArcStr = arcstr::literal!("too_many_request");
pub const AUTHORIZATION_TOO_MANY_RETRIES_ERROR_CODE: ArcStr = arcstr::literal!("too_many_retries");
pub const AUTHORIZATION_BANNED_ERROR_CODE: ArcStr = arcstr::literal!("banned");

// INPUT_VALIDATION -----------------------------------------------------------
pub const INPUT_VALIDATION_CANNOT_MODIFY_VALUE_ERROR_CODE: ArcStr =
    arcstr::literal!("cannot_modify");
pub const INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE: ArcStr = arcstr::literal!("incorrect_state");
pub const INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE: ArcStr = arcstr::literal!("incorrect_value");
pub const INPUT_VALIDATION_MISSING_VALUE_ERROR_CODE: ArcStr = arcstr::literal!("missing_value");
pub const INPUT_VALIDATION_DUPLICATED_VALUE_ERROR_CODE: ArcStr =
    arcstr::literal!("duplicated_value");
pub const INPUT_VALIDATION_EXPIRED_VALUE_ERROR_CODE: ArcStr = arcstr::literal!("expired_value");
pub const INPUT_VALIDATION_UNEXPIRED_VALUE_ERROR_CODE: ArcStr = arcstr::literal!("unexpired_value");
pub const INPUT_VALIDATION_UNDEFINED_MUTEX_ERROR_CODE: ArcStr = arcstr::literal!("undefined_mutex");
pub const INPUT_VALIDATION_UNDEFINED_CHALLENGE_ERROR_CODE: ArcStr =
    arcstr::literal!("undefined_challenge");
pub const INPUT_VALIDATION_UNDEFINED_PARTICIPANT_ERROR_CODE: ArcStr =
    arcstr::literal!("undefined_participant");
pub const INPUT_VALIDATION_UNDEFINED_INVESTMENT_ERROR_CODE: ArcStr =
    arcstr::literal!("undefined_investment");
pub const INPUT_VALIDATION_UNDEFINED_SIGNATURE_ERROR_CODE: ArcStr =
    arcstr::literal!("undefined_signature");
pub const INPUT_VALIDATION_UNDEFINED_AUTHENTICATION_ERROR_CODE: ArcStr =
    arcstr::literal!("undefined_authentication");
pub const INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE: ArcStr = arcstr::literal!("undefined_bet");
pub const INPUT_VALIDATION_UNDEFINED_EMAIL_ERROR_CODE: ArcStr = arcstr::literal!("undefined_email");
pub const INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE: ArcStr =
    arcstr::literal!("incorrect_format");
pub const INPUT_VALIDATION_NOT_ENOUGH_CHARS_ERROR_CODE: ArcStr =
    arcstr::literal!("not_enough_characters");
pub const INPUT_VALIDATION_TOO_MANY_CHARS_ERROR_CODE: ArcStr =
    arcstr::literal!("too_many_characters");
pub const INPUT_VALIDATION_NOT_ENOUGH_ELEMENTS_ERROR_CODE: ArcStr =
    arcstr::literal!("not_enough_elements");
pub const INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE: ArcStr =
    arcstr::literal!("too_many_elements");
pub const INPUT_VALIDATION_TOO_LARGE_ERROR_CODE: ArcStr = arcstr::literal!("too_large");
pub const INPUT_VALIDATION_WITHOUT_NUMBERS_ERROR_CODE: ArcStr = arcstr::literal!("without_numbers");
pub const INPUT_VALIDATION_WITHOUT_SYMBOLS_ERROR_CODE: ArcStr = arcstr::literal!("without_symbols");
pub const INPUT_VALIDATION_WITHOUT_LETTERS_ERROR_CODE: ArcStr = arcstr::literal!("without_letters");
pub const INPUT_VALIDATION_UNSUPPORTED_COUNTRY_ERROR_CODE: ArcStr =
    arcstr::literal!("unsupported_country");
pub const INPUT_VALIDATION_NOT_ENOUGH_CREDITS_ERROR_CODE: ArcStr =
    arcstr::literal!("not_enough_credits");
pub const INPUT_VALIDATION_LOCKED_ELEMENT_ERROR_CODE: ArcStr = arcstr::literal!("locked_element");
pub const INPUT_VALIDATION_UNSUPPORTED_CODEC_ERROR_CODE: ArcStr =
    arcstr::literal!("unsupported_codec");
