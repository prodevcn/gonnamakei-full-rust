use std::sync::Arc;

use warp::Filter;

use clash_royale::build_clash_royale_game_data_routes;
use commons::config::InitServiceConfig;

use crate::context::AppContext;

mod clash_royale;

pub fn build_games_data_routes(
    context: &Arc<AppContext>,
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("games").and(commons::balanced_or_tree!(
        build_clash_royale_game_data_routes(context, config),
    ))
}
