use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};

use commons::clients::games::GameClients;
use commons::clients::solana::SolanaClient;
use commons::config::InitServiceConfig;
use commons::database::DBInfo;

pub struct AppContext {
    pub db_info: Arc<DBInfo>,
    pub http_client: Arc<reqwest::Client>,
    pub solana_client: Arc<SolanaClient>,
    pub game_clients: GameClients,
    inner: Arc<Mutex<Inner>>,
}

pub struct Inner {
    shutdown_signal: oneshot::Sender<()>,
}

impl AppContext {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(
        config: &Arc<InitServiceConfig>,
        db_info: Arc<DBInfo>,
        shutdown_signal: oneshot::Sender<()>,
    ) -> Self {
        let http_client = Arc::new(reqwest::Client::new());
        AppContext {
            db_info,
            solana_client: Arc::new(SolanaClient::new(config)),
            game_clients: GameClients::new(config, &http_client),
            http_client,
            inner: Arc::new(Mutex::new(Inner { shutdown_signal })),
        }
    }

    // METHODS ----------------------------------------------------------------

    pub async fn shutdown(&self) {
        let mut lock = self.inner.lock().await;
        let (shutdown_tx, _) = oneshot::channel();

        let old = std::mem::replace(&mut lock.shutdown_signal, shutdown_tx);
        let _ = old.send(());
    }
}
