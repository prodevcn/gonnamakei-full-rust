use std::sync::Arc;

use warp::Filter;

use commons::config::InitServiceConfig;
use games::build_games_data_routes;

use crate::context::AppContext;

mod games;

pub fn build_data_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("data").and(commons::balanced_or_tree!(build_games_data_routes(
        context, config
    ),))
}
