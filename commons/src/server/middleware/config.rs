use std::sync::Arc;

use warp::Filter;

use crate::config::InitServiceConfig;

/// Adds the AppConfig to the flow.
pub fn with_config(
    config: &Arc<InitServiceConfig>,
) -> impl Filter<Extract = (Arc<InitServiceConfig>,), Error = std::convert::Infallible> + Clone {
    let config = config.clone();
    warp::any().map(move || config.clone())
}
