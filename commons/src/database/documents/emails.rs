use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::database::collections::CollectionKind;
use crate::database::collections::EmailCollection;
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::{DBId, DBUuid};
use crate::database::{DBDocument, NullableOption};

model!(
    pub struct Email {
        #[db_name = "E"]
        pub email: NullableOption<ArcStr>,
    }
);
