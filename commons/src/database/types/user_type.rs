use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

use crate::database::traits::{DBNormalize, DBNormalizeResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserType {
    Anonymous = 0,
    Admin,
}

impl DBNormalize for UserType {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
