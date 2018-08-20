use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use commons::database::types::Address;
use commons::server::requests::RequestWithParam;
use commons::server::{limit_requests_by_ip, validate_api_token, with_body, with_config};

use crate::context::AppContext;
use crate::models::requests::challenge::{
    ChallengeCreateConfigRequestBody, ChallengeCreateRequestBody, ChallengeGetRequestBody,
    ChallengeListRequestBody,
};
use crate::routes::{with_context, RequestContext};

pub fn build_challenge_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    commons::balanced_or_tree!(
        get(context, config),
        info(context, config),
        list(context, config),
        create_config(context, config),
        create(context, config),
        bet(context, config),
    )
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn get(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("challenge" / Address);

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.
    let route = route.and(with_body::<ChallengeGetRequestBody>());
    let route = route.map(RequestWithParam::new);

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::challenge::get_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn info(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("challenge" / Address / "info");

    // Method and other validations.
    let route = route.and(warp::get());

    // Body.

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::challenge::info_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn list(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("challenge" / "list");

    // Method and other validations.
    let route = route.and(warp::post());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // Body.
    let route = route.and(with_body::<ChallengeListRequestBody>());

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::challenge::list_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn create_config(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("challenge" / "create" / "config");

    // Method and other validations.
    let route = route.and(warp::post());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // Body.
    let route = route.and(with_body::<ChallengeCreateConfigRequestBody>());

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.
    let route = validate_api_token(route);

    // Service.
    route.and_then(crate::services::challenge::create_config_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn create(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("challenge" / "create");

    // Method and other validations.
    let route = route.and(warp::post());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // Body.
    let route = route.and(with_body::<ChallengeCreateRequestBody>());

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.
    let route = validate_api_token(route);

    // Service.
    route.and_then(crate::services::challenge::create_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn bet(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("challenge" / Address / "bet");

    // Method and other validations.
    let route = route.and(warp::post());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 75));

    // Body.

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.
    let route = validate_api_token(route);

    // Service.
    route.and_then(crate::services::challenge::bet_service)
}
