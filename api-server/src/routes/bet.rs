use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use commons::database::types::Address;
use commons::server::requests::RequestWithParam;
use commons::server::with_body;
use commons::server::{limit_requests_by_ip, validate_api_token, with_config};

use crate::context::AppContext;
use crate::models::requests::bet::BetSendRequestBody;
use crate::routes::{with_context, RequestContext};

pub fn build_bet_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    commons::balanced_or_tree!(check(context, config), send(context, config),)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn check(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("bet" / Address / "check");

    // Method and other validations.
    let route = route.and(warp::post());

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
    route.and_then(crate::services::bet::check_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn send(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("bet" / Address / "send");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.
    let route = route.and(with_body::<BetSendRequestBody>());
    let route = route.map(RequestWithParam::new);

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
    route.and_then(crate::services::bet::send_service)
}
