// THIS CLASS MUST MATCH THE SAME IN COMMONS: commons::errors::codes

// CLIENT ---------------------------------------------------------------------
export const CLIENT_NETWORK_ERROR_CODE = "network_error";

// INTERNAL -------------------------------------------------------------------
// No mapped.

// AUTHORIZATION --------------------------------------------------------------
export const AUTHORIZATION_MISSING_HEADER_ERROR_CODE: string = "missing_header";
export const AUTHORIZATION_INCORRECT_HEADER_FORMAT_ERROR_CODE: string = "incorrect_header_format";
export const AUTHORIZATION_INCORRECT_TOKEN_TYPE_ERROR_CODE: string = "incorrect_token_type";
export const AUTHORIZATION_INVALID_PERMISSIONS_ERROR_CODE: string = "invalid_permissions";
export const AUTHORIZATION_TOO_MANY_REQUESTS_ERROR_CODE: string = "too_many_request";
export const AUTHORIZATION_TOO_MANY_RETRIES_ERROR_CODE: string = "too_many_retries";
export const AUTHORIZATION_BANNED_ERROR_CODE: string = "banned";

// INPUT_VALIDATION -----------------------------------------------------------
export const INPUT_VALIDATION_CANNOT_MODIFY_VALUE_ERROR_CODE: string = "cannot_modify";
export const INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE: string = "incorrect_value";
export const INPUT_VALIDATION_MISSING_VALUE_ERROR_CODE: string = "missing_value";
export const INPUT_VALIDATION_DUPLICATED_VALUE_ERROR_CODE: string = "duplicated_value";
export const INPUT_VALIDATION_EXPIRED_VALUE_ERROR_CODE: string = "expired_value";
export const INPUT_VALIDATION_UNEXPIRED_VALUE_ERROR_CODE: string = "unexpired_value";
export const INPUT_VALIDATION_UNDEFINED_MUTEX_ERROR_CODE: string = "undefined_mutex";
export const INPUT_VALIDATION_UNDEFINED_CHALLENGE_ERROR_CODE: string = "undefined_challenge";
export const INPUT_VALIDATION_INCORRECT_FORMAT_ERROR_CODE: string = "incorrect_format";
export const INPUT_VALIDATION_NOT_ENOUGH_CHARS_ERROR_CODE: string = "not_enough_characters";
export const INPUT_VALIDATION_TOO_MANY_CHARS_ERROR_CODE: string = "too_many_characters";
export const INPUT_VALIDATION_NOT_ENOUGH_ELEMENTS_ERROR_CODE: string = "not_enough_elements";
export const INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE: string = "too_many_elements";
export const INPUT_VALIDATION_TOO_LARGE_ERROR_CODE: string = "too_large";
export const INPUT_VALIDATION_WITHOUT_NUMBERS_ERROR_CODE: string = "without_numbers";
export const INPUT_VALIDATION_WITHOUT_SYMBOLS_ERROR_CODE: string = "without_symbols";
export const INPUT_VALIDATION_WITHOUT_LETTERS_ERROR_CODE: string = "without_letters";
export const INPUT_VALIDATION_UNSUPPORTED_COUNTRY_ERROR_CODE: string = "unsupported_country";
export const INPUT_VALIDATION_NOT_ENOUGH_CREDITS_ERROR_CODE: string = "not_enough_credits";
export const INPUT_VALIDATION_LOCKED_ELEMENT_ERROR_CODE: string = "locked_element";
export const INPUT_VALIDATION_UNSUPPORTED_CODEC_ERROR_CODE: string = "unsupported_codec";
