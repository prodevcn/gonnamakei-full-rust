use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

pub use authentication::*;
pub use bet::*;
pub use challenge::*;
pub use config::*;
pub use emails::*;
pub use mutex::*;
pub use participant::*;
pub use signature::*;

mod authentication;
mod bet;
mod challenge;
mod config;
mod emails;
mod mutex;
mod participant;
mod signature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DBDocumentField {
    Key,
    Id,
    Rev,
    To,
    From,
    Mutex,
}

impl DBDocumentField {
    // GETTERS ----------------------------------------------------------------

    pub fn path(&self) -> Cow<'static, str> {
        match self {
            DBDocumentField::Key => "_key".into(),
            DBDocumentField::Id => "_id".into(),
            DBDocumentField::Rev => "_rev".into(),
            DBDocumentField::To => "_to".into(),
            DBDocumentField::From => "_from".into(),
            DBDocumentField::Mutex => "_l".into(),
        }
    }
}
