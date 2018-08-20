use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use commons::server::validate_api_token;
use commons::server::{limit_requests_by_ip, with_body, with_config};

use crate::context::AppContext;
use crate::models::requests::participant::{
    ParticipantGetRequestBody, ParticipantLoginRequestBody, ParticipantUpdateRequestBody,
};
use crate::routes::{with_context, RequestContext};

pub fn build_participant_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    commons::balanced_or_tree!(
        login(context, config),
        logout(context, config),
        get(context, config),
        update(context, config),
    )
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn login(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("participant" / "login");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.
    let route = route.and(with_body::<ParticipantLoginRequestBody>());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::participant::login_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn logout(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("participant" / "logout");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new_empty);

    // (Optional) With context validations.
    let route = validate_api_token(route);

    // Service.
    route.and_then(crate::services::participant::logout_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn get(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("participant" / "get");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.
    let route = route.and(with_body::<ParticipantGetRequestBody>());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.
    let route = validate_api_token(route);

    // Service.
    route.and_then(crate::services::participant::get_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn update(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("participant" / "update");

    // Method and other validations.
    let route = route.and(warp::post());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // Body.
    let route = route.and(with_body::<ParticipantUpdateRequestBody>());

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.
    let route = validate_api_token(route);

    // Service.
    route.and_then(crate::services::participant::update_service)
}
