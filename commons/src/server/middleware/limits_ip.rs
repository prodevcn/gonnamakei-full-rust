use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;

use crate::database::types::DateTime;
use crate::error::{AppError, AUTHORIZATION_TOO_MANY_REQUESTS_ERROR_CODE};

struct Limits {
    expiration: DateTime,
    map: HashMap<SocketAddr, u16>,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Limits the requests using the origin IP.
pub fn limit_requests_by_ip(
    window_delay: u64,
    max_requests: u16,
) -> impl Filter<Extract = (), Error = warp::Rejection> + Clone + Send + Sync + 'static {
    let map = Arc::new(Mutex::new(Limits {
        expiration: DateTime::now(),
        map: HashMap::new(),
    }));

    warp::any()
        .map(move || window_delay)
        .and(warp::any().map(move || max_requests))
        .and(warp::filters::addr::remote())
        .and(warp::any().map(move || map.clone()))
        .and_then(validate_process)
        .untuple_one()
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

async fn validate_process(
    window_delay: u64,
    max_requests: u16,
    address: Option<SocketAddr>,
    map: Arc<Mutex<Limits>>,
) -> Result<(), warp::Rejection> {
    if cfg!(debug_assertions) {
        return Ok(());
    }

    let address = match address {
        Some(v) => v,
        None => {
            return Err(AppError::new_with_status(
                StatusCode::TOO_MANY_REQUESTS,
                AUTHORIZATION_TOO_MANY_REQUESTS_ERROR_CODE,
            )
            .message(arcstr::literal!("Too many requests"))
            .into());
        }
    };

    let mut lock = map.lock().await;

    if lock.expiration.is_expired() {
        lock.map.clear();
        lock.expiration = DateTime::now().after_seconds(window_delay);
    }

    let times = if let Some(times) = lock.map.get_mut(&address) {
        *times = times.saturating_add(1);
        *times
    } else {
        lock.map.insert(address, 1);
        1
    };

    if times >= max_requests {
        return Err(AppError::new_with_status(
            StatusCode::TOO_MANY_REQUESTS,
            AUTHORIZATION_TOO_MANY_REQUESTS_ERROR_CODE,
        )
        .message(arcstr::literal!("Too many requests"))
        .into());
    }

    Ok(())
}
