use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use tokio::sync::{oneshot, Mutex};

use commons::config::{read_app_config, InitServiceConfig};
use commons::constants::CONFIG_FILE;
use commons::test::commons::{
    acquire_test_reset_for_parallel, reset_db, set_test_reset_for_serial,
};
use commons::utils::try_init_logger;

use crate::context::AppContext;
use crate::error::ServerResult;
use crate::setup_context;

// Keep the number to not collide with other module's tests.
static TEST_UID: AtomicUsize = AtomicUsize::new(1_000_000);

pub fn next_test_uid() -> usize {
    TEST_UID.fetch_add(1, Ordering::SeqCst)
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Arc<InitServiceConfig>>> = Mutex::new(None);
    static ref SERIAL_LOCK: RwLock<()> = RwLock::new(());
    static ref RUNTIME: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
}

async fn setup(reset_config: bool) -> ServerResult<(Arc<AppContext>, Arc<InitServiceConfig>)> {
    let mut config_lock = CONFIG.lock().await;
    let result = match config_lock.deref() {
        Some(config) => {
            let (shutdown_tx, _) = oneshot::channel();
            let context = setup_context(config, shutdown_tx).await?;

            if reset_config {
                reset(&context, config).await?;
            }

            (context, config.clone())
        }
        None => {
            try_init_logger();

            // Config
            let dir = std::env::current_dir().unwrap();
            let dir = dir.to_str().unwrap();
            let path = format!("{}/../deployment/{}", dir, CONFIG_FILE);
            std::env::set_var("CONFIG_PATH", &path);

            let config = Arc::new(read_app_config()?);

            let (shutdown_tx, _) = oneshot::channel();
            let context = setup_context(&config, shutdown_tx).await?;
            *config_lock = Some(config.clone());

            reset(&context, &config).await?;

            (context, config)
        }
    };

    Ok(result)
}

async fn reset(context: &Arc<AppContext>, _config: &Arc<InitServiceConfig>) -> ServerResult<()> {
    reset_db(&context.db_info)
        .await
        .expect("Error resetting the DB");

    Ok(())
}

#[allow(dead_code)]
pub fn run_db_test_serial<T, F>(test: T)
where
    T: FnOnce(Arc<AppContext>, Arc<InitServiceConfig>, &'static (dyn Fn() -> usize + Sync)) -> F,
    F: std::future::Future<Output = ()> + Send,
{
    let _guard = RUNTIME.enter();
    futures::executor::block_on(async move {
        // Lock SERIAL_LOCK to execute these test in series.
        let _serial_lock = SERIAL_LOCK.write().await;

        let (context, config) = setup(true).await.expect("Error during setup");
        test(context.clone(), config.clone(), &next_test_uid).await;
        set_test_reset_for_serial().await;
    });
}

#[allow(dead_code)]
pub fn run_db_test_parallel<T, F>(test: T)
where
    T: FnOnce(Arc<AppContext>, Arc<InitServiceConfig>, &'static (dyn Fn() -> usize + Sync)) -> F,
    F: std::future::Future<Output = ()> + Send,
{
    let _guard = RUNTIME.enter();
    futures::executor::block_on(async move {
        // Lock SERIAL_LOCK to execute these test in parallel with others.
        let _serial_lock = SERIAL_LOCK.read().await;

        let (context, config) = setup(false).await.expect("Error during setup");

        {
            let mut lock = acquire_test_reset_for_parallel().await;
            if *lock {
                *lock = false;

                reset(&context, &config).await.expect("Error during reset");
            }
        }

        test(context, config, &next_test_uid).await;
    });
}
