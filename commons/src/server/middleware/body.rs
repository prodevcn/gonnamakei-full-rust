use serde::de::DeserializeOwned;
use warp::multipart::FormOptions;
use warp::Filter;

/// Reads a body ensuring the content is at most 50Kb.
pub fn with_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(50 * 1024).and(warp::body::json::<T>())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Reads a body ensuring the content is at most `limit`.
pub fn with_body_and_length<T: DeserializeOwned + Send>(
    limit: u64,
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(limit).and(warp::body::json::<T>())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Reads a form ensuring the content is at most `limit`.
pub fn with_form_and_length(limit: u64) -> FormOptions {
    warp::multipart::form().max_length(limit)
}
