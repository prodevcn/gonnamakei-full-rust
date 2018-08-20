use std::convert::Infallible;

use warp::http::StatusCode;
use warp::reply::Json;
use warp::{Rejection, Reply};

use commons::error::{AppError, AUTHORIZATION_INCORRECT_TOKEN_TYPE_ERROR_CODE};

use crate::error::ServerError;

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, json) = if err.is_not_found() {
        generate_response_for(StatusCode::NOT_FOUND)
    } else if let Some(e) = err.find::<AppError>() {
        (
            e.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            warp::reply::json(e),
        )
    } else if let Some(e) = err.find::<ServerError>() {
        (
            e.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            warp::reply::json(e),
        )
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        generate_response_for(StatusCode::BAD_REQUEST)
    } else if err.find::<warp::reject::LengthRequired>().is_some() {
        generate_response_for(StatusCode::LENGTH_REQUIRED)
    } else if err.find::<warp::reject::MissingHeader>().is_some()
        || err.find::<warp::reject::InvalidQuery>().is_some()
        || err.find::<warp::reject::MissingCookie>().is_some()
    {
        generate_response_for(StatusCode::BAD_REQUEST)
    } else if err.find::<warp::reject::UnsupportedMediaType>().is_some() {
        generate_response_for(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        generate_response_for(StatusCode::METHOD_NOT_ALLOWED)
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        generate_response_for(StatusCode::PAYLOAD_TOO_LARGE)
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        generate_response_for(StatusCode::METHOD_NOT_ALLOWED)
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        (
            StatusCode::BAD_REQUEST,
            warp::reply::json(
                &ServerError::new_with_status(
                    StatusCode::BAD_REQUEST,
                    AUTHORIZATION_INCORRECT_TOKEN_TYPE_ERROR_CODE,
                )
                .message(format!("Missing {} header", e.name()).into()),
            ),
        )
    } else {
        // This should never happen.
        remote_error!("Unhandled rejection: {:?}", err);
        generate_response_for(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(warp::reply::with_status(json, code))
}

fn generate_response_for(code: StatusCode) -> (StatusCode, Json) {
    (
        code,
        warp::reply::json(
            &ServerError::new_with_status(code, format!("HTTP::{}", code.as_str()).into()).message(
                code.canonical_reason()
                    .unwrap_or_else(|| code.as_str())
                    .into(),
            ),
        ),
    )
}
