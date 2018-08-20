use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;

use rand::Rng;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

use crate::constants::NODE_ID;
use crate::constants::{
    REMOTE_MUTEX_ACQUIRE_MAX_INTERVAL, REMOTE_MUTEX_ACQUIRE_MIN_INTERVAL,
    REMOTE_MUTEX_ALIVE_INTERVAL, REMOTE_MUTEX_EXPIRATION,
};
use crate::data::SynchronizedDBDocument;
use crate::database::collections::MutexCollection;
use crate::database::documents::{DBDocumentField, MutexDBDocument};
use crate::database::types::{DBMutexField, DBUuid, DBUuidType, DateTime};
use crate::database::{
    AqlBuilder, AqlLet, AqlLetKind, AqlLimit, AqlReturn, AqlSort, AqlUpdate, DBCollection,
    AQL_DOCUMENT_ID, AQL_NEW_ID,
};
use crate::error::AppResult;

pub struct RemoteMutexGuard<T: SynchronizedDBDocument + 'static> {
    pub(in crate::data::remote_mutex) inner: Arc<Mutex<RemoteMutexGuardInner<T>>>,
}

pub(in crate::data::remote_mutex) struct RemoteMutexGuardInner<T: SynchronizedDBDocument + 'static>
{
    pub(in crate::data::remote_mutex) elements: HashSet<DBUuid>,
    pub(in crate::data::remote_mutex) change_flag: DBUuid,
    pub(in crate::data::remote_mutex) alive_job: Option<JoinHandle<()>>,
    pub(in crate::data::remote_mutex) _type: PhantomData<T>,
}

impl<T: SynchronizedDBDocument + 'static> RemoteMutexGuard<T> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub async fn acquire_with_timeout(
        id: &DBUuid,
        timeout: u64,
        fields: Option<&T>,
    ) -> AppResult<Option<(T, RemoteMutexGuard<T>)>> {
        let time_out = DateTime::now().after_seconds(timeout);
        let mut checked_doc_exists = false;

        loop {
            if time_out.is_expired() {
                return Ok(None);
            }

            match Self::try_acquire(id, fields).await? {
                Some(v) => return Ok(Some(v)),
                None => {
                    if !checked_doc_exists {
                        // Check the document exists and exit if not.
                        // This prevents waiting until timeout when the document
                        // is not present in the DB.
                        let collection = T::collection();

                        // Prepare AQL.
                        // LET i = DOCUMENT(<file_collection>, <key>)
                        // FILTER i != null
                        // RETURN true
                        let mut aql = AqlBuilder::new_simple();

                        aql.let_step(AqlLet {
                            variable: AQL_DOCUMENT_ID,
                            expression: AqlLetKind::Expression(
                                format!(
                                    "DOCUMENT(\"{}\",{})",
                                    T::Collection::name(),
                                    serde_json::to_string(id).unwrap()
                                )
                                .into(),
                            ),
                        });

                        aql.filter_step(format!("{} != null", AQL_DOCUMENT_ID).into());
                        aql.return_step(AqlReturn::new_expression("true".into()));

                        // Send the AQL.
                        let result = collection.send_generic_aql::<bool>(&aql).await?;
                        let result = result.results;
                        if result.is_empty() {
                            return Ok(None);
                        }

                        checked_doc_exists = true;
                    }

                    // Sleep for a while to retry later.
                    let time = {
                        let mut rng = rand::thread_rng();
                        rng.gen_range(
                            REMOTE_MUTEX_ACQUIRE_MIN_INTERVAL..REMOTE_MUTEX_ACQUIRE_MAX_INTERVAL,
                        )
                    };
                    sleep(Duration::from_millis(time)).await;
                }
            }
        }
    }

    pub async fn acquire_collection_with_timeout(
        timeout: u64,
    ) -> AppResult<Option<RemoteMutexGuard<MutexDBDocument>>> {
        let mutex_collection = MutexCollection::instance();
        let id = T::collection_key(&mutex_collection).clone();

        Ok(
            RemoteMutexGuard::<MutexDBDocument>::acquire_with_timeout(&id, timeout, None)
                .await?
                .map(|(_, mutex)| mutex),
        )
    }

    #[cfg(feature = "test")]
    pub fn acquire_for_test(id: &DBUuid) -> RemoteMutexGuard<T> {
        RemoteMutexGuard {
            inner: Arc::new(Mutex::new(RemoteMutexGuardInner {
                elements: {
                    let mut set = HashSet::with_capacity(1);
                    set.insert(id.clone());

                    set
                },
                change_flag: DBUuid::new_with_code_for_test(0, DBUuidType::DBKey),
                alive_job: Some(tokio::spawn(async {})),
                _type: PhantomData::default(),
            })),
        }
    }

    pub async fn try_acquire(
        id: &DBUuid,
        fields: Option<&T>,
    ) -> AppResult<Option<(T, RemoteMutexGuard<T>)>> {
        let ids = [id.clone()];
        let (mut docs, mutex) = Self::try_acquire_list(&ids, fields).await?;

        if !mutex.is_empty().await {
            Ok(Some((docs.remove(0), mutex)))
        } else {
            Ok(None)
        }
    }

    pub async fn try_acquire_collection() -> AppResult<Option<RemoteMutexGuard<MutexDBDocument>>> {
        let mutex_collection = MutexCollection::instance();
        let id = T::collection_key(&mutex_collection).clone();

        Ok(RemoteMutexGuard::<MutexDBDocument>::try_acquire(&id, None)
            .await?
            .map(|(_, mutex)| mutex))
    }

    pub async fn try_acquire_list(
        ids: &[DBUuid],
        fields: Option<&T>,
    ) -> AppResult<(Vec<T>, RemoteMutexGuard<T>)> {
        if ids.is_empty() {
            return Ok((
                Vec::new(),
                RemoteMutexGuard {
                    inner: Arc::new(Mutex::new(RemoteMutexGuardInner {
                        elements: HashSet::new(),
                        change_flag: DBUuid::new(DBUuidType::DBKey),
                        alive_job: Some(tokio::spawn(async {})),
                        _type: PhantomData::default(),
                    })),
                },
            ));
        }

        let collection = T::collection();
        let collection_name = T::Collection::name();
        let mutex_path = DBDocumentField::Mutex.path();

        let node_id = NODE_ID.clone();
        let expiration = DateTime::now().after_seconds(REMOTE_MUTEX_EXPIRATION);

        // FOR i IN <ids>
        //     LET o = Document(<collection>, i)
        //     FILTER o != null && o.<mutex.expiration> <= <now>
        //     UPDATE i WITH { <mutex>: { <node>: <node_id>, <expiration>: <expiration>, <change_flag>: <change_flag> } } IN <collection> OPTIONS { mergeObjects: true, ignoreErrors: true }
        //     FILTER NEW != null
        //     RETURN NEW
        let document_key = "o";
        let change_flag = DBUuid::new(DBUuidType::DBKey);
        let mut aql = AqlBuilder::new_for_in_list(AQL_DOCUMENT_ID, ids);
        aql.let_step(AqlLet {
            variable: document_key,
            expression: AqlLetKind::Expression(
                format!("DOCUMENT({}, {})", collection_name, AQL_DOCUMENT_ID).into(),
            ),
        });
        aql.filter_step(
            format!(
                "{} != null && {}.{}.{} <= {}",
                document_key,
                document_key,
                mutex_path,
                DBMutexField::Expiration(None).path(),
                serde_json::to_string(&DateTime::now()).unwrap()
            )
            .into(),
        );
        aql.update_step(
            AqlUpdate::new(
                AQL_DOCUMENT_ID.into(),
                collection_name,
                format!(
                    "{{ {}: {{ {}: {}, {}: {}, {}: {} }} }}",
                    mutex_path,
                    DBMutexField::Node(None).path(),
                    serde_json::to_string(&node_id).unwrap(),
                    DBMutexField::Expiration(None).path(),
                    serde_json::to_string(&expiration).unwrap(),
                    DBMutexField::ChangeFlag(None).path(),
                    serde_json::to_string(&change_flag).unwrap()
                )
                .into(),
            )
            .apply_ignore_errors(true),
        );
        aql.filter_step(format!("{} != null", AQL_NEW_ID).into());

        if let Some(fields) = fields {
            aql.return_step_with_fields(AQL_NEW_ID, fields);
        } else {
            aql.return_step(AqlReturn::new_updated());
        }

        let result = collection.send_generic_aql::<T>(&aql).await?;
        let result_ids: HashSet<DBUuid> = result
            .results
            .iter()
            .map(|v| v.db_key().as_ref().unwrap().clone())
            .collect();

        let guard = RemoteMutexGuard {
            inner: Arc::new(Mutex::new(RemoteMutexGuardInner {
                elements: result_ids,
                change_flag,
                alive_job: None,
                _type: PhantomData::default(),
            })),
        };

        // Launch alive action.
        {
            let mut lock = guard.inner.lock().await;
            lock.alive_job = Some(tokio::spawn(RemoteMutexGuard::alive_action(
                guard.inner.clone(),
            )));
        }

        Ok((result.results, guard))
    }

    pub async fn try_acquire_aql(
        filter: Option<&str>,
        sort: Option<Vec<AqlSort<'_>>>,
        limits: Option<AqlLimit>,
        fields: Option<&T>,
    ) -> AppResult<(Vec<T>, RemoteMutexGuard<T>)> {
        let collection = T::collection();
        let collection_name = T::Collection::name();
        let mutex_path = DBDocumentField::Mutex.path();

        let node_id = NODE_ID.clone();
        let expiration = DateTime::now().after_seconds(REMOTE_MUTEX_EXPIRATION);

        // FOR i IN <collection>
        //     <custom_filter>
        //     FILTER i.<mutex.expiration> <= <now>
        //     <custom_sort>
        //     <custom_limit>
        //     UPDATE i WITH { <mutex>: { <node>: <node_id>, <expiration>: <expiration>, <change_flag>: <change_flag> } } IN <collection> OPTIONS { mergeObjects: true, ignoreErrors: true }
        //     FILTER NEW != null
        //     RETURN NEW
        let change_flag = DBUuid::new(DBUuidType::DBKey);
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, collection_name);

        if let Some(filter) = filter {
            aql.filter_step(filter.into());
        }
        aql.filter_step(
            format!(
                "{}.{}.{} <= {}",
                AQL_DOCUMENT_ID,
                mutex_path,
                DBMutexField::Expiration(None).path(),
                serde_json::to_string(&DateTime::now()).unwrap()
            )
            .into(),
        );

        if let Some(sort) = sort {
            aql.sort_step(sort);
        }

        if let Some(limits) = limits {
            aql.limit_step(limits);
        }

        aql.update_step(
            AqlUpdate::new_document(
                collection_name,
                format!(
                    "{{ {}: {{ {}: {}, {}: {}, {}: {} }} }}",
                    mutex_path,
                    DBMutexField::Node(None).path(),
                    serde_json::to_string(&node_id).unwrap(),
                    DBMutexField::Expiration(None).path(),
                    serde_json::to_string(&expiration).unwrap(),
                    DBMutexField::ChangeFlag(None).path(),
                    serde_json::to_string(&change_flag).unwrap()
                )
                .into(),
            )
            .apply_ignore_errors(true),
        );
        aql.filter_step(format!("{} != null", AQL_NEW_ID).into());

        if let Some(fields) = fields {
            aql.return_step_with_fields(AQL_NEW_ID, fields);
        } else {
            aql.return_step(AqlReturn::new_updated());
        }

        let result = collection.send_generic_aql::<T>(&aql).await?;
        let result_ids: HashSet<DBUuid> = result
            .results
            .iter()
            .map(|v| v.db_key().as_ref().unwrap().clone())
            .collect();

        let guard = RemoteMutexGuard {
            inner: Arc::new(Mutex::new(RemoteMutexGuardInner {
                elements: result_ids,
                change_flag,
                alive_job: None,
                _type: PhantomData::default(),
            })),
        };

        // Launch alive action.
        {
            let mut lock = guard.inner.lock().await;
            lock.alive_job = Some(tokio::spawn(RemoteMutexGuard::alive_action(
                guard.inner.clone(),
            )));
        }

        Ok((result.results, guard))
    }

    // GETTERS ----------------------------------------------------------------

    pub async fn is_empty(&self) -> bool {
        let lock = self.inner.lock().await;
        lock.elements.is_empty()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn is_id_locked(&self, id: &DBUuid) -> bool {
        let lock = self.inner.lock().await;
        lock.elements.get(id).is_some()
    }

    /// This method removes the lock for the specified id. It is useful to prevent errors when
    /// locked documents are removed before releasing the lock.
    pub async fn remove_id(&self, id: &DBUuid) {
        let mut lock = self.inner.lock().await;
        lock.elements.remove(id);
    }

    /// Removes an id from the current lock and moves it to another one.
    pub async fn pop(&mut self, id: DBUuid) -> Option<RemoteMutexGuard<T>> {
        let mut lock = self.inner.lock().await;

        if !lock.elements.remove(&id) {
            return None;
        }

        let guard = RemoteMutexGuard {
            inner: Arc::new(Mutex::new(RemoteMutexGuardInner {
                elements: {
                    let mut set = HashSet::with_capacity(1);
                    set.insert(id);
                    set
                },
                change_flag: lock.change_flag.clone(),
                alive_job: None,
                _type: PhantomData::default(),
            })),
        };

        // Launch alive action.
        {
            let mut lock = guard.inner.lock().await;
            lock.alive_job = Some(tokio::spawn(RemoteMutexGuard::alive_action(
                guard.inner.clone(),
            )));
        }

        Some(guard)
    }

    pub fn release(self) {
        tokio::spawn(Self::release_action(self.inner.clone()));
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(in crate::data::remote_mutex) async fn alive_action(
        mutex: Arc<Mutex<RemoteMutexGuardInner<T>>>,
    ) {
        loop {
            // Sleep for interval.
            sleep(Duration::from_secs(REMOTE_MUTEX_ALIVE_INTERVAL)).await;

            let mut lock = mutex.lock().await;
            if lock.alive_job.is_none() {
                // The mutex has been released already.
                return;
            }

            // Avoid doing DB requests.
            if lock.elements.is_empty() {
                continue;
            }

            let collection = T::collection();

            let node_id = NODE_ID.clone();
            let now = DateTime::now();
            let expiration = DateTime::now().after_seconds(REMOTE_MUTEX_EXPIRATION);

            let ids = &lock.elements;

            // FOR i IN <ids>
            //     LET o = Document(<collection>, i)
            //     FILTER o != null && o.<mutex.node> == <node> && o.<mutex.change_flag> == <change_flag> && o.<mutex.expiration> > <now>
            //     UPDATE i WITH { <mutex>: { <expiration>: <expiration> } } IN <collection> OPTIONS { mergeObjects: true, ignoreErrors: true }
            //     FILTER NEW != null
            //     RETURN i
            let document_key = "o";
            let collection_name = T::Collection::name();
            let mutex_path = DBDocumentField::Mutex.path();
            let mut aql = AqlBuilder::new_for_in_set(AQL_DOCUMENT_ID, ids);
            aql.let_step(AqlLet {
                variable: document_key,
                expression: AqlLetKind::Expression(
                    format!("DOCUMENT({}, {})", collection_name, AQL_DOCUMENT_ID).into(),
                ),
            });
            aql.filter_step(
                format!(
                    "{} != null && {}.{}.{} == {} && {}.{}.{} == {} && {}.{}.{} > {}",
                    document_key,
                    document_key,
                    mutex_path,
                    DBMutexField::Node(None).path(),
                    serde_json::to_string(&node_id).unwrap(),
                    document_key,
                    mutex_path,
                    DBMutexField::ChangeFlag(None).path(),
                    serde_json::to_string(&lock.change_flag).unwrap(),
                    document_key,
                    mutex_path,
                    DBMutexField::Expiration(None).path(),
                    serde_json::to_string(&now).unwrap(),
                )
                .into(),
            );
            aql.update_step(
                AqlUpdate::new_document(
                    collection_name,
                    format!(
                        "{{ {}: {{ {}: {} }} }}",
                        mutex_path,
                        DBMutexField::Expiration(None).path(),
                        serde_json::to_string(&expiration).unwrap(),
                    )
                    .into(),
                )
                .apply_ignore_errors(true),
            );
            aql.filter_step(format!("{} != null", AQL_NEW_ID).into());
            aql.return_step(AqlReturn::new_document());

            let result = match collection.send_generic_aql::<DBUuid>(&aql).await {
                Ok(v) => v.results,
                Err(e) => {
                    lock.alive_job.take().unwrap().abort();
                    remote_error!(
                        "Cannot access to database in RemoteMutexGuard::alive_action. Error: {}",
                        e
                    );
                    return;
                }
            };
            let result: HashSet<_> = result.into_iter().collect();

            if result.is_empty() {
                lock.alive_job.take().unwrap().abort();
                return;
            }

            lock.elements = result;
        }
    }

    async fn release_action(mutex: Arc<Mutex<RemoteMutexGuardInner<T>>>) {
        let mut lock = mutex.lock().await;
        if lock.alive_job.is_none() {
            // The mutex has been released already.
            return;
        }

        // Abort the alive job.
        lock.alive_job.take().unwrap().abort();

        // Avoid doing DB requests.
        if lock.elements.is_empty() {
            return;
        }

        let collection = T::collection();

        let node_id = NODE_ID.clone();

        let ids = &lock.elements;

        // FOR i IN <ids>
        //     LET o = Document(<collection>, i)
        //     FILTER o != null && o.<mutex.node> == <node> && o.<mutex.change_flag> == <change_flag>
        //     UPDATE i WITH { <mutex>: null } IN <collection> OPTIONS { mergeObjects: true, keepNulls: false, ignoreErrors: true }
        //     FILTER NEW != null
        //     RETURN i
        let document_key = "o";
        let collection_name = T::Collection::name();
        let mutex_path = DBDocumentField::Mutex.path();
        let mut aql = AqlBuilder::new_for_in_set(AQL_DOCUMENT_ID, ids);
        aql.let_step(AqlLet {
            variable: document_key,
            expression: AqlLetKind::Expression(
                format!("DOCUMENT({}, {})", collection_name, AQL_DOCUMENT_ID).into(),
            ),
        });
        aql.filter_step(
            format!(
                "{} != null && {}.{}.{} == {} && {}.{}.{} == {}",
                document_key,
                document_key,
                mutex_path,
                DBMutexField::Node(None).path(),
                serde_json::to_string(&node_id).unwrap(),
                document_key,
                mutex_path,
                DBMutexField::ChangeFlag(None).path(),
                serde_json::to_string(&lock.change_flag).unwrap(),
            )
            .into(),
        );
        aql.update_step(
            AqlUpdate::new_document(
                collection_name,
                format!("{{ {}: null }}", mutex_path).into(),
            )
            .apply_ignore_errors(true),
        );
        aql.filter_step(format!("{} != null", AQL_NEW_ID).into());
        aql.return_step(AqlReturn::new_document());

        let result = match collection.send_generic_aql::<DBUuid>(&aql).await {
            Ok(v) => v.results,
            Err(e) => {
                lock.alive_job.take().unwrap().abort();
                remote_error!(
                    "Cannot access to database in RemoteMutexGuard::release_action. Error: {}",
                    e
                );
                return;
            }
        };
        let result: HashSet<_> = result.iter().collect();

        for element_id in ids {
            if !result.contains(element_id) {
                remote_fatal!(
                    "The mutex (Collection: {}, Id: {}, ChangeFlag: {}) couldn't be released",
                    collection_name,
                    element_id,
                    lock.change_flag
                );
            }
        }
    }

    pub(in crate) async fn release_all_document_mutex() {
        // Remove user locks.
        // FOR i IN <collection>
        //     FILTER i.<mutex.node> == <node>
        //     UPDATE i WITH { <mutex>: null } IN <collection> OPTIONS { mergeObjects: true, keepNulls: false, ignoreErrors: true }
        let node_id = NODE_ID.clone();
        let mutex_path = DBDocumentField::Mutex.path();
        let collection = T::collection();
        let collection_name = T::Collection::name();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, collection_name);
        aql.filter_step(
            format!(
                "{}.{}.{} == {}",
                AQL_DOCUMENT_ID,
                mutex_path,
                DBMutexField::Node(None).path(),
                serde_json::to_string(&node_id).unwrap(),
            )
            .into(),
        );
        aql.update_step(
            AqlUpdate::new(
                AQL_DOCUMENT_ID.into(),
                collection_name,
                format!("{{ {}: null }}", mutex_path).into(),
            )
            .apply_ignore_errors(true),
        );

        if let Err(e) = collection.send_generic_aql::<DBUuid>(&aql).await {
            remote_error!(
                    "Cannot access to database in RemoteMutexGuard::release_all_document_mutex. Error: {}",
                    e
                );
        }
    }
}

impl<T: SynchronizedDBDocument + 'static> Drop for RemoteMutexGuard<T> {
    fn drop(&mut self) {
        tokio::spawn(Self::release_action(self.inner.clone()));
    }
}
