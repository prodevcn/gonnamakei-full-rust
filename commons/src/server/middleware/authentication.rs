use warp::http::header::AUTHORIZATION;
use warp::http::StatusCode;
use warp::Filter;

use crate::database::collections::AuthenticationCollection;
use crate::database::types::{Address, DBUuid, DBUuidType};
use crate::error::{
    AppError, AUTHORIZATION_INCORRECT_HEADER_FORMAT_ERROR_CODE,
    AUTHORIZATION_INVALID_PERMISSIONS_ERROR_CODE,
};
use crate::server::{RequestContext, RequestContextWithAuth};

pub static AUTHENTICATION_BEARER: &str = "GMI ";

pub struct AuthClaims {
    pub address: Address,
}

impl AuthClaims {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(address: Address) -> Self {
        AuthClaims { address }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Validates an API token and returns the associated address.
pub fn validate_api_token<F, C: 'static, R: 'static>(
    filter: F,
) -> impl Filter<Extract = (RequestContextWithAuth<C, R>,), Error = warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static
where
    F: Filter<Extract = (RequestContext<C, R>,), Error = warp::Rejection>
        + Clone
        + Send
        + Sync
        + 'static,
    C: Send,
    R: Send,
{
    filter
        .and(warp::filters::header::header(AUTHORIZATION.as_str()))
        .and(warp::any().map(move || DBUuidType::APIToken))
        .and_then(validate_api_token_process)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Validates a specific token and returns the associated address.
pub fn validate_specific_token<F, C: 'static, R: 'static>(
    filter: F,
    token_type: DBUuidType,
) -> impl Filter<Extract = (RequestContextWithAuth<C, R>,), Error = warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static
where
    F: Filter<Extract = (RequestContext<C, R>,), Error = warp::Rejection>
        + Clone
        + Send
        + Sync
        + 'static,
    C: Send,
    R: Send,
{
    filter
        .and(warp::filters::header::header(AUTHORIZATION.as_str()))
        .and(warp::any().map(move || token_type))
        .and_then(validate_api_token_process)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub(in crate::server::middleware::authentication) async fn validate_api_token_process<C, R>(
    context: RequestContext<C, R>,
    header: String,
    token_type: DBUuidType,
) -> Result<RequestContextWithAuth<C, R>, warp::Rejection> {
    // Validate header and get token from it.
    if !header.starts_with(AUTHENTICATION_BEARER) {
        return Err(AppError::new_with_status(
            StatusCode::UNAUTHORIZED,
            AUTHORIZATION_INCORRECT_HEADER_FORMAT_ERROR_CODE,
        )
        .message(arcstr::literal!("Incorrect Authentication header value"))
        .into());
    }

    let token = header.trim_start_matches(AUTHENTICATION_BEARER).trim();

    let token = match DBUuid::parse_str(token) {
        Ok(v) => v,
        Err(_) => {
            return Err(AppError::new_with_status(
                StatusCode::UNAUTHORIZED,
                AUTHORIZATION_INCORRECT_HEADER_FORMAT_ERROR_CODE,
            )
            .message(arcstr::literal!("Incorrect Authentication header value"))
            .into());
        }
    };

    if token.kind() != token_type {
        return Err(AppError::new_with_status(
            StatusCode::UNAUTHORIZED,
            AUTHORIZATION_INCORRECT_HEADER_FORMAT_ERROR_CODE,
        )
        .message(arcstr::literal!("Incorrect Authentication header value"))
        .into());
    }

    let authorization = match AuthenticationCollection::get_by_key_or_reject(&token, None).await {
        Ok(v) => v,
        Err(_) => {
            return Err(AppError::new_with_status(
                StatusCode::UNAUTHORIZED,
                AUTHORIZATION_INVALID_PERMISSIONS_ERROR_CODE,
            )
            .message(arcstr::literal!("Invalid token"))
            .into());
        }
    };

    let claims = AuthClaims::new(authorization.address.unwrap_as_ref().clone());

    Ok(RequestContextWithAuth::new(context, claims))
}
