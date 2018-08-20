use std::convert::TryFrom;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::database::collections::CollectionKind;
use crate::database::traits::{AQLMapping, DBNormalize, DBNormalizeResult};
use crate::database::types::{DBId, DBUuid};
use crate::database::{
    get_aql_inline_variable, APIDocument, APIReference, AqlBuilder, AqlLet, AqlLetKind,
    DBCollection, DBDocument,
};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
#[serde(bound = "T: DBDocument")]
#[serde(untagged)]
pub enum DBReference<T: DBDocument> {
    // Keep this order because otherwise Key will always be dereferenced in favour of Document
    // ignoring the rest of the fields.
    Document(Box<T>),
    Key(DBReferenceKey),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DBReferenceKey {
    #[serde(rename = "_key")]
    key: DBUuid,
}

impl<T: DBDocument> DBReference<T> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_key(key: DBUuid) -> DBReference<T> {
        Self::Key(DBReferenceKey { key })
    }

    // GETTERS ----------------------------------------------------------------

    pub fn id(&self) -> DBId {
        DBId::new(
            self.key(),
            CollectionKind::try_from(T::Collection::name()).unwrap(),
        )
    }

    pub fn key(&self) -> DBUuid {
        match self {
            DBReference::Key(v) => v.key.clone(),
            DBReference::Document(v) => v.db_key().clone().expect("Missing db_key in reference"),
        }
    }

    pub fn is_key(&self) -> bool {
        matches!(self, DBReference::Key(_))
    }

    pub fn is_document(&self) -> bool {
        matches!(self, DBReference::Document(_))
    }

    pub fn unwrap_document_as_ref(&self) -> &T {
        match self {
            DBReference::Document(v) => v,
            _ => unreachable!("Cannot call 'unwrap_document_as_ref' without a document"),
        }
    }

    pub fn unwrap_document_as_mut_ref(&mut self) -> &mut T {
        match self {
            DBReference::Document(v) => v,
            _ => unreachable!("Cannot call 'unwrap_document_as_mut_ref' without a document"),
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn unwrap_document(self) -> Box<T> {
        match self {
            DBReference::Document(v) => v,
            _ => unreachable!("Cannot call 'unwrap_document' without a document"),
        }
    }

    pub fn map_to_api<F, R>(self, mapper: F) -> APIReference<R>
    where
        F: FnOnce(Box<T>) -> Box<R>,
        R: APIDocument,
    {
        match self {
            DBReference::Document(v) => APIReference::Document(mapper(v)),
            DBReference::Key(v) => APIReference::new_key(v.key),
        }
    }
}

impl<T: DBDocument> DBNormalize for DBReference<T> {
    // METHODS ----------------------------------------------------------------

    fn normalize(&mut self) -> DBNormalizeResult {
        match self {
            DBReference::Key(_) => DBNormalizeResult::NotModified,
            DBReference::Document(document) => match document.normalize() {
                DBNormalizeResult::NotModified => DBNormalizeResult::NotModified,
                DBNormalizeResult::Modified => DBNormalizeResult::Modified,
                DBNormalizeResult::Removed => DBNormalizeResult::Modified,
            },
        }
    }
}

impl<T: DBDocument> PartialEq for DBReference<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DBReference::Key(a) => match other {
                DBReference::Key(b) => a == b,
                DBReference::Document(_) => false,
            },
            DBReference::Document(a) => match other {
                DBReference::Key(_) => false,
                DBReference::Document(b) => a.db_key() == b.db_key(),
            },
        }
    }
}

impl<T: DBDocument> AQLMapping for DBReference<T> {
    fn include_let_steps(&self, aql: &mut AqlBuilder, _path: &str, next_id: &mut usize) {
        if let DBReference::Document(document) = self {
            let document_key = document.db_key();

            let collection_name = T::Collection::name();

            let var_name = get_aql_inline_variable(*next_id);
            *next_id += 1;

            aql.let_step(AqlLet {
                variable: var_name,
                expression: AqlLetKind::Expression(
                    format!(
                        "DOCUMENT(\"{}\",{})",
                        collection_name,
                        serde_json::to_string(&document_key).unwrap()
                    )
                    .into(),
                ),
            });

            document.include_let_steps(aql, var_name, next_id);
        }
    }

    fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize) {
        if let DBReference::Document(document) = self {
            let var_name = get_aql_inline_variable(*next_id);
            *next_id += 1;

            document.map_to_json(buffer, var_name, next_id);
        } else {
            buffer.write_all(path.as_bytes()).unwrap();
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::database::documents::ChallengeDBDocument;
    use crate::database::types::DBUuidType;

    use super::*;

    #[test]
    fn test_serialization() {
        let key = DBUuid::new(DBUuidType::DBKey);
        let key_ref = DBReference::<ChallengeDBDocument>::new_key(key.clone());
        let doc_ref = DBReference::Document(Box::new(ChallengeDBDocument {
            db_key: Some(key),
            ..Default::default()
        }));

        let key_ref_serialization = serde_json::to_string(&key_ref).unwrap();
        let doc_ref_serialization = serde_json::to_string(&doc_ref).unwrap();

        assert_eq!(key_ref_serialization, doc_ref_serialization);
    }
}
