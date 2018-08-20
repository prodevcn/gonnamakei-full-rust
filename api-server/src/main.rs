#[macro_use]
extern crate commons;
#[macro_use]
extern crate log;

use std::sync::Arc;

use tokio::sync::oneshot;

use commons::config::{InitServiceConfig, read_app_config};
use commons::constants::NODE_ID;
use commons::data::release_all_mutex_of_current_microservice;
use commons::database::init_db_connection;
use commons::utils::init_logger;

use crate::context::AppContext;
use crate::error::ServerResult;
use crate::routes::build_routes;

mod constants;
mod context;
mod error;
mod models;
mod routes;
mod services;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    init_logger();

    if let Err(e) = application().await {
        error!("{}", e);
        std::process::exit(1)
    }
}

async fn application() -> ServerResult<()> {
    let config = Arc::new(read_app_config()?);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let context = setup_context(&config, shutdown_tx).await?;

    // Routes
    let routes = build_routes(&context, &config);

    // Free locks.
    release_all_mutex_of_current_microservice().await;

    // Init server.
    let db_config = config.db_config().await;
    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(
        ([0, 0, 0, 0], db_config.api.port),
        async {
            shutdown_rx.await.ok();
            info!("Stop signal received");
        },
    );

    server.await;

    remote_info!("Auth server '{}' stopped", NODE_ID.as_str());

    Ok(())
}

pub async fn setup_context(
    config: &Arc<InitServiceConfig>,
    shutdown_signal: oneshot::Sender<()>,
) -> ServerResult<Arc<AppContext>> {
    // Database
    let db_info = init_db_connection(config).await?;

    // Context
    let context = Arc::new(AppContext::new(config, db_info, shutdown_signal));

    Ok(context)
}
