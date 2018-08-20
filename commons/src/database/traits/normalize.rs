use std::collections::{HashMap, HashSet};

use arcstr::ArcStr;

pub trait DBNormalize {
    /// Normalizes an element and returns whether it has been modified or not and if it can be removed or not.
    fn normalize(&mut self) -> DBNormalizeResult;
}

pub enum DBNormalizeResult {
    NotModified,
    Modified,
    Removed,
}

impl DBNormalizeResult {
    // GETTERS ----------------------------------------------------------------

    pub fn modified(&self) -> bool {
        !matches!(self, DBNormalizeResult::NotModified)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl DBNormalize for bool {
    fn normalize(&mut self) -> DBNormalizeResult {
        if !*self {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for u8 {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self == &0 {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for u16 {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self == &0 {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for i32 {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self == &0 {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for u64 {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self == &0 {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for i64 {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self == &0 {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for f64 {
    fn normalize(&mut self) -> DBNormalizeResult {
        if (*self - 0.0).abs() < f64::EPSILON {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl<T> DBNormalize for Option<T> {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

impl<T> DBNormalize for Vec<T> {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self.is_empty() {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl<T> DBNormalize for HashSet<T> {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self.is_empty() {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl<K, T> DBNormalize for HashMap<K, T> {
    fn normalize(&mut self) -> DBNormalizeResult {
        if self.is_empty() {
            DBNormalizeResult::Removed
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl DBNormalize for ArcStr {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
