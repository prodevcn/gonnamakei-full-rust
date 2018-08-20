use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use commons::server::{limit_requests_by_ip, with_config};

use crate::context::AppContext;
use crate::routes::{with_context, RequestContext};

pub fn build_clash_royale_game_data_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    commons::balanced_or_tree!(cards(context, config), arenas(context, config),)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn cards(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("clash_royale" / "cards");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 8));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new_empty);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::data::games::clash_royale::cards_service)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn arenas(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("clash_royale" / "arenas");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 8));

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new_empty);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::data::games::clash_royale::arenas_service)
}
