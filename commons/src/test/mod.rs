use std::fmt::Debug;
use std::io::Read;

use arcstr::ArcStr;
use libflate::gzip::Decoder;
use serde::Deserialize;
use warp::http::header::CONTENT_TYPE;
use warp::http::{Response, StatusCode};
use warp::hyper::body::Bytes;

use crate::database::types::DateTime;
use crate::database::{APIDocument, APIReference, DBDocument, DBReference, NullableOption};
use crate::error::AppError;

pub mod commons;

pub fn assert_http_response_status(
    response: &Response<Bytes>,
    status: StatusCode,
    compressed: bool,
) {
    if compressed {
        let encoded_data = response.body();
        let mut decoder = Decoder::new(&encoded_data[..]).expect("The body cannot be decoded (1)");
        let mut decoded_data = Vec::new();
        decoder
            .read_to_end(&mut decoded_data)
            .expect("The body cannot be decoded (2)");

        assert_eq!(
            response.status(),
            status,
            "The status code is incorrect: {}",
            String::from_utf8(decoded_data).unwrap_or_else(|_| "Body is not a utf8 string".into())
        );
    } else {
        assert_eq!(
            response.status(),
            status,
            "The status code is incorrect: {}",
            String::from_utf8(response.body().to_vec())
                .unwrap_or_else(|_| "Body is not a utf8 string".into())
        );
    }
}

pub fn assert_http_response_content_type(
    response: &Response<Bytes>,
    file_type: &str,
    compressed: bool,
) {
    let header = response
        .headers()
        .get(CONTENT_TYPE)
        .expect("The content-type header is missing");
    let header = header
        .to_str()
        .expect("The value of the header is not a valid string");

    if compressed {
        let encoded_data = response.body();
        let mut decoder = Decoder::new(&encoded_data[..]).expect("The body cannot be decoded (1)");
        let mut decoded_data = Vec::new();
        decoder
            .read_to_end(&mut decoded_data)
            .expect("The body cannot be decoded (2)");

        assert_eq!(
            header,
            file_type,
            "The content-type header is incorrect: {}",
            String::from_utf8(decoded_data).unwrap_or_else(|_| "Body is not a utf8 string".into())
        );
    } else {
        assert_eq!(
            header,
            file_type,
            "The content-type header is incorrect: {}",
            String::from_utf8(response.body().to_vec())
                .unwrap_or_else(|_| "Body is not a utf8 string".into())
        );
    }
}

pub fn assert_http_body<T: for<'de> Deserialize<'de>>(
    response: &Response<Bytes>,
    compressed: bool,
) -> T {
    if compressed {
        let encoded_data = response.body();
        let mut decoder = Decoder::new(&encoded_data[..]).expect("The body cannot be decoded (1)");
        let mut decoded_data = Vec::new();
        decoder
            .read_to_end(&mut decoded_data)
            .expect("The body cannot be decoded (2)");

        serde_json::de::from_slice(decoded_data.as_slice()).expect("The body is incorrect")
    } else {
        serde_json::de::from_slice(response.body().as_ref()).expect("The body is incorrect")
    }
}

pub fn assert_http_body_text(response: &Response<Bytes>, compressed: bool) -> String {
    if compressed {
        let encoded_data = response.body();
        let mut decoder = Decoder::new(&encoded_data[..]).expect("The body cannot be decoded (1)");
        let mut decoded_data = Vec::new();
        decoder
            .read_to_end(&mut decoded_data)
            .expect("The body cannot be decoded (2)");

        std::str::from_utf8(decoded_data.as_slice())
            .expect("The body is incorrect")
            .to_string()
    } else {
        std::str::from_utf8(response.body().as_ref())
            .expect("The body is incorrect")
            .to_string()
    }
}

pub fn assert_http_body_bytes(response: &Response<Bytes>, compressed: bool) -> Vec<u8> {
    if compressed {
        let encoded_data = response.body();
        let mut decoder = Decoder::new(&encoded_data[..]).expect("The body cannot be decoded (1)");
        let mut decoded_data = Vec::new();
        decoder
            .read_to_end(&mut decoded_data)
            .expect("The body cannot be decoded (2)");

        decoded_data
    } else {
        response.body().to_vec()
    }
}

pub fn assert_error(response: &Response<Bytes>, code: ArcStr, compressed: bool) -> AppError {
    let error: AppError = assert_http_body(response, compressed);

    assert_eq!(error.code, code, "The error code is incorrect");

    error
}

pub fn assert_error_param(error: &AppError, param: Option<ArcStr>) {
    assert_eq!(error.param, param, "The error param is incorrect");
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn assert_db_equal(now: &Option<ArcStr>, pre: &Option<ArcStr>) {
    assert_eq!(now, pre, "The object cannot change")
}

pub fn assert_db_not_equal(now: &Option<ArcStr>, pre: &Option<ArcStr>) {
    assert_ne!(now, pre, "The object must change")
}

pub fn assert_db_value<T: PartialEq + Debug>(
    property: &NullableOption<T>,
    value: &T,
    property_name: &str,
) {
    let property_value = match property {
        NullableOption::Value(v) => v,
        _ => panic!("The property '{}' does not contain a value", property_name),
    };

    assert_eq!(
        property_value, value,
        "The property '{}' is incorrect",
        property_name
    );
}

pub fn assert_db_value_neq<T: PartialEq + Debug>(
    property: &NullableOption<T>,
    value: &T,
    property_name: &str,
) {
    let property_value = match property {
        NullableOption::Value(v) => v,
        _ => panic!("The property '{}' does not contain a value", property_name),
    };

    assert_ne!(
        property_value, value,
        "The property '{}' is incorrect",
        property_name
    );
}

pub fn assert_db_reference<T: DBDocument + Debug>(
    property: &NullableOption<DBReference<T>>,
    value: &DBReference<T>,
    is_empty: bool,
    property_name: &str,
) {
    let property_value = match property {
        NullableOption::Value(v) => v,
        _ => panic!("The property '{}' does not contain a value", property_name),
    };

    assert_eq!(
        property_value, value,
        "The property '{}' is incorrect",
        property_name
    );

    if is_empty {
        match property_value {
            DBReference::Key(_) => panic!(
                "The property '{}' is not a document, therefore it cannot be empty",
                property_name
            ),
            DBReference::Document(document) => {
                assert!(
                    document.is_all_missing(),
                    "The property '{}' is not empty",
                    property_name
                )
            }
        }
    }
}

pub fn assert_api_reference<T: APIDocument + Debug>(
    property: &NullableOption<APIReference<T>>,
    value: &APIReference<T>,
    is_empty: bool,
    property_name: &str,
) {
    let property_value = match property {
        NullableOption::Value(v) => v,
        _ => panic!("The property '{}' does not contain a value", property_name),
    };

    assert_eq!(
        property_value, value,
        "The property '{}' is incorrect",
        property_name
    );

    if is_empty {
        match property_value {
            APIReference::Key(_) => panic!(
                "The property '{}' is not a document, therefore it cannot be empty",
                property_name
            ),
            APIReference::Document(document) => {
                assert!(
                    document.is_all_missing(),
                    "The property '{}' is not empty",
                    property_name
                )
            }
        }
    }
}

pub fn assert_db_value_comparing<T, F>(
    property: &NullableOption<T>,
    property_name: &str,
    comparing_fn: F,
) where
    F: FnOnce(&T) -> bool,
{
    let property_value = match property {
        NullableOption::Value(v) => v,
        _ => panic!("The property '{}' does not contain a value", property_name),
    };

    assert!(
        comparing_fn(property_value),
        "The property '{}' is incorrect",
        property_name
    );
}

pub fn assert_db_expiration_after<'a>(
    value: &'a NullableOption<DateTime>,
    date: &DateTime,
    property_name: &str,
) {
    let expiration = match value {
        NullableOption::Value(v) => v,
        _ => panic!(
            "The value of the property '{}' does not contain a value",
            property_name
        ),
    };

    assert!(
        date.0 < expiration.0,
        "The expiration of the property '{}' is incorrect",
        property_name
    );
}

pub fn assert_db_expiration_after_now(value: &NullableOption<DateTime>, property_name: &str) {
    assert_db_expiration_after(value, &DateTime::default(), property_name)
}

pub fn assert_db_expiration_before<'a>(
    value: &'a NullableOption<DateTime>,
    date: &DateTime,
    property_name: &str,
) {
    let expiration = match value {
        NullableOption::Value(v) => v,
        _ => panic!(
            "The value of the property '{}' does not contain a value",
            property_name
        ),
    };

    assert!(
        expiration.0 < date.0,
        "The expiration of the property '{}' is incorrect",
        property_name
    );
}

pub fn assert_db_expiration_before_now(value: &NullableOption<DateTime>, property_name: &str) {
    assert_db_expiration_before(value, &DateTime::default(), property_name)
}

pub fn assert_db_is_missing<T>(property: &NullableOption<T>, property_name: &str) {
    assert!(
        property.is_missing(),
        "The property '{}' must be missing",
        property_name
    );
}

pub fn assert_db_is_value<T>(property: &NullableOption<T>, property_name: &str) {
    assert!(
        property.is_value(),
        "The property '{}' must be a value",
        property_name
    );
}
