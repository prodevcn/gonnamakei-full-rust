use std::collections::HashMap;
use std::sync::Arc;

use arangors::connection::Connection;
use arangors::{ClientError, GenericConnection};
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use uclient::reqwest::ReqwestClient;

pub use api_document::*;
pub use aql::*;
pub use collection::*;
pub use document::*;
pub use document_edge::*;
pub use nullable_option::*;
pub use reference::*;
pub use reference_api::*;

use crate::config::InitServiceConfig;
use crate::constants::{DATABASE_FUNCTIONS_NAMESPACE, DATABASE_NAME};
use crate::database::collections::CollectionKind;
use crate::error::{AppError, AppResult, INTERNAL_DB_ERROR_CODE};

pub type Database = arangors::Database<ReqwestClient>;
pub type Collection = arangors::Collection<ReqwestClient>;

mod api_document;
mod aql;
mod collection;
pub mod collection_edge;
pub mod collections;
mod document;
mod document_edge;
pub mod documents;
mod nullable_option;
mod reference;
mod reference_api;
pub mod traits;
pub mod types;

/// The database information.
#[derive(Debug)]
pub struct DBInfo {
    config: Arc<InitServiceConfig>,
    connection: GenericConnection<ReqwestClient>,
    pub database: Database,
}

impl DBInfo {
    // METHODS ----------------------------------------------------------------

    pub async fn send_aql_with_retries<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        bind_vars: HashMap<&str, serde_json::Value>,
    ) -> AppResult<Vec<T>> {
        loop {
            match self.database.aql_bind_vars(query, bind_vars.clone()).await {
                Ok(v) => return Ok(v),
                Err(e) => DBInfo::check_client_error_to_retry(e)?,
            }
        }
    }

    pub async fn add_aql_function(
        &self,
        name: &str,
        code: &str,
        is_deterministic: bool,
    ) -> AppResult<()> {
        let name = format!("{}::{}", DATABASE_FUNCTIONS_NAMESPACE, name);
        let client = self.connection.session();
        let database_config = self.config.database.as_ref().unwrap();
        let response = client
            .client
            .post(format!("{}_api/aqlfunction", self.database.url().as_str()))
            .basic_auth(
                database_config.username.as_str(),
                Some(database_config.password.as_str()),
            )
            .json(&AddFunctionRequest {
                name: name.as_str(),
                code,
                is_deterministic,
            })
            .send()
            .await?;

        match response.status().as_u16() {
            200 | 201 => Ok(()),
            _ => {
                let text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "<undefined>".to_string());
                remote_error!("Error trying to create an aqlfunction: {}", text);
                Err(AppError::new_with_status(
                    StatusCode::BAD_REQUEST,
                    INTERNAL_DB_ERROR_CODE,
                ))
            }
        }
    }

    pub async fn remove_all_aql_function(&self) -> AppResult<()> {
        let client = self.connection.session();
        let database_config = self.config.database.as_ref().unwrap();
        let response = client
            .client
            .delete(format!(
                "{}_api/aqlfunction/{}?group=true",
                self.database.url().as_str(),
                DATABASE_FUNCTIONS_NAMESPACE,
            ))
            .basic_auth(
                database_config.username.as_str(),
                Some(database_config.password.as_str()),
            )
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(()),
            _ => {
                let text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "<undefined>".to_string());
                remote_error!("Error trying to delete all aqlfunctions: {}", text);
                Err(AppError::new_with_status(
                    StatusCode::BAD_REQUEST,
                    INTERNAL_DB_ERROR_CODE,
                ))
            }
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub fn check_client_error_to_retry(error: ClientError) -> AppResult<()> {
        match &error {
            ClientError::Arango(e) => match e.error_num() {
                1200 => {
                    warn!("Conflict write-write");
                    Ok(())
                }
                1210 => {
                    warn!("Unique constraint violated");
                    Err(error.into())
                }
                _ => Err(error.into()),
            },
            _ => Err(error.into()),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub async fn init_db_connection(config: &Arc<InitServiceConfig>) -> AppResult<Arc<DBInfo>> {
    let database_config = config.database.as_ref().unwrap();
    let connection = Connection::establish_jwt(
        database_config.url.as_str(),
        database_config.username.as_str(),
        database_config.password.as_str(),
    )
    .await?;

    let database = match connection.create_database(DATABASE_NAME).await {
        Ok(v) => v,
        Err(_) => connection.db(DATABASE_NAME).await?,
    };

    let db_info = Arc::new(DBInfo {
        config: config.clone(),
        connection,
        database,
    });

    for kind in CollectionKind::enum_list() {
        kind.init(&db_info);
    }

    Ok(db_info)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub async fn init_db(db_info: &Arc<DBInfo>) -> AppResult<()> {
    // First execute the configuration.
    let _ = CollectionKind::Configuration
        .create_collection(db_info)
        .await;

    // Then the rest.
    let jobs: Vec<_> = CollectionKind::enum_list()
        .iter()
        .filter(|kind| **kind != CollectionKind::Configuration)
        .map(|kind| {
            let db_info = db_info.clone();
            tokio::spawn(async move { kind.create_collection(&db_info).await })
        })
        .collect();

    for job in jobs {
        job.await.unwrap()?;
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddFunctionRequest<'a> {
    name: &'a str,
    code: &'a str,
    is_deterministic: bool,
}
