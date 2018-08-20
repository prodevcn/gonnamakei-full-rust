use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use commons::server::{limit_requests_by_ip, with_body, with_config};

use crate::context::AppContext;
use crate::models::requests::mail_list::MailListSubscribeRequestBody;
use crate::routes::{with_context, RequestContext};

pub fn build_mail_list_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    commons::balanced_or_tree!(subscribe(context, config),)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn subscribe(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Path, params and queries.
    let route = warp::path!("email" / "subscribe");

    // Method and other validations.
    let route = route.and(warp::post());

    // (Optional) Request limits.
    let route = route.and(limit_requests_by_ip(15 * 60 /* 15 min */, 15));

    // Body.
    let route = route.and(with_body::<MailListSubscribeRequestBody>());

    // (Optional) Make the request object to combine all data.

    // Add application context and config.
    let route = route.and(with_context(context)).and(with_config(config));

    // Make the request context.
    let route = route.map(RequestContext::new);

    // (Optional) With context validations.

    // Service.
    route.and_then(crate::services::mail_list::subscribe_service)
}
