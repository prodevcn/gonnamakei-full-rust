use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard};

use crate::database::aql_functions::{ARANGODB_WAP_FN_TEST, ARANGODB_WAP_FN_TEST_CODE};
use crate::database::collections::CollectionKind;
use crate::database::{init_db, DBInfo};
use crate::error::AppResult;

lazy_static! {
    static ref TEST_RESET_NEXT: Mutex<bool> = Mutex::new(false);
}

pub async fn set_test_reset_for_serial() {
    let mut lock = TEST_RESET_NEXT.lock().await;
    *lock = true;
}

pub async fn acquire_test_reset_for_parallel() -> MutexGuard<'static, bool> {
    TEST_RESET_NEXT.lock().await
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub async fn reset_db(db_info: &Arc<DBInfo>) -> AppResult<()> {
    // Drop collections.
    let jobs: Vec<_> = CollectionKind::enum_list()
        .iter()
        .map(|kind| tokio::spawn(kind.drop_collection()))
        .collect();

    for job in jobs {
        let _ = job.await;
    }

    // Reset collections.
    init_db(db_info).await?;

    // Remove all custom aql functions.
    db_info.remove_all_aql_function().await?;

    // Add the aql functions.
    db_info
        .add_aql_function(ARANGODB_WAP_FN_TEST, ARANGODB_WAP_FN_TEST_CODE, true)
        .await?;

    Ok(())
}
