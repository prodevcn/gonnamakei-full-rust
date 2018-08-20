use std::convert::TryInto;
use std::fmt;

use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::database::collections::CollectionKind;
use crate::database::traits::{DBNormalize, DBNormalizeResult};
use crate::database::types::DBUuid;

/// The id of a collection.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DBId {
    key: DBUuid,
    collection: CollectionKind,
}

impl DBId {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(key: DBUuid, collection: CollectionKind) -> DBId {
        DBId { key, collection }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn key(&self) -> &DBUuid {
        &self.key
    }

    pub fn collection(&self) -> CollectionKind {
        self.collection
    }
}

impl Serialize for DBId {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}/{}", self.collection.name(), self.key).as_str())
    }
}

impl<'de> Deserialize<'de> for DBId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DBIdVisitor;
        impl<'de> Visitor<'de> for DBIdVisitor {
            type Value = DBId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with the format: <CollectionName>/<DocumentId>")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut value = v.split('/');
                let collection: CollectionKind = match value.next() {
                    Some(v) => match v.try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(E::custom(format!("Incorrect value for a DBId: {}", v)));
                        }
                    },
                    None => return Err(E::custom(format!("Incorrect value for a DBId: {}", v))),
                };

                let key = match value.next() {
                    Some(v) => match DBUuid::parse_str(v) {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(E::custom(format!("Incorrect value for a DBId: {}", v)));
                        }
                    },
                    None => return Err(E::custom(format!("Incorrect value for a DBId: {}", v))),
                };

                // Too much values.
                if value.next().is_some() {
                    return Err(E::custom(format!("Incorrect value for a DBId: {}", v)));
                }

                Ok(DBId { key, collection })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(v.as_str())
            }
        }

        deserializer.deserialize_string(DBIdVisitor)
    }
}

impl DBNormalize for DBId {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
