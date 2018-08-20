use arcstr::ArcStr;
use lazy_static::lazy_static;

use crate::database::types::DBUuid;
use crate::database::types::DBUuidType;

// GENERAL --------------------------------------------------------------------

pub static NODE_ID_ENV_VAR: &str = "NODE_ID";
pub static CONFIG_FILE: &str = "config.toml";

lazy_static! {
    pub static ref NODE_ID: ArcStr = std::env::var(NODE_ID_ENV_VAR)
        .map(|v| v.into())
        .unwrap_or_else(|_| DBUuid::new(DBUuidType::DBKey).as_string().clone());
}

// DATABASE -------------------------------------------------------------------

pub static DATABASE_NAME: &str = "GMIData";
pub static DATABASE_FUNCTIONS_NAMESPACE: &str = "gmifn";
pub static DATABASE_MUTEX_INDEX: &str = "MutexIndex";
pub static DATABASE_TTL_INDEX: &str = "TTLIndex";
pub static BET_PARTICIPANT_INDEX: &str = "ParticipantAddress";
pub static BET_CHALLENGE_INDEX: &str = "ChallengeAddress";
pub static EMAIL_INDEX: &str = "Email";
pub static AUTHORIZATION_ADDRESS_INDEX: &str = "AuthorizationAddress";
pub static MAX_AQL_RETRIES: usize = 100;

// 1 minute in seconds
pub static REMOTE_MUTEX_TIMEOUT: u64 = 60;
pub const MAX_INSERT_RETRIES: usize = 5;

// MUTEX ----------------------------------------------------------------------
// From 50ms to 150ms
pub static REMOTE_MUTEX_ACQUIRE_MIN_INTERVAL: u64 = 100;
pub static REMOTE_MUTEX_ACQUIRE_MAX_INTERVAL: u64 = 150;
// 1:20 min in seconds
#[cfg(not(feature = "test"))]
pub static REMOTE_MUTEX_ALIVE_INTERVAL: u64 = 80;
#[cfg(feature = "test")]
pub static REMOTE_MUTEX_ALIVE_INTERVAL: u64 = 3;
// 2 min in seconds
pub static REMOTE_MUTEX_EXPIRATION: u64 = 2 * 60;

// CHALLENGES -----------------------------------------------------------------
pub static MIN_CHALLENGE_RESPONSES_PER_PAGE: u64 = 10;
pub static MAX_CHALLENGE_RESPONSES_PER_PAGE: u64 = 50;
pub static MAX_REWARD_MULTIPLIER: u64 = 1_000_000;

// SIGNATURES -----------------------------------------------------------------
// 5 minutes in seconds
pub const SIGNATURE_EXPIRATION_DELAY: u64 = 300;

// AUTHORIZATIONS -------------------------------------------------------------
// 1 hour in seconds
pub const AUTHORIZATION_EXPIRATION_DELAY: u64 = 60 * 60;
