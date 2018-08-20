use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use commons::database::types::{DBUuid, DBUuidType};
use commons::server::{validate_specific_token, with_config};

use crate::context::AppContext;
use crate::routes::{with_context, RequestContext};

pub fn build_internal_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    commons::balanced_or_tree!(auth_check(context, config),)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn auth_check(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("internal" / "auth" / DBUuid / "check");

    // Method and other validations.
    let route = route.and(warp::post());

    // Body.

    // (Optional) Request limits.

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.
    let route = validate_specific_token(route, DBUuidType::InternalToken);

    // Service.
    route.and_then(crate::services::internal::auth_check_service)
}
