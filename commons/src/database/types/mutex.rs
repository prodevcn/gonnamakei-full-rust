use std::borrow::Cow;

use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use crate::database::traits::AQLMapping;
use crate::database::traits::{DBNormalize, DBNormalizeResult};
use crate::database::types::{DBUuid, DateTime};

sub_model!(
    /// This type stores a mutex for a document.
    pub struct DBMutex {
        #[db_name = "N"]
        pub node: ArcStr,
        #[db_name = "F"]
        pub change_flag: DBUuid,
        #[db_name = "E"]
        pub expiration: DateTime,
    }
);
