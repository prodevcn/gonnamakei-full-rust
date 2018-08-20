use std::sync::Arc;

use warp::http::{HeaderMap, HeaderValue};
use warp::Filter;

use commons::config::InitServiceConfig;

use crate::context::AppContext;
use crate::routes::bet::build_bet_routes;
use crate::routes::challenge::build_challenge_routes;
use crate::routes::data::build_data_routes;
use crate::routes::internal::build_internal_routes;
use crate::routes::mail_list::build_mail_list_routes;
use crate::routes::participant::build_participant_routes;
use crate::routes::rejections::handle_rejection;
use crate::routes::signature::build_signature_routes;

mod bet;
mod challenge;
mod data;
mod internal;
mod mail_list;
mod participant;
mod rejections;
mod signature;

#[allow(dead_code)]
pub type RequestContext<R> = commons::server::RequestContext<Arc<AppContext>, R>;
pub type RequestContextWithAuth<R> = commons::server::RequestContextWithAuth<Arc<AppContext>, R>;

pub fn build_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Routes
    let routes = commons::balanced_or_tree!(
        build_challenge_routes(context, config),
        build_bet_routes(context, config),
        build_data_routes(context, config),
        build_signature_routes(context, config),
        build_participant_routes(context, config),
        build_mail_list_routes(context, config),
        build_internal_routes(context, config),
    );

    // Rejections
    let routes = routes.recover(handle_rejection);

    // Cors
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_header("authorization")
        .allow_methods(vec!["GET", "POST"]);
    let routes = routes.with(cors);

    // Compression
    let routes = routes.with(warp::compression::gzip());

    // Headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "Server",
        HeaderValue::from_str(format!("{}/{}", config.app_name, config.version).as_str()).unwrap(),
    );

    routes.with(warp::reply::with::headers(headers))
}

/// Adds the context of the application to the flow.
pub fn with_context(
    context: &Arc<AppContext>,
) -> impl Filter<Extract = (Arc<AppContext>,), Error = std::convert::Infallible> + Clone {
    let context = context.clone();
    warp::any().map(move || context.clone())
}
